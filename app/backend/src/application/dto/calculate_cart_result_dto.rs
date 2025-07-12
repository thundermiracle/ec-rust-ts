use crate::domain::{Cart, Money};

/// カートアイテム計算結果DTO
#[derive(Debug, Clone)]
pub struct CalculatedCartItemDto {
    pub sku_id: String,
    pub product_id: String,
    pub product_name: String,
    pub unit_price: Money,
    pub quantity: u32,
    pub subtotal: Money,
}

/// カート計算結果DTO
/// CQRS命名規則: CalculateCartCommand の結果
/// すべての計算済みの値を含む
#[derive(Debug, Clone)]
pub struct CalculateCartResultDto {
    pub items: Vec<CalculatedCartItemDto>,
    pub total_quantity: u32,
    pub item_count: usize,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total_with_tax: Money,
    pub is_empty: bool,
    pub shipping_fee: Money,
    pub payment_fee: Money,
}

impl CalculateCartResultDto {
    pub fn from_cart(cart: Cart, shipping_fee: Money, payment_fee: Money) -> Result<Self, String> {
        // カートアイテムの計算
        let mut items = Vec::new();
        for item in cart.items() {
            let subtotal = item.subtotal()
                .map_err(|e| format!("Failed to calculate item subtotal: {}", e))?;
            
            items.push(CalculatedCartItemDto {
                sku_id: item.sku_id().to_string(),
                product_id: item.product_id().to_string(),
                product_name: item.product_name().value().to_string(),
                unit_price: item.unit_price(),
                quantity: item.quantity(),
                subtotal,
            });
        }

        // カート全体の計算
        let subtotal = cart.subtotal()
            .map_err(|e| format!("Failed to calculate cart subtotal: {}", e))?;
        
        let tax_amount = cart.tax_amount()
            .map_err(|e| format!("Failed to calculate tax amount: {}", e))?;
        
        let total_with_tax = cart.total_with_tax()
            .map_err(|e| format!("Failed to calculate total with tax: {}", e))?;

        Ok(Self {
            items,
            total_quantity: cart.total_quantity(),
            item_count: cart.item_count(),
            subtotal,
            tax_amount,
            total_with_tax,
            is_empty: cart.is_empty(),
            shipping_fee,
            payment_fee,
        })
    }
}