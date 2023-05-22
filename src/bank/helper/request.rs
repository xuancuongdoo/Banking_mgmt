use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bank::models::{
    customer::{self, Customer},
    types::{AccountType, Status, CardStatus, CardType}, accounts::Account,
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomerData {
    pub id: Uuid,
    pub customer_name: String,
    pub email: String,
    pub phone_number: String,
    pub cic_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomerBody {
    pub customer: CustomerData,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountData {
    pub id: Uuid,
    pub account_number: String,
    pub account_type: AccountType,
    pub customer: customer::Customer,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountBody {
    pub customer: AccountData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BeneficiaryData {
    id: Uuid,
    customer: Customer,
    beneficiary_name: String,
    beneficiary_account_number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BeneficiaryBody {
    pub beneficiary: BeneficiaryData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CardData {
    id: Uuid,
    card_number: String,
    account: Account,
    expiration_date: NaiveDate,
    cvv: String,
    issued_date: NaiveDate,
    balance: i32,
    card_status: CardStatus,
    card_type: CardType,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RefundBody {
    pub transaction: TransactionData,
}
