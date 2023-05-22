use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{Status, TransactionType};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub account_number: String,
    pub transaction_type: TransactionType,
    pub card_number: String,
    pub amount: i32,
    pub transaction_date: NaiveDate,
    pub status: Status,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
