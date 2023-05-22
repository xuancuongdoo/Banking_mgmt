use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, thiserror::Error)]
pub enum CustomerErrorReps  {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
}

// Helper function for validating inputs and returning a result
pub fn validate_input<T, F>(value: T, validator: F) -> Result<T, ValidationResult>
where
    F: FnOnce(&T) -> ValidationResult,
{
    let validation_result = validator(&value);
    if validation_result.is_valid {
        Ok(value)
    } else {
        Err(validation_result)
    }
}
pub fn validate_cic_number(cic_number: &str) -> ValidationResult {
    // Implement CIC number validation logic according to your requirements
    // Example: Checking for a valid CIC number format and length
    if cic_number.len() == 10 && cic_number.chars().all(char::is_numeric) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid CIC number format or length.".to_string()),
        }
    }
}

pub fn validate_customer_name(customer_name: &str) -> ValidationResult {
    if customer_name.trim().is_empty() {
        ValidationResult {
            is_valid: false,
            error_message: Some("Customer name cannot be empty.".to_string()),
        }
    } else if customer_name.len() > 100 {
        ValidationResult {
            is_valid: false,
            error_message: Some("Customer name cannot exceed 100 characters.".to_string()),
        }
    } else {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    }
}

pub fn validate_email(email: &str) -> ValidationResult {
    // Implement email validation logic according to your requirements
    // Example: Checking for a valid email format using regex
    let email_regex =
        regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    if email_regex.is_match(email) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid email format.".to_string()),
        }
    }
}

pub fn validate_phone_number(phone_number: &str) -> ValidationResult {
    // Implement phone number validation logic according to your requirements
    // Example: Checking for a valid phone number format using regex
    let phone_regex = regex::Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
    if phone_regex.is_match(phone_number) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid phone number format.".to_string()),
        }
    }
}

pub fn validate_account_number(account_number: &str) -> ValidationResult {
    // Implement account number validation logic according to your requirements
    // Example: Checking for a valid account number format and length
    if account_number.len() >= 6 && account_number.len() <= 20 {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid account number format or length.".to_string()),
        }
    }
}

pub fn validate_account_balance(balance: i32) -> ValidationResult {
    // Implement account balance validation logic according to your requirements
    // Example: Checking for a non-negative balance
    if balance >= 0 {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Account balance cannot be negative.".to_string()),
        }
    }
}

pub fn validate_card_number(card_number: &str) -> ValidationResult {
    // Implement card number validation logic according to your requirements
    // Example: Checking for a valid card number format and length
    if card_number.len() == 16 && card_number.chars().all(char::is_numeric) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid card number format or length.".to_string()),
        }
    }
}

pub fn validate_cvv(cvv: &str) -> ValidationResult {
    // Implement CVV validation logic according to your requirements
    // Example: Checking for a valid CVV format and length
    if cvv.len() == 3 && cvv.chars().all(char::is_numeric) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid CVV format or length.".to_string()),
        }
    }
}

pub fn validate_card_expiration_date(expiration_date: NaiveDate, issued_date: NaiveDate) -> ValidationResult {
    // Implement card expiration date validation logic according to your requirements
    // Example: Checking if the expiration date is after the issued date

    if expiration_date >= issued_date {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Card has expired.".to_string()),
        }
    }
}

pub fn validate_card_status(card_status: &str) -> ValidationResult {
    // Implement card status validation logic according to your requirements
    // Example: Checking for valid card status values (e.g., "active", "inactive")
    let valid_statuses = vec!["active", "inactive"];
    if valid_statuses.contains(&card_status) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid card status.".to_string()),
        }
    }
}

pub fn validate_card_type(card_type: &str) -> ValidationResult {
    // Implement card type validation logic according to your requirements
    // Example: Checking for valid card type values (e.g., "credit", "debit")
    let valid_types = vec!["credit", "debit"];
    if valid_types.contains(&card_type) {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Invalid card type.".to_string()),
        }
    }
}

pub fn validate_account_opened_date(opened_date: NaiveDateTime) -> ValidationResult {
    // Implement account opened date validation logic according to your requirements
    // Example: Checking if the opened date is in the past
    let current_datetime = chrono::Utc::now().naive_utc();
    if opened_date <= current_datetime {
        ValidationResult {
            is_valid: true,
            error_message: None,
        }
    } else {
        ValidationResult {
            is_valid: false,
            error_message: Some("Account cannot be opened in the future.".to_string()),
        }
    }
}
