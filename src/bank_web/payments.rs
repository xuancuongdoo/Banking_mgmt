use super::BankWeb;
use crate::bank::payments::{Status};
use crate::bank::{accounts::AccountService, payments};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestData {
    pub card_id: Uuid,
    pub amount: i32,
    pub status: payments::Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RequestBody {
    pub payment: RequestData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseData {
    pub payment_id: Uuid,
    pub card_id: Uuid,
    pub amount: i32,
    pub status: payments::Status,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseBody {
    pub data: ResponseData,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct InvalidDataResponse<T> {
    pub response_data: T,
    pub invalid_data: InvalidData,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct InvalidData {
    pub error_message: String,
    pub status_code: i32,
}

pub fn create_invalid_data_response<T>(
    response_data: T,
    error_message: &str,
    status_code: i32,
) -> InvalidDataResponse<T> {
    InvalidDataResponse {
        response_data,
        invalid_data: InvalidData {
            error_message: error_message.to_string(),
            status_code,
        },
    }
}

pub async fn post<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Json(body): Json<RequestBody>,
) -> (
    StatusCode,
    Json<Result<ResponseBody, InvalidDataResponse<ResponseData>>>,
) {
    let existing_payment = payments::get_by_card_id(&bank_web.pool, &body.payment.card_id)
        .await
        .unwrap();

    if let Some(payment) = existing_payment {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(Ok(ResponseBody {
                data: ResponseData {
                    payment_id: payment.id,
                    card_id: payment.card_id,
                    amount: payment.amount,
                    status: payment.status,
                },
            })),
        );
    }
    if body.payment.amount < 0 {
        let response_data = ResponseData {
            payment_id :  Uuid::nil(),
            amount: body.payment.amount,
            card_id: body.payment.card_id,
            status: Status::Declined,
        };
        let invalid_data_response = create_invalid_data_response(
            response_data,
            "invalid amount must be a interger (> 0)",
            400,
        );

        return (StatusCode::PAYMENT_REQUIRED, Json(Err(invalid_data_response)));
    }
    let payment_id = payments::insert(
        &bank_web.pool,
        body.payment.card_id,
        body.payment.amount,
        payments::Status::Approved,
    )
    .await
    .unwrap();

    let payment = payments::get(&bank_web.pool, payment_id).await.unwrap();

    (
        StatusCode::CREATED,
        Json(Ok(ResponseBody {
            data: ResponseData {
                payment_id : payment.id,
                amount: payment.amount,
                card_id: payment.card_id,
                status: payment.status,
            },
        })),
    )
}

pub async fn get<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Path(payment_id): Path<Uuid>,
) -> (StatusCode, Json<ResponseBody>) {
    let payment_result = payments::get(&bank_web.pool, payment_id).await;

    match payment_result {
        Ok(payment) => (
            StatusCode::OK,
            Json(ResponseBody {
                data: ResponseData {
                    payment_id: payment.id,
                    amount: payment.amount,
                    card_id: payment.card_id,
                    status: payment.status,
                },
            }),
        ),
        Err(e) => {
            // Log the error or create a custom error response
            eprintln!("Error: {:?}", e);
            (
                StatusCode::NOT_FOUND,
                Json(ResponseBody {
                    data: ResponseData {
                        payment_id: Uuid::nil(),
                        amount: 0,
                        card_id: Uuid::nil(),
                        status : payments::Status::Failed,

                    },
                }),
            )
        }
    }
}
