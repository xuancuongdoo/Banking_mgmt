use axum::extract::State;
use axum::Json;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bank::accounts::{self, Account, AccountService, BankService, Hold};
use crate::bank_web::payments::{InvalidData, InvalidDataResponse};

use super::payments::create_invalid_data_response;
use super::BankWeb;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HoldRequestBody {
    pub hold: HoldRequestData,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HoldResponseBody {
    pub hold: Hold,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HoldRequestData {
    pub account_number: String,
    pub card_id: Uuid,
    pub amount: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HoldResponseData {
    pub hold: Hold,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountRequestBody {
    pub account: AccountRequestData,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountRequestData {
    pub cic_number: String,
    pub account_number: String,
    pub balance: i32,
    pub account_type: accounts::AccountType,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountResponseData {
    pub id: Uuid,
    pub cic_number: String,
    pub account_number: String,
    pub balance: i32,
    pub account_type: accounts::AccountType,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountResponseBody {
    pub data: AccountResponseData,
}

/// POST ACCOUNT
pub async fn create_account<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Json(body): Json<AccountRequestBody>,
) -> (
    StatusCode,
    Json<Result<AccountResponseBody, InvalidDataResponse<AccountResponseData>>>,
) {
    let existing_account = accounts::check_cic_exist(
        &bank_web.pool,
        &body.account.cic_number,
        &body.account.account_number,
    )
    .await
    .unwrap();

    if let Some(account) = existing_account {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(Ok(AccountResponseBody {
                data: AccountResponseData {
                    id: account.id,
                    cic_number: account.cic_number,
                    account_number: account.account_number,
                    balance: account.balance,
                    account_type: account.account_type,
                },
            })),
        );
    }
    let account_data = &body.account;
    let account = Account::new_with_defaults(
        account_data.account_number.clone(),
        account_data.account_type.clone(),
        account_data.cic_number.clone(),
    );

    let inserted_account = match bank_web
        .account_service
        .create_account(&bank_web.pool, &account)
        .await
    {
        Ok(account) => account,
        Err(_) => {
            let error_data = AccountResponseData {
                id: Uuid::new_v4(),
                cic_number: account_data.cic_number.clone(),
                account_number: account_data.account_number.clone(),
                balance: account_data.balance,
                account_type: account_data.account_type.clone(),
            };
            let response =
                create_invalid_data_response(error_data, "Your custom error message here", 400);
            return (StatusCode::BAD_REQUEST, Json(Err(response)));
        }
    };

    let response_data = AccountResponseBody {
        account: AccountResponseData {
            id: inserted_account.id,
            cic_number: inserted_account.cic_number,
            account_number: inserted_account.account_number,
            balance: inserted_account.balance,
            account_type: inserted_account.account_type,
        },
    };

    (StatusCode::CREATED, Json(Ok(response_data)))
}
