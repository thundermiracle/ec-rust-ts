use crate::domain::aggregates::CartItem;
use crate::domain::value_objects::*;
use crate::domain::error::DomainError;

/// カートアグリゲート
#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    items: Vec<CartItem>,
}

impl Cart {
    /// 空のカートを作成
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// カートアイテムのリストからカートを作成
    pub fn from_items(items: Vec<CartItem>) -> Self {
        Self { items }
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
    pub fn update_item_quantity(&mut self, sku_id: &SKUId, new_quantity: u32) -> Result<(), DomainError> {
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

    /// カートの総額を計算
    pub fn total(&self) -> Result<Money, DomainError> {
        let mut total = Money::from_yen(0);
        for item in &self.items {
            let subtotal = item.subtotal()?;
            total = total.add(subtotal)?;
        }
        Ok(total)
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

    /// 税込み総額を計算
    pub fn total_with_tax(&self) -> Result<Money, DomainError> {
        let subtotal = self.total()?;
        Ok(subtotal.with_tax())
    }

    /// 税額を計算
    pub fn tax_amount(&self) -> Result<Money, DomainError> {
        let subtotal = self.total()?;
        Ok(subtotal.tax_amount())
    }

    // Getters
    pub fn items(&self) -> &[CartItem] {
        &self.items
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
        ).unwrap()
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
        
        let total = cart.total().unwrap();
        assert_eq!(total.yen(), 4000); // (1000 * 2) + (2000 * 1)
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
        ).unwrap();
        
        let item2 = CartItem::new(
            sku_id.clone(),
            ProductId::new(),
            ProductName::new("Product".to_string()).unwrap(),
            Money::from_yen(1000),
            3,
        ).unwrap();

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();

        assert_eq!(cart.item_count(), 1); // Still 1 unique item
        assert_eq!(cart.total_quantity(), 5); // 2 + 3
        
        let total = cart.total().unwrap();
        assert_eq!(total.yen(), 5000); // 1000 * 5
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
        let total = cart.total().unwrap();
        assert_eq!(total.yen(), 5000);
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

        let total_with_tax = cart.total_with_tax().unwrap();
        let tax_amount = cart.tax_amount().unwrap();
        
        assert_eq!(tax_amount.yen(), 100); // 10% of 1000
        assert_eq!(total_with_tax.yen(), 1100); // 1000 + 100
    }
} 