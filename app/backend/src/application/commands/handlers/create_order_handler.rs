use crate::application::commands::models::CreateOrderCommand;
use crate::application::dto::CreateOrderResultDTO;
use crate::application::error::ApplicationError;
use crate::application::repositories::{
    OrderRepository, PaymentMethodRepository, ProductRepository, ShippingMethodRepository,
};
use crate::domain::aggregates::order::{CustomerInfo, Order, OrderItem, PaymentInfo, ShippingInfo};
use crate::domain::value_objects::*;
use chrono::Datelike;
use std::sync::Arc;
use uuid::Uuid;

/// 注文作成ハンドラ（ユースケース）
pub struct CreateOrderHandler {
    product_repository: Arc<dyn ProductRepository>,
    shipping_method_repository: Arc<dyn ShippingMethodRepository>,
    payment_method_repository: Arc<dyn PaymentMethodRepository>,
    order_repository: Arc<dyn OrderRepository>,
}

impl CreateOrderHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        shipping_method_repository: Arc<dyn ShippingMethodRepository>,
        payment_method_repository: Arc<dyn PaymentMethodRepository>,
        order_repository: Arc<dyn OrderRepository>,
    ) -> Self {
        Self {
            product_repository,
            shipping_method_repository,
            payment_method_repository,
            order_repository,
        }
    }

    /// 注文作成を実行
    pub async fn handle(
        &self,
        command: CreateOrderCommand,
    ) -> Result<CreateOrderResultDTO, ApplicationError> {
        // 1. 顧客情報の作成
        let customer_info = self.create_customer_info(&command)?;

        // 2. 注文アイテムの作成
        let order_items = self.create_order_items(&command).await?;

        // 3. 配送情報の作成
        let shipping_info = self.create_shipping_info(&command).await?;

        // 4. 支払い情報の作成
        let payment_info = self.create_payment_info(&command).await?;

        // 5. 注文番号の生成
        let current_year = chrono::Utc::now().year();
        let sequence_number = self.order_repository
            .get_next_sequence_number(current_year)
            .await
            .map_err(ApplicationError::Repository)?;
        let order_number = OrderNumber::generate(current_year, sequence_number);

        // 6. 注文の作成
        let order = Order::new(
            order_number,
            customer_info,
            order_items,
            shipping_info,
            payment_info,
        )
        .map_err(ApplicationError::Domain)?;

        // 7. 注文の保存
        self.order_repository
            .save(&order)
            .await
            .map_err(ApplicationError::Repository)?;

        // 8. 結果DTOの作成
        Ok(CreateOrderResultDTO::from_order(&order))
    }

    fn create_customer_info(
        &self,
        command: &CreateOrderCommand,
    ) -> Result<CustomerInfo, ApplicationError> {
        let first_name = FirstName::new(command.customer_info.first_name.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        let last_name = LastName::new(command.customer_info.last_name.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        let personal_info = PersonalInfo::new(first_name, last_name);

        let email = Email::new(command.customer_info.email.clone())
            .map_err(|e| ApplicationError::InvalidInput(format!("{:?}", e)))?;

        let phone = PhoneNumber::new(command.customer_info.phone.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        Ok(CustomerInfo::new(personal_info, email, phone))
    }

    async fn create_order_items(
        &self,
        command: &CreateOrderCommand,
    ) -> Result<Vec<OrderItem>, ApplicationError> {
        if command.items.is_empty() {
            return Err(ApplicationError::InvalidInput(
                "Order must have at least one item".to_string(),
            ));
        }

        // SKU IDの抽出
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

        // SKU情報の取得
        let variants = self
            .product_repository
            .find_variants_by_ids(&sku_ids)
            .await
            .map_err(ApplicationError::Repository)?;

        // 注文アイテムの作成
        let mut order_items = Vec::new();
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
                .ok_or_else(|| {
                    ApplicationError::NotFound(format!("SKU not found: {}", item_request.sku_id))
                })?;

            // 在庫チェック
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

            // OrderItemの作成
            let sku_id =
                SKUId::from_uuid(Uuid::parse_str(&variant.id).map_err(|_| {
                    ApplicationError::InvalidInput("Invalid variant ID".to_string())
                })?);

            let sku_code = SKUCode::new(variant.sku_code.clone())
                .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            let product_name = ProductName::new(variant.name.clone())
                .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            let sku_name = SKUName::new(variant.name.clone())
                .map_err(|e| ApplicationError::InvalidInput(format!("{:?}", e)))?;

            let unit_price = Money::from_yen(variant.sale_price.unwrap_or(variant.price));

            let order_item = OrderItem::new(
                sku_id,
                sku_code,
                product_name,
                sku_name,
                unit_price,
                item_request.quantity as i32,
            )
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

            order_items.push(order_item);
        }

        Ok(order_items)
    }

    async fn create_shipping_info(
        &self,
        command: &CreateOrderCommand,
    ) -> Result<ShippingInfo, ApplicationError> {
        // 配送方法の取得
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

        // 配送先住所の作成
        let address = Address::new(
            command.shipping_address.postal_code.clone(),
            command.shipping_address.prefecture.clone(),
            command.shipping_address.city.clone(),
            command.shipping_address.street_address.clone(),
            command.shipping_address.building.clone(),
        )
        .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        // ShippingMethodIdの作成
        let shipping_method_id = ShippingMethodId::new(command.shipping_method_id.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;

        let shipping_fee = *shipping_method.price();

        Ok(ShippingInfo::new(
            shipping_method_id,
            shipping_method.name().to_string(),
            shipping_fee,
            address,
        ))
    }

    async fn create_payment_info(
        &self,
        command: &CreateOrderCommand,
    ) -> Result<PaymentInfo, ApplicationError> {
        // 支払い方法の取得
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

        let payment_method_id = PaymentMethodId::new(command.payment_method_id.clone())
            .map_err(|e| ApplicationError::InvalidInput(e.to_string()))?;
        let payment_fee = Money::from_yen(0); // TODO: PaymentMethodDTOにfeeフィールドを追加

        Ok(PaymentInfo::new(
            payment_method_id,
            payment_method.name().to_string(),
            payment_fee,
            None,
        ))
    }
}
