use crate::domain::value_objects::{
    Email, PersonalInfo, Address, PhoneNumber, DeliveryInfoId
};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct DeliveryInfo {
    // 識別子
    pub id: DeliveryInfoId,
    
    // 基本配送情報（不変）
    pub email: Email,
    pub personal_info: PersonalInfo,
    pub address: Address,
    pub phone_number: PhoneNumber,
    
    // 配送状態（可変）
    pub status: DeliveryStatus,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,
    pub shipping_method: Option<String>,
    
    // タイムスタンプ
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    Processing,
    Shipped,
    InTransit,
    Delivered,
    Failed,
}

#[derive(Debug, PartialEq)]
pub enum DeliveryInfoError {
    InvalidStatusTransition { from: DeliveryStatus, to: DeliveryStatus },
    MissingTrackingInfo,
    AlreadyDelivered,
    InvalidData(String),
}

impl DeliveryInfo {
    pub fn new(
        email: Email,
        personal_info: PersonalInfo,
        address: Address,
        phone_number: PhoneNumber,
        shipping_method: Option<String>,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: DeliveryInfoId::new(),
            email,
            personal_info,
            address,
            phone_number,
            status: DeliveryStatus::Pending,
            carrier: None,
            tracking_number: None,
            shipping_method,
            created_at: now,
            updated_at: now,
            shipped_at: None,
            delivered_at: None,
        }
    }

    pub fn with_id(
        id: DeliveryInfoId,
        email: Email,
        personal_info: PersonalInfo,
        address: Address,
        phone_number: PhoneNumber,
        status: DeliveryStatus,
        carrier: Option<String>,
        tracking_number: Option<String>,
        shipping_method: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        shipped_at: Option<DateTime<Utc>>,
        delivered_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            email,
            personal_info,
            address,
            phone_number,
            status,
            carrier,
            tracking_number,
            shipping_method,
            created_at,
            updated_at,
            shipped_at,
            delivered_at,
        }
    }

    pub fn update_status(&mut self, new_status: DeliveryStatus) -> Result<(), DeliveryInfoError> {
        if !self.is_valid_status_transition(&new_status) {
            return Err(DeliveryInfoError::InvalidStatusTransition {
                from: self.status.clone(),
                to: new_status,
            });
        }

        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn set_tracking_info(
        &mut self,
        carrier: String,
        tracking_number: String,
    ) -> Result<(), DeliveryInfoError> {
        if matches!(self.status, DeliveryStatus::Delivered) {
            return Err(DeliveryInfoError::AlreadyDelivered);
        }

        self.carrier = Some(carrier);
        self.tracking_number = Some(tracking_number);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_as_shipped(&mut self) -> Result<(), DeliveryInfoError> {
        if self.carrier.is_none() || self.tracking_number.is_none() {
            return Err(DeliveryInfoError::MissingTrackingInfo);
        }

        self.update_status(DeliveryStatus::Shipped)?;
        self.shipped_at = Some(Utc::now());
        Ok(())
    }

    pub fn mark_as_in_transit(&mut self) -> Result<(), DeliveryInfoError> {
        self.update_status(DeliveryStatus::InTransit)
    }

    pub fn mark_as_delivered(&mut self) -> Result<(), DeliveryInfoError> {
        self.update_status(DeliveryStatus::Delivered)?;
        self.delivered_at = Some(Utc::now());
        Ok(())
    }

    pub fn mark_as_failed(&mut self) -> Result<(), DeliveryInfoError> {
        self.update_status(DeliveryStatus::Failed)
    }

    fn is_valid_status_transition(&self, new_status: &DeliveryStatus) -> bool {
        use DeliveryStatus::*;
        
        match (&self.status, new_status) {
            (Pending, Processing) => true,
            (Processing, Shipped) => true,
            (Shipped, InTransit) => true,
            (InTransit, Delivered) => true,
            (_, Failed) => true, // 任意の状態からFailedに遷移可能
            _ => false,
        }
    }

    // Getters
    pub fn id(&self) -> &DeliveryInfoId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn personal_info(&self) -> &PersonalInfo {
        &self.personal_info
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }

    pub fn status(&self) -> &DeliveryStatus {
        &self.status
    }

    pub fn carrier(&self) -> Option<&str> {
        self.carrier.as_deref()
    }

    pub fn tracking_number(&self) -> Option<&str> {
        self.tracking_number.as_deref()
    }

    pub fn shipping_method(&self) -> Option<&str> {
        self.shipping_method.as_deref()
    }

    pub fn is_delivered(&self) -> bool {
        matches!(self.status, DeliveryStatus::Delivered)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status, DeliveryStatus::Processing | DeliveryStatus::Shipped | DeliveryStatus::InTransit)
    }
}

impl fmt::Display for DeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryStatus::Pending => write!(f, "配送準備中"),
            DeliveryStatus::Processing => write!(f, "処理中"),
            DeliveryStatus::Shipped => write!(f, "発送済み"),
            DeliveryStatus::InTransit => write!(f, "配送中"),
            DeliveryStatus::Delivered => write!(f, "配送完了"),
            DeliveryStatus::Failed => write!(f, "配送失敗"),
        }
    }
}

impl fmt::Display for DeliveryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "配送情報 [{}]\n名前: {}\nメール: {}\n住所: {}\n電話番号: {}\n状態: {}",
            self.id,
            self.personal_info,
            self.email.value(),
            self.address,
            self.phone_number,
            self.status
        )?;

        if let Some(carrier) = &self.carrier {
            write!(f, "\n配送業者: {}", carrier)?;
        }

        if let Some(tracking) = &self.tracking_number {
            write!(f, "\n追跡番号: {}", tracking)?;
        }

        if let Some(method) = &self.shipping_method {
            write!(f, "\n配送方法: {}", method)?;
        }

        Ok(())
    }
}

impl fmt::Display for DeliveryInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryInfoError::InvalidStatusTransition { from, to } => {
                write!(f, "無効な状態遷移: {} から {} への変更はできません", from, to)
            }
            DeliveryInfoError::MissingTrackingInfo => {
                write!(f, "配送業者と追跡番号が設定されていません")
            }
            DeliveryInfoError::AlreadyDelivered => {
                write!(f, "配送が既に完了しています")
            }
            DeliveryInfoError::InvalidData(msg) => {
                write!(f, "無効なデータ: {}", msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Email, PersonalInfo, Address, PhoneNumber};

    fn create_test_delivery_info() -> DeliveryInfo {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let personal_info = PersonalInfo::from_strings("太郎".to_string(), "田中".to_string()).unwrap();
        let address = Address::new(
            "123-4567".to_string(),
            "東京都".to_string(),
            "渋谷区".to_string(),
            "渋谷1-1-1".to_string(),
            None,
        ).unwrap();
        let phone_number = PhoneNumber::new("03-1234-5678".to_string()).unwrap();

        DeliveryInfo::new(email, personal_info, address, phone_number, Some("宅配便".to_string()))
    }

    #[test]
    fn test_new_delivery_info() {
        let delivery_info = create_test_delivery_info();
        assert_eq!(delivery_info.status, DeliveryStatus::Pending);
        assert!(delivery_info.carrier.is_none());
        assert!(delivery_info.tracking_number.is_none());
    }

    #[test]
    fn test_status_transitions() {
        let mut delivery_info = create_test_delivery_info();
        
        // Pending -> Processing
        assert!(delivery_info.update_status(DeliveryStatus::Processing).is_ok());
        assert_eq!(delivery_info.status, DeliveryStatus::Processing);
        
        // Processing -> Shipped (should fail without tracking info)
        assert!(delivery_info.mark_as_shipped().is_err());
        
        // Set tracking info and try again
        delivery_info.set_tracking_info("ヤマト運輸".to_string(), "1234567890".to_string()).unwrap();
        assert!(delivery_info.mark_as_shipped().is_ok());
        assert_eq!(delivery_info.status, DeliveryStatus::Shipped);
        
        // Shipped -> InTransit -> Delivered
        assert!(delivery_info.mark_as_in_transit().is_ok());
        assert!(delivery_info.mark_as_delivered().is_ok());
        assert_eq!(delivery_info.status, DeliveryStatus::Delivered);
        assert!(delivery_info.delivered_at.is_some());
    }

    #[test]
    fn test_invalid_status_transition() {
        let mut delivery_info = create_test_delivery_info();
        
        // Pending -> Delivered (invalid)
        let result = delivery_info.update_status(DeliveryStatus::Delivered);
        assert!(result.is_err());
        assert_eq!(delivery_info.status, DeliveryStatus::Pending);
    }

    #[test]
    fn test_mark_as_failed() {
        let mut delivery_info = create_test_delivery_info();
        
        // Can fail from any status
        assert!(delivery_info.mark_as_failed().is_ok());
        assert_eq!(delivery_info.status, DeliveryStatus::Failed);
    }
}