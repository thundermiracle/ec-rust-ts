use super::{CustomerInfo, OrderItem, OrderPricing, PaymentInfo, ShippingInfo};
use crate::domain::entities::DeliveryInfo;
use crate::domain::error::DomainError;
use crate::domain::value_objects::*;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,    // Order created, awaiting payment
    Paid,       // Payment confirmed
    Processing, // Order being prepared
    Shipped,    // Order dispatched
    Delivered,  // Order delivered
    Cancelled,  // Order cancelled
    Refunded,   // Order refunded
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Paid => write!(f, "paid"),
            OrderStatus::Processing => write!(f, "processing"),
            OrderStatus::Shipped => write!(f, "shipped"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Refunded => write!(f, "refunded"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = crate::domain::error::DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(OrderStatus::Pending),
            "paid" => Ok(OrderStatus::Paid),
            "processing" => Ok(OrderStatus::Processing),
            "shipped" => Ok(OrderStatus::Shipped),
            "delivered" => Ok(OrderStatus::Delivered),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "refunded" => Ok(OrderStatus::Refunded),
            _ => Err(crate::domain::error::DomainError::InvalidProductData(
                format!("Invalid order status: {}", s),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderTimestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub order_number: OrderNumber,
    pub customer_info: CustomerInfo,
    pub items: Vec<OrderItem>,
    pub shipping_info: ShippingInfo,
    pub payment_info: PaymentInfo,
    pub pricing: OrderPricing,
    pub status: OrderStatus,
    pub timestamps: OrderTimestamps,
    pub delivery_info: Option<DeliveryInfo>,
    pub notes: Option<String>,
}

impl Order {
    pub fn new(
        order_number: OrderNumber,
        customer_info: CustomerInfo,
        items: Vec<OrderItem>,
        shipping_info: ShippingInfo,
        payment_info: PaymentInfo,
    ) -> Result<Self, DomainError> {
        if items.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Order must have at least one item".to_string(),
            ));
        }

        let pricing = Self::calculate_pricing(&items, &shipping_info, &payment_info)?;
        let now = Utc::now();

        Ok(Order {
            id: OrderId::new(),
            order_number,
            customer_info,
            items,
            shipping_info,
            payment_info,
            pricing,
            status: OrderStatus::Pending,
            timestamps: OrderTimestamps {
                created_at: now,
                updated_at: now,
                paid_at: None,
                shipped_at: None,
                delivered_at: None,
                cancelled_at: None,
            },
            delivery_info: None,
            notes: None,
        })
    }

    pub fn update_status(&mut self, new_status: OrderStatus) -> Result<(), DomainError> {
        self.validate_status_transition(&new_status)?;

        let now = Utc::now();
        self.status = new_status.clone();
        self.timestamps.updated_at = now;

        // Update relevant timestamps
        match new_status {
            OrderStatus::Paid => self.timestamps.paid_at = Some(now),
            OrderStatus::Shipped => self.timestamps.shipped_at = Some(now),
            OrderStatus::Delivered => self.timestamps.delivered_at = Some(now),
            OrderStatus::Cancelled => self.timestamps.cancelled_at = Some(now),
            _ => {}
        }

        Ok(())
    }

    pub fn add_delivery_info(&mut self, delivery_info: DeliveryInfo) -> Result<(), DomainError> {
        if !matches!(self.status, OrderStatus::Paid | OrderStatus::Processing) {
            return Err(DomainError::InvalidProductData(format!(
                "Cannot add delivery info to order in status {:?}",
                self.status
            )));
        }

        self.delivery_info = Some(delivery_info);
        self.timestamps.updated_at = Utc::now();
        Ok(())
    }

    pub fn cancel(&mut self, reason: String) -> Result<(), DomainError> {
        if matches!(
            self.status,
            OrderStatus::Delivered | OrderStatus::Cancelled | OrderStatus::Refunded
        ) {
            return Err(DomainError::InvalidProductData(format!(
                "Cannot cancel order in status {:?}",
                self.status
            )));
        }

        self.status = OrderStatus::Cancelled;
        self.notes = Some(reason);
        self.timestamps.cancelled_at = Some(Utc::now());
        self.timestamps.updated_at = Utc::now();

        Ok(())
    }

    pub fn add_note(&mut self, note: String) -> Result<(), DomainError> {
        if note.len() > 1000 {
            return Err(DomainError::InvalidProductData(
                "Note cannot exceed 1000 characters".to_string(),
            ));
        }

        self.notes = Some(note);
        self.timestamps.updated_at = Utc::now();
        Ok(())
    }

    fn validate_status_transition(&self, new_status: &OrderStatus) -> Result<(), DomainError> {
        use OrderStatus::*;

        let valid = match (&self.status, new_status) {
            (Pending, Paid) => true,
            (Pending, Cancelled) => true,
            (Paid, Processing) => true,
            (Paid, Cancelled) => true,
            (Processing, Shipped) => true,
            (Processing, Cancelled) => true,
            (Shipped, Delivered) => true,
            (_, Refunded) => matches!(self.status, Paid | Processing | Shipped | Delivered),
            _ => false,
        };

        if !valid {
            return Err(DomainError::InvalidProductData(format!(
                "Invalid status transition from {:?} to {:?}",
                self.status, new_status
            )));
        }

        Ok(())
    }

    fn calculate_pricing(
        items: &[OrderItem],
        shipping_info: &ShippingInfo,
        payment_info: &PaymentInfo,
    ) -> Result<OrderPricing, DomainError> {
        let mut subtotal = Money::zero();
        for item in items {
            subtotal = subtotal.add(item.subtotal()?)?;
        }

        let total_before_tax = subtotal.add(shipping_info.fee)?.add(payment_info.fee)?;
        let tax_amount = total_before_tax.tax_amount();
        let total = total_before_tax.add(tax_amount)?;

        Ok(OrderPricing {
            subtotal,
            shipping_fee: shipping_info.fee,
            payment_fee: payment_info.fee,
            tax_amount,
            total,
        })
    }

    pub fn can_be_cancelled(&self) -> bool {
        !matches!(
            self.status,
            OrderStatus::Delivered | OrderStatus::Cancelled | OrderStatus::Refunded
        )
    }

    pub fn can_be_modified(&self) -> bool {
        matches!(self.status, OrderStatus::Pending)
    }

    pub fn total_item_count(&self) -> i32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::*;

    fn create_test_order() -> Order {
        let order_number = OrderNumber::generate(2024, 1);
        let customer_info = CustomerInfo::new(
            PersonalInfo::new(
                FirstName::new("太郎".to_string()).unwrap(),
                LastName::new("田中".to_string()).unwrap(),
            ),
            Email::new("test@example.com".to_string()).unwrap(),
            PhoneNumber::new("090-1234-5678".to_string()).unwrap(),
        );

        let items = vec![
            OrderItem::new(
                SKUId::new(),
                SKUCode::new("TEST-001".to_string()).unwrap(),
                ProductName::new("Test Product".to_string()).unwrap(),
                SKUName::new("Test SKU".to_string()).unwrap(),
                Money::from_yen(1000),
                2,
            )
            .unwrap(),
        ];

        let shipping_info = ShippingInfo::new(
            ShippingMethodId::new("standard".to_string()).unwrap(),
            "Standard Shipping".to_string(),
            Money::from_yen(500),
            Address::new(
                "123-4567".to_string(),
                "Tokyo".to_string(),
                "Shibuya".to_string(),
                "1-2-3".to_string(),
                None,
            )
            .unwrap(),
        );

        let payment_info = PaymentInfo::new(
            PaymentMethodId::new(),
            "Credit Card".to_string(),
            Money::from_yen(100),
            None,
        );

        Order::new(
            order_number,
            customer_info,
            items,
            shipping_info,
            payment_info,
        )
        .unwrap()
    }

    #[test]
    fn test_create_order() {
        let order = create_test_order();
        assert_eq!(order.status, OrderStatus::Pending);
        assert_eq!(order.items.len(), 1);
        assert!(order.delivery_info.is_none());
    }

    #[test]
    fn test_status_transitions() {
        let mut order = create_test_order();

        // Pending -> Paid
        assert!(order.update_status(OrderStatus::Paid).is_ok());
        assert_eq!(order.status, OrderStatus::Paid);
        assert!(order.timestamps.paid_at.is_some());

        // Paid -> Processing
        assert!(order.update_status(OrderStatus::Processing).is_ok());
        assert_eq!(order.status, OrderStatus::Processing);

        // Processing -> Shipped
        assert!(order.update_status(OrderStatus::Shipped).is_ok());
        assert_eq!(order.status, OrderStatus::Shipped);
        assert!(order.timestamps.shipped_at.is_some());

        // Shipped -> Delivered
        assert!(order.update_status(OrderStatus::Delivered).is_ok());
        assert_eq!(order.status, OrderStatus::Delivered);
        assert!(order.timestamps.delivered_at.is_some());
    }

    #[test]
    fn test_invalid_status_transitions() {
        let mut order = create_test_order();

        // Cannot go from Pending to Delivered
        assert!(order.update_status(OrderStatus::Delivered).is_err());

        // Cannot go from Pending to Shipped
        assert!(order.update_status(OrderStatus::Shipped).is_err());
    }

    #[test]
    fn test_cancel_order() {
        let mut order = create_test_order();

        assert!(order.cancel("Customer request".to_string()).is_ok());
        assert_eq!(order.status, OrderStatus::Cancelled);
        assert!(order.timestamps.cancelled_at.is_some());
        assert_eq!(order.notes, Some("Customer request".to_string()));
    }

    #[test]
    fn test_cannot_cancel_delivered_order() {
        let mut order = create_test_order();
        order.status = OrderStatus::Delivered;

        assert!(order.cancel("Too late".to_string()).is_err());
    }

    #[test]
    fn test_empty_items_error() {
        let order_number = OrderNumber::generate(2024, 1);
        let customer_info = CustomerInfo::new(
            PersonalInfo::new(
                FirstName::new("太郎".to_string()).unwrap(),
                LastName::new("田中".to_string()).unwrap(),
            ),
            Email::new("test@example.com".to_string()).unwrap(),
            PhoneNumber::new("090-1234-5678".to_string()).unwrap(),
        );

        let items = vec![];

        let shipping_info = ShippingInfo::new(
            ShippingMethodId::new("standard".to_string()).unwrap(),
            "Standard Shipping".to_string(),
            Money::from_yen(500),
            Address::new(
                "123-4567".to_string(),
                "Tokyo".to_string(),
                "Shibuya".to_string(),
                "1-2-3".to_string(),
                None,
            )
            .unwrap(),
        );

        let payment_info = PaymentInfo::new(
            PaymentMethodId::new(),
            "Credit Card".to_string(),
            Money::from_yen(100),
            None,
        );

        let result = Order::new(
            order_number,
            customer_info,
            items,
            shipping_info,
            payment_info,
        );
        assert!(result.is_err());
    }
}
