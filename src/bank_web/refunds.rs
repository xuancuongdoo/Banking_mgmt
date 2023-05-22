use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::BankWeb;
use crate::bank::{accounts::AccountService, refunds};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestData {
    refund_amount: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestBody {
    refund: RequestData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseData {
    id: Uuid,
    amount: i32,
    payment_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    data: ResponseData,
}

pub async fn post<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Path(payment_id): Path<Uuid>,
    Json(body): Json<RequestBody>,
) -> (StatusCode, Json<ResponseBody>) {
    let refund_id = refunds::insert(&bank_web.pool,
         payment_id,
          body.refund.refund_amount,
        )
        .await
        .unwrap();

    (
        StatusCode::CREATED,
        Json(ResponseBody {
            data: ResponseData {
                id: refund_id,
                amount: body.refund.refund_amount,
                payment_id,
            },
        }),
    )
}

pub async fn get<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Path((payment_id, refund_id)): Path<(Uuid, Uuid)>,
) -> (StatusCode, Json<ResponseBody>) {
    let data = refunds::get(&bank_web.pool, refund_id).await.unwrap();

    (
        StatusCode::OK,
        Json(ResponseBody {
            data: ResponseData {
                id: data.id,
                amount: data.refund_amount,
                payment_id,
            },
        }),
    )
}

