use crate::application::dto::CalculateCartResultDto;
use crate::presentation::cart::responses::{AppliedCouponResponse, CalculateCartItemResponse, CalculateCartResponse, CouponErrorResponse};

/// カートプレゼンター
pub struct CartPresenter;

impl CartPresenter {
    /// CalculateCartResultDtoをCartResponseに変換
    /// 純粋なデータ変換のみを行う
    pub fn to_response(result: CalculateCartResultDto) -> CalculateCartResponse {
        // 各カートアイテムを変換
        let items = result
            .items
            .into_iter()
            .map(|item| CalculateCartItemResponse {
                sku_id: item.sku_id,
                product_id: item.product_id,
                product_name: item.product_name,
                unit_price: item.unit_price.yen(),
                quantity: item.quantity,
                subtotal: item.subtotal.yen(),
            })
            .collect();

        // クーポン適用結果を変換
        let applied_coupon = result.applied_coupon.map(|coupon| AppliedCouponResponse {
            coupon_code: coupon.coupon_code,
            coupon_name: coupon.coupon_name,
            discount_amount: coupon.discount_amount.yen(),
            message: coupon.message,
        });

        // クーポンエラーを変換
        let coupon_error = result.coupon_error.map(|error| CouponErrorResponse {
            coupon_code: error.coupon_code,
            error_message: error.error_message,
        });

        CalculateCartResponse {
            items,
            total_quantity: result.total_quantity,
            item_count: result.item_count,
            subtotal: result.subtotal.yen(),
            tax_amount: result.tax_amount.yen(),
            total: result.total_with_tax.yen(),
            is_empty: result.is_empty,
            shipping_fee: result.shipping_fee.yen(),
            payment_fee: result.payment_fee.yen(),
            applied_coupon,
            coupon_error,
        }
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
        )
        .unwrap()
    }

    #[test]
    fn empty_cart_to_response() {
        let cart = Cart::new();
        let result =
            CalculateCartResultDto::from_cart(cart, None)
                .unwrap();
        let response = CartPresenter::to_response(result);

        assert!(response.is_empty);
        assert_eq!(response.item_count, 0);
        assert_eq!(response.total_quantity, 0);
        assert_eq!(response.subtotal, 0);
        assert_eq!(response.tax_amount, 0);
        assert_eq!(response.total, 0);
        assert_eq!(response.shipping_fee, 0);
        assert_eq!(response.payment_fee, 0);
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
        )
        .unwrap();

        cart.add_item(item1).unwrap();
        cart.add_item(item2).unwrap();

        let result =
            CalculateCartResultDto::from_cart(cart, None)
                .unwrap();
        let response = CartPresenter::to_response(result);

        assert!(!response.is_empty);
        assert_eq!(response.item_count, 2);
        assert_eq!(response.total_quantity, 3); // 2 + 1
        assert_eq!(response.subtotal, 3500); // (1000 * 2) + (1500 * 1)
        assert_eq!(response.tax_amount, 350); // 10% tax
        assert_eq!(response.total, 3850); // subtotal + tax
        assert_eq!(response.shipping_fee, 0);
        assert_eq!(response.payment_fee, 0);
        assert_eq!(response.items.len(), 2);
    }

    #[test]
    fn cart_item_response_fields() {
        let mut cart = Cart::new();
        let item = create_test_cart_item();
        let sku_id = item.sku_id().to_string();
        let product_name = item.product_name().value().to_string();

        cart.add_item(item).unwrap();
        let result =
            CalculateCartResultDto::from_cart(cart, None)
                .unwrap();
        let response = CartPresenter::to_response(result);

        assert_eq!(response.items.len(), 1);
        let item_response = &response.items[0];
        assert_eq!(item_response.sku_id, sku_id);
        assert_eq!(item_response.product_name, product_name);
        assert_eq!(item_response.unit_price, 1000);
        assert_eq!(item_response.quantity, 2);
        assert_eq!(item_response.subtotal, 2000);
    }
}
