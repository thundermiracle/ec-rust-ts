use crate::domain::value_objects::{PersonalInfo, Email, PhoneNumber};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerInfo {
    pub personal_info: PersonalInfo,
    pub email: Email,
    pub phone: PhoneNumber,
}

impl CustomerInfo {
    pub fn new(
        personal_info: PersonalInfo,
        email: Email,
        phone: PhoneNumber,
    ) -> Self {
        CustomerInfo {
            personal_info,
            email,
            phone,
        }
    }
    
    pub fn full_name(&self) -> String {
        self.personal_info.full_name()
    }
    
    pub fn email_address(&self) -> &str {
        self.email.value()
    }
    
    pub fn phone_number(&self) -> &str {
        self.phone.value()
    }
    
    pub fn first_name(&self) -> &str {
        self.personal_info.first_name().value()
    }
    
    pub fn last_name(&self) -> &str {
        self.personal_info.last_name().value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{FirstName, LastName};

    #[test]
    fn test_customer_info() {
        let personal_info = PersonalInfo::new(
            FirstName::new("太郎".to_string()).unwrap(),
            LastName::new("田中".to_string()).unwrap(),
        );
        let email = Email::new("tanaka@example.com".to_string()).unwrap();
        let phone = PhoneNumber::new("090-1234-5678".to_string()).unwrap();
        
        let customer_info = CustomerInfo::new(personal_info, email, phone);
        
        assert_eq!(customer_info.full_name(), "太郎 田中");
        assert_eq!(customer_info.email_address(), "tanaka@example.com");
        assert_eq!(customer_info.phone_number(), "090-1234-5678");
        assert_eq!(customer_info.first_name(), "太郎");
        assert_eq!(customer_info.last_name(), "田中");
    }
}