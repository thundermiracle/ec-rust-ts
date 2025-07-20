use regex::Regex;
use std::sync::LazyLock;
use validator::ValidationError;

/// 日本の郵便番号バリデーター関数
/// 
/// 以下の形式に対応:
/// - 1234567 (7桁の数字)
/// - 123-4567 (3桁-4桁の形式)
/// 
/// # Examples
/// ```
/// use crate::presentation::validators::validate_japanese_postal_code;
/// 
/// assert!(validate_japanese_postal_code("1234567").is_ok());
/// assert!(validate_japanese_postal_code("123-4567").is_ok());
/// assert!(validate_japanese_postal_code("12345").is_err());
/// ```
pub fn validate_japanese_postal_code(value: &str) -> Result<(), ValidationError> {
    static POSTAL_CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\d{3}-?\d{4}$").unwrap() // ハイフンありなしどちらも対応
    });
    
    if POSTAL_CODE_REGEX.is_match(value) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_postal_code");
        error.message = Some("郵便番号は7桁の数字またはXXX-XXXX形式で入力してください".into());
        Err(error)
    }
}