use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "accounttype", rename_all = "snake_case")]
pub enum AccountType {
    Checkings,
    Savings,
    Credits,
}

#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "cardstatus", rename_all = "snake_case")]
pub enum CardStatus {
    Active,
    Inactive,
    Closed,
}
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "cardtype", rename_all = "snake_case")]
pub enum CardType {
    Debit,
    Credit,
}
#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum Status {
    Pending,
    Approved,
    Rejected,
    Close,
}

#[derive(Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "transactiontype", rename_all = "snake_case")]
pub enum TransactionType {
    RepayLoan,
    RepayInterest,
    P2P,
    CashWithdrawal,
    CashDeposit,
    DebitCardCharge,
}
