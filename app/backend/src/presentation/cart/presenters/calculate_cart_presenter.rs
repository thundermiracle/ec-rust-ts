use crate::application::dto::CalculateCartResultDto;
use crate::presentation::cart::responses::CalculateCartResponse;
use crate::presentation::cart::responses::CalculateCartItemResponse;

/// カートプレゼンター
pub struct CartPresenter;

impl CartPresenter {
    /// CalculateCartResultDtoをCartResponseに変換
    pub fn to_response(result: CalculateCartResultDto) -> Result<CalculateCartResponse, String> {
        let cart = &result.cart;
        let mut items = Vec::new();
        
        // 各カートアイテムを変換
        for item in cart.items() {
            let subtotal = item.subtotal()
                .map_err(|e| format!("Failed to calculate subtotal: {}", e))?;
            
            let cart_item_response = CalculateCartItemResponse {
                sku_id: item.sku_id().to_string(),
                product_id: item.product_id().to_string(),
                product_name: item.product_name().value().to_string(),
                unit_price: item.unit_price().yen(),
                quantity: item.quantity(),
                subtotal: subtotal.yen(),
            };
            
            items.push(cart_item_response);
        }

        // カート全体の計算
        let subtotal = cart.subtotal()
            .map_err(|e| format!("Failed to calculate cart subtotal: {}", e))?;
        
        let tax_amount = cart.tax_amount()
            .map_err(|e| format!("Failed to calculate tax amount: {}", e))?;
        
        let total_with_tax = cart.total_with_tax()
            .map_err(|e| format!("Failed to calculate total with tax: {}", e))?;

        Ok(CalculateCartResponse {
            items,
            total_quantity: cart.total_quantity(),
            item_count: cart.item_count(),
            subtotal: subtotal.yen(),
            tax_amount: tax_amount.yen(),
            total: total_with_tax.yen(),
            is_empty: cart.is_empty(),
            shipping_fee: result.shipping_fee.yen(),
            payment_fee: result.payment_fee.yen(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;
    use uuid::Uuid;

    fn create_test_cart_item() -> CartItem {
        CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            ProductId::from_uuid(Uuid::new_v4()),
            ProductName::new("Test Product".to_string()).unwrap(),
            Money::from_yen(1000),
            2,
        ).unwrap()
    }

    #[test]
    fn empty_cart_to_response() {
        let cart = Cart::new();
        let result = CalculateCartResultDto::new(cart, Money::from_yen(500), Money::from_yen(200));
        let response = CartPresenter::to_response(result).unwrap();
        
        assert!(response.is_empty);
        assert_eq!(response.item_count, 0);
        assert_eq!(response.total_quantity, 0);
        assert_eq!(response.subtotal, 0);
        assert_eq!(response.tax_amount, 0);
        assert_eq!(response.total, 0);
        assert_eq!(response.shipping_fee, 500);
        assert_eq!(response.payment_fee, 200);
        assert!(response.items.is_empty());
    }

    #[test]
    fn cart_with_items_to_response() {
        let mut cart = Cart::new();
        let item1 = create_test_cart_item();
        let item2 = CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            ProductId::from_uuid(Uuid::new_v4()),
            ProductName::new("Another Product".to_string()).unwrap(),
            Money::from_yen(1500),
            1,
        ).unwrap();

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();

        let result = CalculateCartResultDto::new(cart, Money::from_yen(500), Money::from_yen(330));
        let response = CartPresenter::to_response(result).unwrap();
        
        assert!(!response.is_empty);
        assert_eq!(response.item_count, 2);
        assert_eq!(response.total_quantity, 3); // 2 + 1
        assert_eq!(response.subtotal, 3500); // (1000 * 2) + (1500 * 1)
        assert_eq!(response.tax_amount, 350); // 10% tax
        assert_eq!(response.total, 3850); // subtotal + tax
        assert_eq!(response.shipping_fee, 500);
        assert_eq!(response.payment_fee, 330);
        assert_eq!(response.items.len(), 2);
    }

    #[test]
    fn cart_item_response_fields() {
        let mut cart = Cart::new();
        let item = create_test_cart_item();
        let sku_id = item.sku_id().to_string();
        let product_name = item.product_name().value().to_string();
        
        cart.add_item(item).unwrap();
        let result = CalculateCartResultDto::new(cart, Money::from_yen(500), Money::from_yen(0));
        let response = CartPresenter::to_response(result).unwrap();
        
        assert_eq!(response.items.len(), 1);
        let item_response = &response.items[0];
        assert_eq!(item_response.sku_id, sku_id);
        assert_eq!(item_response.product_name, product_name);
        assert_eq!(item_response.unit_price, 1000);
        assert_eq!(item_response.quantity, 2);
        assert_eq!(item_response.subtotal, 2000);
    }
}
