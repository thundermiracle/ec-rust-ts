use crate::domain::{ColorId, Dimensions, Material};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantAttributes {
    pub color_id: Option<ColorId>,
    pub dimensions: Option<Dimensions>,
    pub material: Option<Material>,
}

impl VariantAttributes {
    pub fn new() -> Self {
        Self {
            color_id: None,
            dimensions: None,
            material: None,
        }
    }

    // Builder pattern
    pub fn with_color_id(mut self, color_id: ColorId) -> Self {
        self.color_id = Some(color_id);
        self
    }

    pub fn with_dimensions(mut self, dimensions: Dimensions) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    // Setters
    pub fn set_color_id(&mut self, color_id: Option<ColorId>) {
        self.color_id = color_id;
    }

    pub fn set_dimensions(&mut self, dimensions: Option<Dimensions>) {
        self.dimensions = dimensions;
    }

    pub fn set_material(&mut self, material: Option<Material>) {
        self.material = material;
    }

    // Business logic
    pub fn has_any_attributes(&self) -> bool {
        self.color_id.is_some() || self.dimensions.is_some() || self.material.is_some()
    }

    pub fn display_name(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(dimensions) = &self.dimensions {
            parts.push(dimensions.value());
        }
        if let Some(material) = &self.material {
            parts.push(material.value());
        }
        
        if parts.is_empty() {
            "Standard".to_string()
        } else {
            parts.join(" ")
        }
    }

    pub fn generate_sku_suffix(&self) -> String {
        if !self.has_any_attributes() {
            return "STD".to_string();
        }

        let mut parts = Vec::new();
        
        if let Some(material) = &self.material {
            parts.push(material.abbreviated().to_uppercase());
        }
        
        if parts.is_empty() {
            "STD".to_string()
        } else {
            parts.join("-")
        }
    }

    // Getters
    pub fn color_id(&self) -> Option<&ColorId> {
        self.color_id.as_ref()
    }

    pub fn dimensions(&self) -> Option<&Dimensions> {
        self.dimensions.as_ref()
    }

    pub fn material(&self) -> Option<&Material> {
        self.material.as_ref()
    }
}

impl Default for VariantAttributes {
    fn default() -> Self {
        Self::new()
    }
} 