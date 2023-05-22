use super::{
    cards::{self, Card},
    types::Status,
};

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Loan {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub amount: i32,
    pub interest_rate: i32,
    pub lender_card: cards::Card,
    pub borrower_card: cards::Card,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: Status,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
