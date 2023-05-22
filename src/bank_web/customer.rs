use super::BankWeb;
use crate::bank::accounts::AccountService;
use crate::bank::helper::{ request, response };
use crate::bank::models::customer;
use axum::{ extract::{ Path, State }, http::StatusCode, Json };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

pub async fn post<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Json(body): Json<request::CustomerBody>
) -> (StatusCode, Json<response::CustomerBody>) {
    let cic_number_validation = validate_input(&body.customer.cic_number, validate_cic_number);
    let customer_name_validation = validate_input(
        &body.customer.customer_name,
        validate_customer_name
    );
    let email_validation = validate_input(&body.customer.email, validate_email);
    let phone_number_validation = validate_input(
        &body.customer.phone_number,
        validate_phone_number
    );

    if let Err(validation_result) = cic_number_validation {
        return (
            StatusCode::BAD_REQUEST,
            Json(response::CustomerBody {
                customer: response::CustomerData {
                    id: Uuid::nil(),
                    customer_name: String::new(),
                    email: String::new(),
                    phone_number: String::new(),
                    cic_number: body.customer.cic_number.clone(),
                    inserted_at: None,
                    updated_at: None,
                    error_message: validation_result.error_message,
                },
            }),
        );
    }

    if let Err(validation_result) = customer_name_validation {
        return (
            StatusCode::BAD_REQUEST,
            Json(response::CustomerBody {
                customer: response::CustomerData {
                    id: Uuid::nil(),
                    customer_name: body.customer.customer_name.clone(),
                    email: String::new(),
                    phone_number: String::new(),
                    cic_number: String::new(),
                    inserted_at: None,
                    updated_at: None,
                    error_message: validation_result.error_message,
                },
            }),
        );
    }

    if let Err(validation_result) = email_validation {
        return (
            StatusCode::BAD_REQUEST,
            Json(response::CustomerBody {
                customer: response::CustomerData {
                    id: Uuid::nil(),
                    customer_name: String::new(),
                    email: body.customer.email.clone(),
                    phone_number: String::new(),
                    cic_number: String::new(),
                    inserted_at: None,
                    updated_at: None,
                    error_message: validation_result.error_message,
                },
            }),
        );
    }

    if let Err(validation_result) = phone_number_validation {
        return (
            StatusCode::BAD_REQUEST,
            Json(response::CustomerBody {
                customer: response::CustomerData {
                    id: Uuid::nil(),
                    customer_name: String::new(),
                    email: String::new(),
                    phone_number: body.customer.phone_number.clone(),
                    cic_number: String::new(),
                    inserted_at: None,
                    updated_at: None,
                    error_message: validation_result.error_message,
                },
            }),
        );
    }

    let existing_customer = customer
        ::get_by_customer_cic_number(&bank_web.pool, &body.customer.cic_number).await
        .unwrap();

    if let Some(customer) = existing_customer {
        return (
            StatusCode::BAD_REQUEST,
            Json(response::CustomerBody {
                customer: response::CustomerData {
                    id: customer.id,
                    customer_name: customer.customer_name.clone(),
                    email: customer.email.clone(),
                    phone_number: customer.phone_number.clone(),
                    cic_number: customer.cic_number.clone(),
                    inserted_at: customer.inserted_at.clone(),
                    updated_at: customer.updated_at.clone(),
                    error_message: Some(
                        "Customer with the same CIC number already exists.".to_string()
                    ),
                },
            }),
        );
    }

    let customer_id = customer
        ::insert(
            &bank_web.pool,
            body.customer.customer_name,
            body.customer.email,
            body.customer.phone_number,
            body.customer.cic_number
        ).await
        .unwrap();

    let customer = customer::get(&bank_web.pool, customer_id).await.unwrap();

    (
        StatusCode::CREATED,
        Json(response::CustomerBody {
            customer: response::CustomerData {
                id: customer.id,
                customer_name: customer.customer_name.clone(),
                email: customer.email.clone(),
                phone_number: customer.phone_number.clone(),
                cic_number: customer.cic_number.clone(),
                inserted_at: customer.inserted_at.clone(),
                updated_at: customer.updated_at.clone(),
                error_message: None,
            },
        }),
    )
}

pub async fn get<T: AccountService>(
    State(bank_web): State<BankWeb<T>>,
    Path(customer_id): Path<Uuid>
) -> (StatusCode, Json<response::CustomerBody>) {
    let customer = customer::get(&bank_web.pool, customer_id).await.unwrap();

    (
        StatusCode::OK,
        Json(response::CustomerBody {
            customer: response::CustomerData {
                id: customer.id,
                customer_name: customer.customer_name.clone(),
                email: customer.email.clone(),
                phone_number: customer.phone_number.clone(),
                cic_number: customer.cic_number.clone(),
                inserted_at: customer.inserted_at.clone(),
                updated_at: customer.updated_at.clone(),
                error_message:None,
            },
        }),
    )
}
