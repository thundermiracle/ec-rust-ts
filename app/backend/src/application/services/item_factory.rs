use crate::application::dto::VariantDTO;
use crate::application::error::ApplicationError;
use crate::domain::aggregates::{CartItem, order::OrderItem};
use crate::domain::value_objects::*;
use uuid::Uuid;

/// アイテム作成ファクトリー
/// VariantDTOからCartItemやOrderItemを作成する共通ロジックを提供
pub struct ItemFactory;

impl ItemFactory {
    /// VariantDTOからCartItemを作成
    pub fn create_cart_item(
        variant: &VariantDTO,
        quantity: u32,
    ) -> Result<CartItem, ApplicationError> {
        // SKUId
        let sku_id = SKUId::from_uuid(
            Uuid::parse_str(&variant.id)
                .map_err(|_| ApplicationError::InvalidInput("Invalid variant ID".to_string()))?,
        );

        // ProductId
        let product_id = ProductId::from_uuid(
            Uuid::parse_str(&variant.product_id)
                .map_err(|_| ApplicationError::InvalidInput("Invalid product ID".to_string()))?,
        );

        // ProductName
        let product_name = ProductName::new(variant.name.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        // 価格計算（セール価格優先）
        let unit_price = Money::from_yen(variant.sale_price.unwrap_or(variant.price));

        // CartItem作成
        CartItem::new(sku_id, product_id, product_name, unit_price, quantity)
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))
    }

    /// VariantDTOからOrderItemを作成
    pub fn create_order_item(
        variant: &VariantDTO,
        quantity: i32,
    ) -> Result<OrderItem, ApplicationError> {
        if quantity <= 0 {
            return Err(ApplicationError::InvalidInput(
                "Order item quantity must be positive".to_string(),
            ));
        }

        // SKUId
        let sku_id = SKUId::from_uuid(
            Uuid::parse_str(&variant.id)
                .map_err(|_| ApplicationError::InvalidInput("Invalid variant ID".to_string()))?,
        );

        // SKUCode
        let sku_code = SKUCode::new(variant.sku_code.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        // ProductName
        let product_name = ProductName::new(variant.name.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        // SKUName（バリアント名）
        let sku_name = SKUName::new(variant.name.clone())
            .map_err(|e| ApplicationError::InvalidInput(format!("{:?}", e)))?;

        // 価格計算（セール価格優先）
        let unit_price = Money::from_yen(variant.sale_price.unwrap_or(variant.price));

        // OrderItem作成
        OrderItem::new(
            sku_id,
            sku_code,
            product_name,
            sku_name,
            unit_price,
            quantity,
        )
        .map_err(|e| ApplicationError::InvalidInput(e.to_string()))
    }

    /// 複数のVariantDTOからCartItemのベクターを作成
    pub fn create_cart_items(
        items: &[(String, u32)], // (sku_id, quantity)
        variants: &[VariantDTO],
    ) -> Result<Vec<CartItem>, ApplicationError> {
        let mut cart_items = Vec::new();

        for (sku_id, quantity) in items {
            if *quantity == 0 {
                return Err(ApplicationError::InvalidInput(
                    "Cart item quantity must be greater than zero".to_string(),
                ));
            }

            let variant = variants.iter().find(|v| v.id == *sku_id).ok_or_else(|| {
                ApplicationError::NotFound(format!("Variant not found for SKU: {}", sku_id))
            })?;

            let cart_item = Self::create_cart_item(variant, *quantity)?;
            cart_items.push(cart_item);
        }

        Ok(cart_items)
    }

    /// 複数のVariantDTOからOrderItemのベクターを作成
    pub fn create_order_items(
        items: &[(String, u32)], // (sku_id, quantity)
        variants: &[VariantDTO],
    ) -> Result<Vec<OrderItem>, ApplicationError> {
        let mut order_items = Vec::new();

        for (sku_id, quantity) in items {
            if *quantity == 0 {
                return Err(ApplicationError::InvalidInput(
                    "Order item quantity must be greater than zero".to_string(),
                ));
            }

            let variant = variants.iter().find(|v| v.id == *sku_id).ok_or_else(|| {
                ApplicationError::NotFound(format!("Variant not found for SKU: {}", sku_id))
            })?;

            let order_item = Self::create_order_item(variant, *quantity as i32)?;
            order_items.push(order_item);
        }

        Ok(order_items)
    }
}
