use super::BankWeb;
use crate::bank::accounts::AccountService;
use crate::bank::cards::{self, Card, CardStatus, CardType};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestData {
    pub card_number: String,
    pub account_id: Uuid,
    pub card_type: CardType,
    pub cvv: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestBody {
    pub card: RequestData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseData {
    pub card_id: Uuid,
    pub card_number: String,
    pub account_id: Uuid,
    pub card_type: CardType,
    pub expiration_date: String,
    pub card_status: CardStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseBody {
    pub data: ResponseData,
}

pub async fn post<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Json(body): Json<RequestBody>,
) -> (StatusCode, Json<Result<ResponseBody, String>>) {
    let card_id = match cards::insert(
        &bank_web.pool,
        body.card.card_number.clone(),
        body.card.account_id,
        body.card.card_type,
        body.card.cvv.clone(),
    )
    .await
    {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(Err("Failed to create card".to_string())),
            )
        }
    };

    let card = cards::get_by_card_number(&bank_web.pool, &body.card.card_number)
        .await
        .unwrap()
        .unwrap();

    (
        StatusCode::CREATED,
        Json(Ok(ResponseBody {
            data: ResponseData {
                card_id: card.id,
                card_number: card.card_number,
                account_id: card.account_id,
                card_type: card.card_type,
                expiration_date: card.expiration_date.to_string(),
                card_status: card.card_status,
            },
        })),
    )
}

pub async fn get<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Path(card_number): Path<String>,
) -> (StatusCode, Json<Result<ResponseBody, String>>) {
    let card = match cards::get_by_card_number(&bank_web.pool, &card_number).await {
        Ok(card) => card,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(Err("Card not found".to_string())),
            )
        }
    };

    let card = match card {
        Some(c) => c,
        None => panic!("Card not found"),
    };
    (
        StatusCode::OK,
        Json(Ok(ResponseBody {
            data: ResponseData {
                card_id: card.id,
                card_number: card.card_number,
                account_id: card.account_id,
                card_type: card.card_type,
                expiration_date: card.expiration_date.to_string(),
                card_status: card.card_status,
            },
        })),
    )
}
