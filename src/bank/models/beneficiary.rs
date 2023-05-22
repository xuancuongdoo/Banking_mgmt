use super::customer::Customer;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Beneficiary {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub customer: Customer,
    pub beneficiary_name: String,
    pub beneficiary_account_number: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
