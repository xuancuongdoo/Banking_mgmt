use chrono::{NaiveDate, NaiveDateTime};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::bank::helper::validation::{
    validate_account_balance, validate_account_opened_date, validate_input, CustomerErrorReps,
};

use super::{
    customer::{self, Customer},
    types::AccountType,
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Account {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub account_number: String,
    pub balance: i32,
    pub account_type: AccountType,
    pub customer_id: Uuid,
    pub opened_date: NaiveDate,
    pub last_updated_date: NaiveDate,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn insert_account(
    pool: &PgPool,
    branch_id: Uuid,
    bank_id: Uuid,
    balance: i32,
    opened_date: NaiveDate,
    customer_id: Uuid,
) -> Result<Account, CustomerErrorReps> {
    // Validate input
    let validated_balance =
        validate_input(balance, validate_account_balance)
            .map_err(|validation_result| {
                CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap())
            })?;

    let validated_opened_date =
        validate_input(opened_date, validate_account_opened_date)
            .map_err(|validation_result| {
                CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap())
            })?;

    let account_type = AccountType::Checkings;
    let account_id = Uuid::new_v4();
    let current_timestamp = chrono::Utc::now().naive_utc();
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen_range(0..=999_999_999_99);
    let account_number = format!("0{:011}", random_number);

    let account = sqlx::query_as!(
        Account,
        r#"
        INSERT INTO accounts (branch_id, bank_id, id, account_number, balance, account_type, customer_id, opened_date, last_updated_date, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING account_type as "account_type: _", branch_id, bank_id, id, account_number, balance, customer_id, opened_date, last_updated_date, inserted_at, updated_at
        "#,
        branch_id,
        bank_id,
        account_id,
        account_number,
        validated_balance,
        account_type as AccountType,
        customer_id,
        validated_opened_date,
        current_timestamp,
    )
    .fetch_one(pool)
    .await
    .map_err(CustomerErrorReps::DatabaseError)?;

    Ok(account)
}
