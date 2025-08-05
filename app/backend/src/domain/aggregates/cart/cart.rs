use crate::domain::aggregates::CartItem;
use crate::domain::error::DomainError;
use crate::domain::value_objects::*;
use crate::domain::{Coupon, CouponDiscountService};

/// カートアグリゲート
#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    items: Vec<CartItem>,
    shipping_fee: Option<Money>,
    payment_fee: Option<Money>,
    coupon: Option<Coupon>,
}

impl Cart {
    /// 空のカートを作成
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            shipping_fee: None,
            payment_fee: None,
            coupon: None,
        }
    }

    /// カートアイテムのリストからカートを作成
    pub fn from_items(items: Vec<CartItem>) -> Self {
        Self {
            items,
            shipping_fee: None,
            payment_fee: None,
            coupon: None,
        }
    }

    /// カートにアイテムを追加
    /// 同じSKUが既に存在する場合は数量を加算
    pub fn add_item(&mut self, item: CartItem) -> Result<(), DomainError> {
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.sku_id() == item.sku_id()) {
            existing_item.increase_quantity(item.quantity())?;
        } else {
            self.items.push(item);
        }
        Ok(())
    }

    /// SKUでアイテムを削除
    pub fn remove_item(&mut self, sku_id: &SKUId) {
        self.items.retain(|item| item.sku_id() != sku_id);
    }

    /// SKUのアイテム数量を更新
    pub fn update_item_quantity(
        &mut self,
        sku_id: &SKUId,
        new_quantity: u32,
    ) -> Result<(), DomainError> {
        if new_quantity == 0 {
            self.remove_item(sku_id);
            return Ok(());
        }

        if let Some(item) = self.items.iter_mut().find(|i| i.sku_id() == sku_id) {
            item.update_quantity(new_quantity)?;
        } else {
            return Err(DomainError::InvalidProductData(
                "Item not found in cart".to_string(),
            ));
        }
        Ok(())
    }


    /// カート内の全アイテムの総数量
    pub fn total_quantity(&self) -> u32 {
        self.items.iter().map(|item| item.quantity()).sum()
    }

    /// カート内のアイテム種類数
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// カートが空かどうか
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 配送料を計算
    /// Clean Architecture: Entity First - 配送料計算はCartの責務
    pub fn calculate_shipping_fee(
        &self,
        shipping_method: &crate::domain::entities::ShippingMethod,
    ) -> Result<Money, DomainError> {
        // 配送方法エンティティ全体を使用してより柔軟な計算が可能
        if !shipping_method.is_active() {
            return Err(DomainError::InvalidProductData(
                "Shipping method is not active".to_string(),
            ));
        }

        let calculation = self.calculate()?;
        let _cart_total = calculation.final_subtotal;

        // 将来的にはカート金額による配送料無料、重量制限、地域制限なども考慮可能
        // 現在はシンプルに配送方法の料金をそのまま適用
        Ok(*shipping_method.price())
    }

    /// 支払い手数料を計算
    /// Clean Architecture: Entity First - 支払い手数料計算はCartの責務
    pub fn calculate_payment_fee(
        &self,
        payment_method: &crate::domain::entities::PaymentMethod,
    ) -> Result<Money, DomainError> {
        let calculation = self.calculate()?;
        let cart_total = calculation.final_subtotal;

        // PaymentMethodエンティティに委譲してより豊富な情報を活用
        payment_method.calculate_fee(cart_total)
    }

    /// 配送方法を適用
    /// Clean Architecture: エンティティ内で状態管理とビジネスロジック完結
    pub fn apply_shipping_method(
        &mut self,
        shipping_method: &crate::domain::entities::ShippingMethod,
    ) -> Result<(), DomainError> {
        let fee = self.calculate_shipping_fee(shipping_method)?;
        self.shipping_fee = Some(fee);
        Ok(())
    }

    /// 支払い方法を適用
    /// Clean Architecture: エンティティ内で状態管理とビジネスロジック完結
    pub fn apply_payment_method(
        &mut self,
        payment_method: &crate::domain::entities::PaymentMethod,
    ) -> Result<(), DomainError> {
        let fee = self.calculate_payment_fee(payment_method)?;
        self.payment_fee = Some(fee);
        Ok(())
    }

    /// クーポンを適用
    /// Clean Architecture: エンティティ内で状態管理とビジネスロジック完結
    pub fn apply_coupon(
        &mut self,
        coupon: crate::domain::entities::Coupon,
    ) -> Result<(), DomainError> {
        // クーポンの有効性チェック
        if !coupon.is_valid() {
            return Err(DomainError::InvalidProductData(
                "Coupon is expired or invalid".to_string(),
            ));
        }

        if !coupon.is_valid_usage_limit() {
            return Err(DomainError::InvalidProductData(
                "Coupon usage limit exceeded".to_string(),
            ));
        }

        self.coupon = Some(coupon);
        Ok(())
    }

    /// クーポンを削除
    pub fn remove_coupon(&mut self) {
        self.coupon = None;
    }

    /// カートをクリア
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// 特定のSKUがカートに含まれているかチェック
    pub fn contains_sku(&self, sku_id: &SKUId) -> bool {
        self.items.iter().any(|item| item.sku_id() == sku_id)
    }

    /// 特定のSKUのアイテムを取得
    pub fn get_item(&self, sku_id: &SKUId) -> Option<&CartItem> {
        self.items.iter().find(|item| item.sku_id() == sku_id)
    }


    /// カート計算を一度に実行（重複計算を避ける）
    pub fn calculate(&self) -> Result<CartCalculationResult, DomainError> {
        // 1. 原価小計を計算
        let mut original_subtotal = Money::from_yen(0);
        for item in &self.items {
            let item_subtotal = item.subtotal()?;
            original_subtotal = original_subtotal.add(item_subtotal)?;
        }

        // 2. クーポン割引を計算
        let discount_amount = if let Some(coupon) = &self.coupon {
            let purchase_info = self.to_purchase_info(original_subtotal)?;
            let discount_result = CouponDiscountService::apply_coupon(coupon, &purchase_info)?;
            discount_result.discount_amount
        } else {
            Money::from_yen(0)
        };

        // 3. 割引後小計
        let final_subtotal = original_subtotal.subtract(discount_amount)?;

        // 4. 税額と税込み合計
        let tax_amount = final_subtotal.tax_amount();
        let total_with_tax = final_subtotal.with_tax();

        // 5. 手数料
        let shipping_fee = self.shipping_fee.unwrap_or(Money::from_yen(0));
        let payment_fee = self.payment_fee.unwrap_or(Money::from_yen(0));

        // 6. 最終合計を計算
        let grand_total = total_with_tax
            .add(shipping_fee)?
            .add(payment_fee)?;

        // 7. 結果を構築
        Ok(CartCalculationResult::new(
            original_subtotal,
            discount_amount,
            final_subtotal,
            tax_amount,
            total_with_tax,
            shipping_fee,
            payment_fee,
            grand_total,
        ))
    }

    // Getters
    pub fn items(&self) -> &[CartItem] {
        &self.items
    }

    pub fn shipping_fee(&self) -> Option<Money> {
        self.shipping_fee
    }

    pub fn payment_fee(&self) -> Option<Money> {
        self.payment_fee
    }

    pub fn coupon(&self) -> Option<&crate::domain::entities::Coupon> {
        self.coupon.as_ref()
    }

    /// PurchaseInfoに変換
    /// クーポン割引計算に必要な情報を集約したPurchaseInfoを生成
    fn to_purchase_info(&self, items_subtotal: Money) -> Result<PurchaseInfo, DomainError> {
        Ok(PurchaseInfo::new(
            self.items.clone(),
            items_subtotal,
            self.shipping_fee,
            self.payment_fee,
        ))
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_cart_item(name: &str, price: u32, quantity: u32) -> CartItem {
        CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            ProductId::from_uuid(Uuid::new_v4()),
            ProductName::new(name.to_string()).unwrap(),
            Money::from_yen(price),
            quantity,
        )
        .unwrap()
    }

    #[test]
    fn create_empty_cart() {
        let cart = Cart::new();
        assert!(cart.is_empty());
        assert_eq!(cart.item_count(), 0);
        assert_eq!(cart.total_quantity(), 0);
    }

    #[test]
    fn add_items_to_cart() {
        let mut cart = Cart::new();
        let item1 = create_test_cart_item("Product 1", 1000, 2);
        let item2 = create_test_cart_item("Product 2", 2000, 1);

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();

        assert_eq!(cart.item_count(), 2);
        assert_eq!(cart.total_quantity(), 3); // 2 + 1

        let calculation = cart.calculate().unwrap();
        assert_eq!(calculation.final_subtotal.yen(), 4000); // (1000 * 2) + (2000 * 1)
    }

    #[test]
    fn add_same_sku_increases_quantity() {
        let mut cart = Cart::new();
        let sku_id = SKUId::from_uuid(Uuid::new_v4());

        let item1 = CartItem::new(
            sku_id.clone(),
            ProductId::new(),
            ProductName::new("Product".to_string()).unwrap(),
            Money::from_yen(1000),
            2,
        )
        .unwrap();

        let item2 = CartItem::new(
            sku_id.clone(),
            ProductId::new(),
            ProductName::new("Product".to_string()).unwrap(),
            Money::from_yen(1000),
            3,
        )
        .unwrap();

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();

        assert_eq!(cart.item_count(), 1); // Still 1 unique item
        assert_eq!(cart.total_quantity(), 5); // 2 + 3

        let calculation = cart.calculate().unwrap();
        assert_eq!(calculation.final_subtotal.yen(), 5000); // 1000 * 5
    }

    #[test]
    fn remove_item_from_cart() {
        let mut cart = Cart::new();
        let item = create_test_cart_item("Product", 1000, 2);
        let sku_id = item.sku_id().clone();

        cart.add_item(item).unwrap();
        assert_eq!(cart.item_count(), 1);

        cart.remove_item(&sku_id);
        assert!(cart.is_empty());
    }

    #[test]
    fn update_item_quantity() {
        let mut cart = Cart::new();
        let item = create_test_cart_item("Product", 1000, 2);
        let sku_id = item.sku_id().clone();

        cart.add_item(item).unwrap();
        cart.update_item_quantity(&sku_id, 5).unwrap();

        assert_eq!(cart.total_quantity(), 5);
        let calculation = cart.calculate().unwrap();
        assert_eq!(calculation.final_subtotal.yen(), 5000);
    }

    #[test]
    fn update_quantity_to_zero_removes_item() {
        let mut cart = Cart::new();
        let item = create_test_cart_item("Product", 1000, 2);
        let sku_id = item.sku_id().clone();

        cart.add_item(item).unwrap();
        cart.update_item_quantity(&sku_id, 0).unwrap();

        assert!(cart.is_empty());
    }

    #[test]
    fn calculate_tax_amounts() {
        let mut cart = Cart::new();
        let item = create_test_cart_item("Product", 1000, 1);
        cart.add_item(item).unwrap();

        let calculation = cart.calculate().unwrap();

        assert_eq!(calculation.tax_amount.yen(), 100); // 10% of 1000
        assert_eq!(calculation.total_with_tax.yen(), 1100); // 1000 + 100
    }

    #[test]
    fn convert_to_purchase_info() {
        let mut cart = Cart::new();
        let item1 = create_test_cart_item("Product 1", 1000, 2);
        let item2 = create_test_cart_item("Product 2", 1500, 1);

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();
        cart.shipping_fee = Some(Money::from_yen(500));
        cart.payment_fee = Some(Money::from_yen(100));

        let subtotal = Money::from_yen(3500); // (1000 * 2) + (1500 * 1)
        let purchase_info = cart.to_purchase_info(subtotal).unwrap();

        assert_eq!(purchase_info.subtotal().yen(), 3500); // (1000 * 2) + (1500 * 1)
        assert!(purchase_info.meets_minimum_amount(Money::from_yen(3000)));
        assert!(!purchase_info.meets_minimum_amount(Money::from_yen(4000)));
    }

    #[test]
    fn convert_simple_cart_to_purchase_info() {
        let mut cart = Cart::new();
        let item = create_test_cart_item("Product", 2000, 1);
        cart.add_item(item).unwrap();

        let subtotal = Money::from_yen(2000);
        let purchase_info = cart.to_purchase_info(subtotal).unwrap();

        assert_eq!(purchase_info.subtotal().yen(), 2000);
    }
}
