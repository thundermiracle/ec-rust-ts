use crate::application::commands::models::CalculateCartCommand;
use crate::application::dto::{CalculateCartResultDto, CouponErrorDto};
use crate::application::error::ApplicationError;
use crate::application::repositories::{
    CouponRepository, PaymentMethodRepository, ProductRepository, ShippingMethodRepository,
};
use crate::application::services::{ItemFactory, ItemValidationService};
use crate::domain::value_objects::CouponCode;
use crate::domain::Cart;
use std::sync::Arc;

/// カート計算ハンドラ（ユースケース）
pub struct CalculateCartHandler {
    item_validation_service: ItemValidationService,
    shipping_method_repository: Arc<dyn ShippingMethodRepository>,
    payment_method_repository: Arc<dyn PaymentMethodRepository>,
    coupon_repository: Arc<dyn CouponRepository>,
}

impl CalculateCartHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        shipping_method_repository: Arc<dyn ShippingMethodRepository>,
        payment_method_repository: Arc<dyn PaymentMethodRepository>,
        coupon_repository: Arc<dyn CouponRepository>,
    ) -> Self {
        let item_validation_service = ItemValidationService::new(product_repository);
        Self {
            item_validation_service,
            shipping_method_repository,
            payment_method_repository,
            coupon_repository,
        }
    }

    async fn try_apply_coupon(
        &self,
        cart: &mut Cart,
        coupon_code_str: String,
    ) -> Option<CouponErrorDto> {
        let coupon_code = match CouponCode::from_string(coupon_code_str.clone()) {
            Ok(code) => code,
            Err(e) => {
                return Some(CouponErrorDto {
                    coupon_code: Some(coupon_code_str),
                    error_message: e.to_string(),
                });
            }
        };

        let coupon = match self.coupon_repository.find_by_code(&coupon_code).await {
            Ok(Some(coupon)) => coupon,
            Ok(None) => {
                return Some(CouponErrorDto {
                    coupon_code: Some(coupon_code_str),
                    error_message: "Coupon not found".to_string(),
                });
            }
            Err(e) => {
                return Some(CouponErrorDto {
                    coupon_code: Some(coupon_code_str),
                    error_message: format!("Failed to find coupon: {}", e),
                });
            }
        };

        match cart.apply_coupon(coupon) {
            Ok(_) => None,
            Err(e) => Some(CouponErrorDto {
                coupon_code: Some(coupon_code_str),
                error_message: e.to_string(),
            }),
        }
    }

    /// カート計算を実行
    pub async fn handle(
        &self,
        command: CalculateCartCommand,
    ) -> Result<CalculateCartResultDto, ApplicationError> {
        // 1. リクエストデータを準備
        let items: Vec<(String, u32)> = command
            .items
            .iter()
            .map(|item| (item.sku_id.clone(), item.quantity))
            .collect();

        // 2. 共通サービスを使用して検証とバリアント取得
        let variants = self
            .item_validation_service
            .validate_items_with_stock(&items)
            .await?;

        // 3. 共通ファクトリーを使用してCartItem作成
        let cart_items = ItemFactory::create_cart_items(&items, &variants)?;

        // 4. 配送方法の取得
        let shipping_method = self
            .shipping_method_repository
            .find_by_id(&command.shipping_method_id)
            .await
            .map_err(ApplicationError::Repository)?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!(
                    "Shipping method not found: {}",
                    command.shipping_method_id
                ))
            })?;

        // 5. 支払い方法の取得
        let payment_method = self
            .payment_method_repository
            .find_by_id(&command.payment_method_id)
            .await
            .map_err(ApplicationError::Repository)?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!(
                    "Payment method not found: {}",
                    command.payment_method_id
                ))
            })?;

        // 6. Cart作成と設定（Domain層で全て完結）
        let mut cart = Cart::from_items(cart_items);
        cart.apply_shipping_method(&shipping_method)
            .map_err(ApplicationError::Domain)?;
        cart.apply_payment_method(&payment_method)
            .map_err(ApplicationError::Domain)?;

        // 7. クーポン適用処理（エラーハンドリング改善）
        let coupon_error = if let Some(coupon_code_str) = command.coupon_code.clone() {
            self.try_apply_coupon(&mut cart, coupon_code_str).await
        } else {
            None
        };

        CalculateCartResultDto::from_cart(cart, coupon_error)
            .map_err(ApplicationError::InvalidInput)
    }
}
