use crate::domain::models::value_objects::*;
use crate::domain::error::DomainError;
use crate::domain::models::{SKU, StockAdjustment, ProductImage, Tag};
use chrono::{DateTime, Utc};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Product {
    // 基本情報
    id: ProductId,
    name: ProductName,
    description: Description,
    category_id: CategoryId,
    is_best_seller: bool,
    is_quick_ship: bool,
    is_available: bool,
    
    // SKUs（このProductに属するSKU）
    skus: Vec<SKU>,
    
    // 関連エンティティ
    images: Vec<ProductImage>,
    tags: Vec<Tag>,
    
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Product {
    pub fn create(
        id: ProductId,
        name: ProductName,
        description: Description,
        category_id: CategoryId,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            name,
            description,
            category_id,
            is_best_seller: false,
            is_quick_ship: false,
            is_available: false,
            skus: vec![],
            images: vec![],
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // SKU管理
    pub fn add_sku(&mut self, sku: SKU) -> Result<(), DomainError> {
        // ビジネスルール: 同じSKUコードは追加不可
        if self.skus.iter().any(|s| s.sku_code() == sku.sku_code()) {
            return Err(DomainError::BusinessRuleViolation(
                format!("SKU code '{}' already exists in this product", sku.sku_code().value()),
            ));
        }

        // ビジネスルール: SKUは同じProductに属する必要がある
        if sku.product_id() != &self.id {
            return Err(DomainError::BusinessRuleViolation(
                "SKU must belong to this product".to_string(),
            ));
        }

        self.skus.push(sku);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_sku(&mut self, sku_id: &SKUId) -> Result<SKU, DomainError> {
        let index = self.skus.iter().position(|s| s.id() == sku_id)
            .ok_or_else(|| DomainError::BusinessRuleViolation("SKU not found".to_string()))?;

        let removed_sku = self.skus.remove(index);
        self.updated_at = Utc::now();
        Ok(removed_sku)
    }

    pub fn find_sku_by_id(&self, sku_id: &SKUId) -> Option<&SKU> {
        self.skus.iter().find(|s| s.id() == sku_id)
    }

    pub fn find_sku_by_id_mut(&mut self, sku_id: &SKUId) -> Option<&mut SKU> {
        self.skus.iter_mut().find(|s| s.id() == sku_id)
    }

    pub fn find_sku_by_code(&self, sku_code: &SKUCode) -> Option<&SKU> {
        self.skus.iter().find(|s| s.sku_code() == sku_code)
    }

    // 在庫操作（Product経由でSKU操作）
    pub fn adjust_sku_stock(&mut self, sku_id: &SKUId, adjustment: StockAdjustment) -> Result<(), DomainError> {
        let sku = self.find_sku_by_id_mut(sku_id)
            .ok_or_else(|| DomainError::BusinessRuleViolation("SKU not found".to_string()))?;
        
        sku.adjust_stock(adjustment)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reserve_sku_stock(&mut self, sku_id: &SKUId, quantity: u32) -> Result<(), DomainError> {
        let sku = self.find_sku_by_id_mut(sku_id)
            .ok_or_else(|| DomainError::BusinessRuleViolation("SKU not found".to_string()))?;
        
        sku.reserve_stock(quantity)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_sku_sale_price(&mut self, sku_id: &SKUId, sale_price: Money) -> Result<(), DomainError> {
        let sku = self.find_sku_by_id_mut(sku_id)
            .ok_or_else(|| DomainError::BusinessRuleViolation("SKU not found".to_string()))?;
        
        sku.set_sale_price(sale_price)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Product レベルのビジネスロジック
    pub fn has_variants(&self) -> bool {
        self.skus.len() > 1 || 
        self.skus.iter().any(|sku| sku.variant_attributes().has_any_attributes())
    }

    pub fn is_available_for_purchase(&self) -> bool {
        self.is_available && self.skus.iter().any(|sku| sku.is_purchasable())
    }

    pub fn total_available_stock(&self) -> u32 {
        self.skus.iter().map(|sku| sku.available_quantity()).sum()
    }

    pub fn price_range(&self) -> Option<(Money, Money)> {
        if self.skus.is_empty() {
            return None;
        }

        let prices: Vec<Money> = self.skus.iter().map(|sku| sku.current_price()).collect();
        let min_price = prices.iter().min().cloned()?;
        let max_price = prices.iter().max().cloned()?;
        
        Some((min_price, max_price))
    }

    pub fn available_colors(&self) -> Vec<ColorId> {
        let mut colors = Vec::new();
        let mut seen_colors = HashSet::new();

        for sku in &self.skus {
            if let Some(color_id) = sku.variant_attributes().color_id() {
                if seen_colors.insert(color_id.value()) {
                    colors.push(color_id.clone());
                }
            }
        }

        colors
    }

    pub fn available_dimensions(&self) -> Vec<Dimensions> {
        let mut dimensions = Vec::new();
        let mut seen_dimensions = HashSet::new();

        for sku in &self.skus {
            if let Some(dim) = sku.variant_attributes().dimensions() {
                if seen_dimensions.insert(dim.value()) {
                    dimensions.push(dim.clone());
                }
            }
        }

        dimensions
    }

    pub fn available_materials(&self) -> Vec<Material> {
        let mut materials = Vec::new();
        let mut seen_materials = HashSet::new();

        for sku in &self.skus {
            if let Some(material) = sku.variant_attributes().material() {
                if seen_materials.insert(material.value()) {
                    materials.push(material.clone());
                }
            }
        }

        materials
    }

    pub fn low_stock_skus(&self) -> Vec<&SKU> {
        self.skus.iter().filter(|sku| sku.is_low_stock()).collect()
    }

    pub fn out_of_stock_skus(&self) -> Vec<&SKU> {
        self.skus.iter().filter(|sku| sku.is_out_of_stock()).collect()
    }

    // 画像管理
    pub fn add_image(&mut self, image: ProductImage) {
        self.images.push(image);
        self.updated_at = Utc::now();
    }

    pub fn main_image(&self) -> Option<&ProductImage> {
        self.images.iter().find(|img| img.is_main_image())
    }

    // タグ管理
    pub fn add_tag(&mut self, tag: Tag) {
        if !self.tags.iter().any(|t| t.slug().value() == tag.slug().value()) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag_slug: &str) {
        self.tags.retain(|t| t.slug().value() != tag_slug);
        self.updated_at = Utc::now();
    }

    // 商品の公開・非公開
    pub fn publish(&mut self) -> Result<(), DomainError> {
        // ビジネスルール: SKUが存在しない場合は公開不可
        if self.skus.is_empty() {
            return Err(DomainError::BusinessRuleViolation(
                "Cannot publish product without SKUs".to_string(),
            ));
        }

        // ビジネスルール: 購入可能なSKUが必要
        if !self.skus.iter().any(|sku| sku.is_purchasable()) {
            return Err(DomainError::BusinessRuleViolation(
                "Cannot publish product without purchasable SKUs".to_string(),
            ));
        }

        self.is_available = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn discontinue(&mut self) -> Result<(), DomainError> {
        self.is_available = false;
        
        // 全SKUも販売停止
        for sku in &mut self.skus {
            sku.discontinue();
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_as_best_seller(&mut self) {
        self.is_best_seller = true;
        self.updated_at = Utc::now();
    }

    pub fn unmark_as_best_seller(&mut self) {
        self.is_best_seller = false;
        self.updated_at = Utc::now();
    }

    pub fn enable_quick_ship(&mut self) {
        self.is_quick_ship = true;
        self.updated_at = Utc::now();
    }

    pub fn disable_quick_ship(&mut self) {
        self.is_quick_ship = false;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, description: Description) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn category_id(&self) -> &CategoryId {
        &self.category_id
    }

    pub fn is_active(&self) -> bool {
        self.is_available
    }

    pub fn is_best_seller(&self) -> bool {
        self.is_best_seller
    }

    pub fn is_quick_ship(&self) -> bool {
        self.is_quick_ship
    }

    pub fn skus(&self) -> &[SKU] {
        &self.skus
    }

    pub fn images(&self) -> &[ProductImage] {
        &self.images
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
