use crate::bank::models::{
    accounts::Account,
    customer::{self, Customer},
    types::{AccountType, CardStatus, CardType, Status},
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomerData {
    pub id: Uuid,
    pub customer_name: String,
    pub email: String,
    pub phone_number: String,
    pub cic_number: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomerBody {
    pub customer: CustomerData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountData {
    pub id: Uuid,
    pub account_number: String,
    pub balance: i32,
    pub opened_date: NaiveDate,
    pub last_updated_date: NaiveDate,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub account_type: AccountType,
    pub customer: customer::Customer,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountBody {
    pub customer: AccountData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BeneficiaryData {
    pub id: Uuid,
    pub customer: Customer,
    pub beneficiary_name: String,
    pub beneficiary_account_number: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BeneficiaryBody {
    pub beneficiary: BeneficiaryData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CardData {
    pub id: Uuid,
    pub card_number: String,
    pub account_id: Uuid,
    pub expiration_date: NaiveDate,
    pub cvv: String,
    pub issued_date: NaiveDate,
    pub balance: i32,
    pub card_status: CardStatus,
    pub card_type: CardType,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CardBody {
    pub card: CardData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadData {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub refund_amount: i32,
    pub refund_date: NaiveDate,
    pub status: Status,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoanBody {
    pub load: LoadData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransferData {
    pub id: Uuid,
    pub sender_card_number: String,
    pub beneficiary_account_number: String,
    pub amount: i32,
    pub transfer_date: NaiveDate,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransferBody {
    pub transfer: TransferData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionData {
    pub id: Uuid,
    pub account_number: String,
    pub transaction_type: String,
    pub card_number: String,
    pub amount: i32,
    pub transaction_date: NaiveDate,
    pub status: Status,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RefundBody {
    pub transaction: TransactionData,
}
