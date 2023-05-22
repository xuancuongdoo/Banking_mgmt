use chrono::{NaiveDate, NaiveDateTime}
;use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Transfer {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub sender_card_number: String,
    pub beneficiary_account_number: String,
    pub amount: i32,
    pub transfer_date: NaiveDate,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
