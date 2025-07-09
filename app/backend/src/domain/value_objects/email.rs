use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    value: String,
}

#[derive(Debug, PartialEq)]
pub enum EmailError {
    Empty,
    InvalidFormat,
    TooLong,
}

impl Email {
    pub fn new(value: String) -> Result<Self, EmailError> {
        Self::validate(&value)?;

        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    // メールアドレスのバリデーション
    pub fn validate(value: &str) -> Result<(), EmailError> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }

        if value.len() > 255 {
            return Err(EmailError::TooLong);
        }

        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !re.is_match(value) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(())
    }
}
