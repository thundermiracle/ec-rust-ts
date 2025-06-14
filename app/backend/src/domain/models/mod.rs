mod product;
mod category;
mod color;
mod product_image;
mod product_variant;
mod tag;
mod inventory;

pub use self::product::Product;
pub use self::category::{Category, CategoryId};
pub use self::color::{Color, ColorName};
pub use self::product_image::{ProductImage, ProductImageId, ProductImageProductId, ImageUrl};
pub use self::product_variant::{ProductVariant, ProductVariantId, ProductVariantProductId};
pub use self::tag::{Tag, TagName};
pub use self::inventory::{Inventory, InventoryId, InventoryProductId, InventoryVariantId};
