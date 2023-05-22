use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use crate::bank::cards::{CardStatus, Card};

#[macro_export]
macro_rules! validate {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
        else {
            Ok(())
        }
    };
}

pub fn has_sufficient_balance(card_balance: i32, transaction_amount: i32) -> bool {
    card_balance >= transaction_amount
}

pub async fn card_belongs_to_account(
    pool: &PgPool,
    card_number: &String,
    account_id: &Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM cards
                WHERE card_number = $1 AND account_id = $2
            ) as exists
        "#,
        card_number,
        account_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.exists.unwrap_or(false))
}

pub fn is_valid_card_type(card_type: &str) -> bool {
    match card_type.to_lowercase().as_str() {
        "debit" | "credit" => true,
        _ => false,
    }
}

pub fn is_card_not_expired(expiration_date: &NaiveDate) -> bool {
    let current_date = Utc::now().date().naive_utc();
    expiration_date > &current_date
}

pub fn is_card_active(card_status: CardStatus) -> bool {
    card_status == CardStatus::Active
}

pub fn is_valid_card_number(card_number: &str) -> bool {
    let digits = card_number
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    if digits.len() < 13 {
        return false;
    }

    let sum = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d } else { d * 2 % 9 })
        .sum::<u64>();

    sum % 10 == 0
}

