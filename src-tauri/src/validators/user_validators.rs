use once_cell::sync::Lazy;
use regex::Regex;
use validator::{Validate, ValidationError, ValidationErrors};

pub static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\+\d{1,3})?-\d{6,14}$").unwrap());

pub static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&_-]{10,20}$").unwrap());

pub fn validate_no_spaces(username: &str) -> Result<(), ValidationError> {
    if username.contains(' ') {
        let mut error = ValidationError::new("no_spaces");
        error.message = Some("Username cannot contain spaces".into());
        return Err(error);
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if !PASSWORD_REGEX.is_match(password) {
        let mut error = ValidationError::new("invalid_password_format");
        error.message = Some("Password must be 10-20 characters and must contain lower and upper letters, numbers, and special characters (@$!%*?&)".into());
        return Err(error);
    }

    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "@$!%*?&_-".contains(c));

    if !has_lowercase || !has_uppercase || !has_digit || !has_special {
        let mut error = ValidationError::new("invalid_password_requirement");
        error.message = Some("Password must include at least one uppercase letter, one lowercase letter, one number, and one special character (@$!%*?&)".into());
        return Err(error);
    }

    Ok(())
}

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        let mut error = ValidationError::new("invalid_phone");
        error.message = Some("Phone number must be in the format +123-1234567890".into());
        Err(error)
    }
}
