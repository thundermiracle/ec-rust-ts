use crate::domain::aggregates::CartItem;
use crate::domain::error::DomainError;
use crate::domain::value_objects::Money;

/// 購入情報値オブジェクト
/// クーポン割引計算に必要な購入関連情報を集約
#[derive(Debug, Clone, PartialEq)]
pub struct PurchaseInfo {
    /// カートアイテムのリスト
    cart_items: Vec<CartItem>,
    /// 商品の小計（割引前の金額）
    subtotal: Money,
    /// 配送料
    shipping_fee: Option<Money>,
    /// 支払い手数料
    payment_fee: Option<Money>,
}

impl PurchaseInfo {
    /// カートアイテムのリストから作成（小計は自動計算）
    pub fn from_cart_items(
        cart_items: Vec<CartItem>,
        shipping_fee: Option<Money>,
        payment_fee: Option<Money>,
    ) -> Result<Self, DomainError> {
        let subtotal = Self::calculate_subtotal(&cart_items)?;

        Ok(Self {
            cart_items,
            subtotal,
            shipping_fee,
            payment_fee,
        })
    }

    /// カートアイテムから小計を計算
    fn calculate_subtotal(cart_items: &[CartItem]) -> Result<Money, DomainError> {
        let mut subtotal = Money::from_yen(0);
        for item in cart_items {
            let item_subtotal = item.subtotal()?;
            subtotal = subtotal.add(item_subtotal)?;
        }
        Ok(subtotal)
    }

    /// 配送料と支払い手数料を含まない総額（商品のみ）
    pub fn subtotal(&self) -> Money {
        self.subtotal
    }

    /// 特定の商品IDが含まれているかどうかを判定
    pub fn contains_product(&self, product_id: &crate::domain::value_objects::ProductId) -> bool {
        self.cart_items
            .iter()
            .any(|item| item.product_id() == product_id)
    }

    /// 最低購入金額の条件を満たしているかを判定
    pub fn meets_minimum_amount(&self, minimum_amount: Money) -> bool {
        self.subtotal >= minimum_amount
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{ProductId, ProductName, SKUId};
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
    fn create_purchase_info_from_cart_items() {
        let items = vec![
            create_test_cart_item("Product 1", 1000, 2),
            create_test_cart_item("Product 2", 2000, 1),
        ];
        let shipping_fee = Some(Money::from_yen(500));
        let payment_fee = Some(Money::from_yen(100));

        let purchase_info =
            PurchaseInfo::from_cart_items(items, shipping_fee, payment_fee).unwrap();

        assert_eq!(purchase_info.subtotal().yen(), 4000); // (1000 * 2) + (2000 * 1)
    }

    #[test]
    fn meets_minimum_amount_check() {
        let items = vec![create_test_cart_item("Product", 1000, 1)];
        let purchase_info = PurchaseInfo::from_cart_items(items, None, None).unwrap();

        assert!(purchase_info.meets_minimum_amount(Money::from_yen(1000)));
        assert!(purchase_info.meets_minimum_amount(Money::from_yen(500)));
        assert!(!purchase_info.meets_minimum_amount(Money::from_yen(1500)));
    }

    #[test]
    fn product_containment() {
        let product_id = ProductId::from_uuid(Uuid::new_v4());
        let sku_id = SKUId::from_uuid(Uuid::new_v4());

        let item = CartItem::new(
            sku_id.clone(),
            product_id.clone(),
            ProductName::new("Product".to_string()).unwrap(),
            Money::from_yen(1000),
            1,
        )
        .unwrap();

        let purchase_info = PurchaseInfo::from_cart_items(vec![item], None, None).unwrap();

        assert!(purchase_info.contains_product(&product_id));

        let other_product_id = ProductId::from_uuid(Uuid::new_v4());
        assert!(!purchase_info.contains_product(&other_product_id));
    }
}
