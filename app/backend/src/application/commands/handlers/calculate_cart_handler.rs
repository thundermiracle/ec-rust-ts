use std::sync::Arc;
use uuid::Uuid;
use crate::application::commands::models::CalculateCartCommand;
use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::domain::{Cart, CartItem, SKUId, ProductId, ProductName, Money};

/// カート計算ハンドラ（ユースケース）
pub struct CalculateCartHandler<R: ProductRepository> {
    product_repository: Arc<R>,
}

impl<R: ProductRepository> CalculateCartHandler<R> {
    pub fn new(product_repository: Arc<R>) -> Self {
        Self {
            product_repository,
        }
    }

    /// カート計算を実行
    pub async fn handle(&self, command: CalculateCartCommand) -> Result<Cart, ApplicationError> {
        // 1. すべてのsku_idを抽出
        let sku_ids: Result<Vec<SKUId>, ApplicationError> = command
            .items
            .iter()
            .map(|item| {
                Uuid::parse_str(&item.sku_id)
                    .map(SKUId::from_uuid)
                    .map_err(|_| ApplicationError::InvalidInput(
                        format!("Invalid SKU ID format: {}", item.sku_id)
                    ))
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
                return Err(ApplicationError::NotFound(
                    format!("SKU not found: {}", item_request.sku_id)
                ));
            }
        }

        // 4. CartItemドメインオブジェクトを作成
        let mut cart_items = Vec::new();
        for item_request in &command.items {
            if item_request.quantity == 0 {
                return Err(ApplicationError::InvalidInput(
                    "Item quantity must be greater than zero".to_string()
                ));
            }

            // 対応するバリアント情報を見つける
            let variant = variants
                .iter()
                .find(|v| v.id == item_request.sku_id)
                .expect("Variant should exist after validation");

            // SKUが購入可能かチェック
            if variant.is_sold_out {
                return Err(ApplicationError::InvalidInput(
                    format!("SKU {} is sold out", variant.sku_code)
                ));
            }

            if variant.stock_quantity < item_request.quantity {
                return Err(ApplicationError::InvalidInput(
                    format!(
                        "Insufficient stock for SKU {}: requested {}, available {}",
                        variant.sku_code, item_request.quantity, variant.stock_quantity
                    )
                ));
            }

            // ドメインオブジェクトを作成
            let sku_id = SKUId::from_uuid(
                Uuid::parse_str(&variant.id)
                    .map_err(|_| ApplicationError::InvalidInput("Invalid variant ID".to_string()))?
            );

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
            ).map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            cart_items.push(cart_item);
        }

        // 5. Cartアグリゲートを作成
        Ok(Cart::from_items(cart_items))
    }
} 