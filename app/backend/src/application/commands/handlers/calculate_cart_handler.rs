use crate::application::commands::models::CalculateCartCommand;
use crate::application::dto::CalculateCartResultDto;
use crate::application::error::ApplicationError;
use crate::application::repositories::{
    PaymentMethodRepository, ProductRepository, ShippingMethodRepository,
};
use crate::domain::{Cart, CartItem, Money, ProductId, ProductName, SKUId};
use std::sync::Arc;
use uuid::Uuid;

/// カート計算ハンドラ（ユースケース）
pub struct CalculateCartHandler {
    product_repository: Arc<dyn ProductRepository>,
    shipping_method_repository: Arc<dyn ShippingMethodRepository>,
    payment_method_repository: Arc<dyn PaymentMethodRepository>,
}

impl CalculateCartHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        shipping_method_repository: Arc<dyn ShippingMethodRepository>,
        payment_method_repository: Arc<dyn PaymentMethodRepository>,
    ) -> Self {
        Self {
            product_repository,
            shipping_method_repository,
            payment_method_repository,
        }
    }

    /// カート計算を実行
    pub async fn handle(
        &self,
        command: CalculateCartCommand,
    ) -> Result<CalculateCartResultDto, ApplicationError> {
        // 1. すべてのsku_idを抽出
        let sku_ids: Result<Vec<SKUId>, ApplicationError> = command
            .items
            .iter()
            .map(|item| {
                Uuid::parse_str(&item.sku_id)
                    .map(SKUId::from_uuid)
                    .map_err(|_| {
                        ApplicationError::InvalidInput(format!(
                            "Invalid SKU ID format: {}",
                            item.sku_id
                        ))
                    })
            })
            .collect();

        let sku_ids = sku_ids?;

        // 2. リポジトリから複数のSKU情報を取得
        let variants = self
            .product_repository
            .find_variants_by_ids(&sku_ids)
            .await
            .map_err(ApplicationError::Repository)?;

        // 3. 要求されたすべてのSKUが見つかることを検証
        for item_request in &command.items {
            let found = variants
                .iter()
                .any(|variant| variant.id == item_request.sku_id);

            if !found {
                return Err(ApplicationError::NotFound(format!(
                    "SKU not found: {}",
                    item_request.sku_id
                )));
            }
        }

        // 4. CartItemドメインオブジェクトを作成
        let mut cart_items = Vec::new();
        for item_request in &command.items {
            if item_request.quantity == 0 {
                return Err(ApplicationError::InvalidInput(
                    "Item quantity must be greater than zero".to_string(),
                ));
            }

            // 対応するバリアント情報を見つける
            let variant = variants
                .iter()
                .find(|v| v.id == item_request.sku_id)
                .expect("Variant should exist after validation");

            // SKUが購入可能かチェック
            if variant.is_sold_out {
                return Err(ApplicationError::InvalidInput(format!(
                    "SKU {} is sold out",
                    variant.sku_code
                )));
            }

            if variant.stock_quantity < item_request.quantity {
                return Err(ApplicationError::InvalidInput(format!(
                    "Insufficient stock for SKU {}: requested {}, available {}",
                    variant.sku_code, item_request.quantity, variant.stock_quantity
                )));
            }

            // ドメインオブジェクトを作成
            let sku_id =
                SKUId::from_uuid(Uuid::parse_str(&variant.id).map_err(|_| {
                    ApplicationError::InvalidInput("Invalid variant ID".to_string())
                })?);

            // ProductIdは実際のプロダクト情報から取得する必要があるが、
            // 現在のVariantDTOには含まれていないため、新規UUIDを生成
            // TODO: VariantDTOにproduct_idを追加するか、別途取得する
            let product_id = ProductId::new();

            let product_name = ProductName::new(variant.name.clone())
                .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            // 価格を決定（セール価格がある場合はそれを使用）
            let unit_price = Money::from_yen(variant.sale_price.unwrap_or(variant.price));

            let cart_item = CartItem::new(
                sku_id,
                product_id,
                product_name,
                unit_price,
                item_request.quantity,
            )
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            cart_items.push(cart_item);
        }

        // 5. 配送方法の取得
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

        // 6. 支払い方法の取得
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

        // 7. Cart作成と設定（Domain層で全て完結）
        let mut cart = Cart::from_items(cart_items);
        cart.apply_shipping_method(&shipping_method)
            .map_err(ApplicationError::Domain)?;
        cart.apply_payment_method(&payment_method)
            .map_err(ApplicationError::Domain)?;

        // 8. 手数料をCartから取得
        let shipping_fee = cart.shipping_fee().unwrap_or(Money::from_yen(0));
        let payment_fee = cart.payment_fee().unwrap_or(Money::from_yen(0));

        CalculateCartResultDto::from_cart(cart, shipping_fee, payment_fee)
            .map_err(ApplicationError::InvalidInput)
    }
}
