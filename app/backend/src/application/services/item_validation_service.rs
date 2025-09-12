use crate::application::dto::VariantDTO;
use crate::application::error::ApplicationError;
use crate::application::repositories::ProductRepository;
use crate::domain::value_objects::SKUId;
use std::sync::Arc;
use uuid::Uuid;

/// アイテム検証サービス
/// CartとOrderで共通するSKU検証・在庫チェックロジックを提供
pub struct ItemValidationService {
    product_repository: Arc<dyn ProductRepository>,
}

impl ItemValidationService {
    pub fn new(product_repository: Arc<dyn ProductRepository>) -> Self {
        Self { product_repository }
    }

    /// 複合的な検証：SKU存在チェック + 在庫チェック
    pub async fn validate_items_with_stock(
        &self,
        items: &[(String, u32)], // (sku_id, quantity)のタプル
    ) -> Result<Vec<VariantDTO>, ApplicationError> {
        // SKU IDを抽出・変換
        let sku_id_strings: Vec<String> = items.iter().map(|(sku_id, _)| sku_id.clone()).collect();
        let sku_ids = Self::parse_sku_ids(&sku_id_strings)?;

        // バリアント情報を取得
        let variants = self.get_variants(&sku_ids).await?;

        // 全SKUの存在確認
        Self::validate_all_skus_exist(&sku_id_strings, &variants)?;

        // 在庫チェック
        for (sku_id, quantity) in items {
            let variant = Self::find_variant_by_sku_id(sku_id, &variants)?;
            Self::validate_stock_availability(variant, *quantity)?;
        }

        Ok(variants)
    }

    /// SKU ID文字列をSKUIdドメインオブジェクトに変換
    fn parse_sku_ids(sku_id_strings: &[String]) -> Result<Vec<SKUId>, ApplicationError> {
        sku_id_strings
            .iter()
            .map(|sku_id_str| {
                Uuid::parse_str(sku_id_str)
                    .map(SKUId::from_uuid)
                    .map_err(|_| {
                        ApplicationError::InvalidInput(format!(
                            "Invalid SKU ID format: {}",
                            sku_id_str
                        ))
                    })
            })
            .collect()
    }

    /// バリアント情報を取得し、基本検証を実行
    async fn get_variants(&self, sku_ids: &[SKUId]) -> Result<Vec<VariantDTO>, ApplicationError> {
        self.product_repository
            .find_variants_by_ids(sku_ids)
            .await
            .map_err(ApplicationError::Repository)
    }

    /// 要求されたSKUがすべて存在することを検証
    fn validate_all_skus_exist(
        requested_sku_ids: &[String],
        variants: &[VariantDTO],
    ) -> Result<(), ApplicationError> {
        for sku_id in requested_sku_ids {
            let found = variants.iter().any(|variant| variant.id == *sku_id);
            if !found {
                return Err(ApplicationError::NotFound(format!(
                    "SKU not found: {}",
                    sku_id
                )));
            }
        }
        Ok(())
    }

    /// 在庫状況をチェック
    fn validate_stock_availability(
        variant: &VariantDTO,
        requested_quantity: u32,
    ) -> Result<(), ApplicationError> {
        if variant.is_sold_out {
            return Err(ApplicationError::InvalidInput(format!(
                "SKU {} is sold out",
                variant.sku_code
            )));
        }

        if variant.stock_quantity < requested_quantity {
            return Err(ApplicationError::InvalidInput(format!(
                "Insufficient stock for SKU {}: requested {}, available {}",
                variant.sku_code, requested_quantity, variant.stock_quantity
            )));
        }

        Ok(())
    }

    /// SKU ID文字列からVariantDTOを検索
    fn find_variant_by_sku_id<'a>(
        sku_id: &str,
        variants: &'a [VariantDTO],
    ) -> Result<&'a VariantDTO, ApplicationError> {
        variants.iter().find(|v| v.id == sku_id).ok_or_else(|| {
            ApplicationError::NotFound(format!("Variant not found for SKU: {}", sku_id))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_sku_ids() {
        let sku_ids = vec![
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string(),
        ];

        let result = ItemValidationService::parse_sku_ids(&sku_ids);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_parse_invalid_sku_id() {
        let sku_ids = vec!["invalid-uuid".to_string()];
        let result = ItemValidationService::parse_sku_ids(&sku_ids);
        assert!(result.is_err());
    }
}
