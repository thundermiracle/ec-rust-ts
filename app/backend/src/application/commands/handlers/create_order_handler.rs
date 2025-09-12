use crate::application::commands::models::CreateOrderCommand;
use crate::application::dto::{CreateOrderResultDTO, VariantDTO};
use crate::application::error::ApplicationError;
use crate::application::repositories::{
    CouponRepository, OrderRepository, PaymentMethodRepository, ProductRepository,
    ShippingMethodRepository,
};
use crate::application::services::{ItemFactory, ItemValidationService};
use crate::domain::aggregates::order::{CustomerInfo, Order, OrderItem, PaymentInfo, ShippingInfo};
use crate::domain::order::{CouponInfo, DiscountInfo};
use crate::domain::{CouponDiscountService, value_objects::*};
use chrono::Datelike;
use std::sync::Arc;

/// 注文作成ハンドラ（ユースケース）
pub struct CreateOrderHandler {
    item_validation_service: ItemValidationService,
    shipping_method_repository: Arc<dyn ShippingMethodRepository>,
    payment_method_repository: Arc<dyn PaymentMethodRepository>,
    order_repository: Arc<dyn OrderRepository>,
    coupon_repository: Arc<dyn CouponRepository>,
}

impl CreateOrderHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        shipping_method_repository: Arc<dyn ShippingMethodRepository>,
        payment_method_repository: Arc<dyn PaymentMethodRepository>,
        order_repository: Arc<dyn OrderRepository>,
        coupon_repository: Arc<dyn CouponRepository>,
    ) -> Self {
        let item_validation_service = ItemValidationService::new(product_repository);
        Self {
            item_validation_service,
            shipping_method_repository,
            payment_method_repository,
            order_repository,
            coupon_repository,
        }
    }

    /// 注文作成を実行
    pub async fn handle(
        &self,
        command: CreateOrderCommand,
    ) -> Result<CreateOrderResultDTO, ApplicationError> {
        // 1. 顧客情報の作成
        let customer_info = self.create_customer_info(&command)?;

        // 2. 一度だけ在庫チェックを実行して結果を共有
        let validated_items = self.validate_items_once(&command).await?;

        // 3. バリデーション済みアイテムから注文アイテムを作成
        let order_items = self.create_order_items_from_validated(&command, &validated_items)?;

        // 4. 配送情報の作成
        let shipping_info = self.create_shipping_info(&command).await?;

        // 5. 支払い情報の作成
        let payment_info = self.create_payment_info(&command).await?;

        // 6. バリデーション済みアイテムからクーポン情報を作成
        let discount_info = self
            .create_discount_info_from_validated(&command, &validated_items)
            .await?;

        // 7. 注文番号の生成
        let current_year = chrono::Utc::now().year();
        let sequence_number = self
            .order_repository
            .get_next_sequence_number(current_year)
            .await?;
        let order_number = OrderNumber::generate(current_year, sequence_number);

        // 8. 注文の作成
        let order = Order::new(
            order_number,
            customer_info,
            order_items,
            shipping_info,
            payment_info,
            discount_info,
        )?;

        // 9. 注文の保存
        self.order_repository.save(&order).await?;

        // 10. 結果DTOの作成
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

    /// 一度だけ在庫チェックを実行（重複を避ける）
    async fn validate_items_once(
        &self,
        command: &CreateOrderCommand,
    ) -> Result<Vec<VariantDTO>, ApplicationError> {
        if command.items.is_empty() {
            return Err(ApplicationError::InvalidInput(
                "Order must have at least one item".to_string(),
            ));
        }

        // リクエストデータを準備
        let items: Vec<(String, u32)> = command
            .items
            .iter()
            .map(|item| (item.sku_id.clone(), item.quantity))
            .collect();

        // 共通サービスを使用して検証とバリアント取得
        self.item_validation_service
            .validate_items_with_stock(&items)
            .await
    }

    /// バリデーション済みアイテムから注文アイテムを作成
    fn create_order_items_from_validated(
        &self,
        command: &CreateOrderCommand,
        variants: &[VariantDTO],
    ) -> Result<Vec<OrderItem>, ApplicationError> {
        // 元のコマンドから正しいquantityを取得
        let items: Vec<(String, u32)> = command
            .items
            .iter()
            .map(|item| (item.sku_id.clone(), item.quantity))
            .collect();

        // 共通ファクトリーを使用してOrderItem作成
        ItemFactory::create_order_items(&items, variants)
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

    /// バリデーション済みアイテムからクーポン情報を作成（重複チェックを避ける）
    async fn create_discount_info_from_validated(
        &self,
        command: &CreateOrderCommand,
        variants: &[VariantDTO],
    ) -> Result<Option<DiscountInfo>, ApplicationError> {
        // クーポンコードがない場合は、None を返す
        if command.coupon_code.is_none() {
            return Ok(None);
        }

        // クーポンを取得
        let coupon_code = CouponCode::from_string(command.coupon_code.clone().unwrap())?;
        let coupon = match self.coupon_repository.find_by_code(&coupon_code).await {
            Ok(Some(coupon)) => coupon,
            Ok(None) => {
                return Ok(None);
            }
            Err(e) => {
                return Err(ApplicationError::Domain(e));
            }
        };

        // 元のコマンドから正しいquantityを取得
        let items: Vec<(String, u32)> = command
            .items
            .iter()
            .map(|item| (item.sku_id.clone(), item.quantity))
            .collect();

        // 既に検証済みのバリアントを使用してCartItem作成
        let cart_items = ItemFactory::create_cart_items(&items, variants)?;

        // 小計を計算
        let subtotal = cart_items.iter().fold(Money::zero(), |acc, item| {
            acc.add(item.subtotal().unwrap_or(Money::zero()))
                .unwrap_or(acc)
        });

        let purchase_info = PurchaseInfo::new(cart_items, subtotal, None, None);

        // クーポンを適用して割引計算
        match CouponDiscountService::apply_coupon(&coupon, &purchase_info) {
            Ok(discount_result) => {
                // CouponInfoを作成
                let coupon_info = CouponInfo::new(
                    coupon.id().clone(),
                    coupon.code().clone(),
                    coupon.name().to_string(),
                    coupon.discount_policy().clone(),
                    discount_result.discount_amount,
                );

                // DiscountInfoを生成して返す
                Ok(Some(DiscountInfo::new(vec![coupon_info])))
            }
            Err(domain_error) => {
                // クーポンが適用できない場合はNoneを返す（エラーにしない）
                println!("Coupon application failed: {:?}", domain_error);
                Ok(None)
            }
        }
    }
}
