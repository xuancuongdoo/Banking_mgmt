use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::bank::helper::{ validation::{*}};
use super::{accounts, cards};




#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Customer {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub customer_name: String,
    pub email: String,
    pub phone_number: String,
    pub cic_number: String,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
pub async fn create_customer(
    pool: &PgPool,
    branch_id: Uuid,
    bank_id: Uuid,
    customer_name: String,
    email: String,
    phone_number: String,
    cic_number: String,
) -> Result<Customer, CustomerErrorReps> {
    // Check if the bank exists
    let bank_exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM banks
            WHERE id = $1
        )
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await
    .map_err(CustomerErrorReps::DatabaseError)?;

    if !bank_exists {
        return Err(CustomerErrorReps::BankNotFound);
    }

    // Check if the branch belongs to the bank
    let branch_belongs_to_bank = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM branches
            WHERE id = $1 AND bank_id = $2
        )
        "#,
        branch_id,
        bank_id
    )
    .fetch_one(pool)
    .await
    .map_err(CustomerErrorReps::DatabaseError)?;

    if !branch_belongs_to_bank {
        return Err(CustomerErrorReps::BranchNotFound);
    }

    // Validate input
    let validated_customer_name = validate_input(customer_name.clone(), validate_customer_name)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let validated_email = validate_input(email.clone(), validate_email)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let validated_phone_number = validate_input(phone_number.clone(), validate_phone_number)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let validated_cic_number = validate_input(cic_number.clone(), validate_cic_number)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let customer_id = Uuid::new_v4();
    let current_timestamp = chrono::Utc::now().naive_utc();

    let customer = sqlx::query!(
        r#"
        INSERT INTO customers (branch_id, bank_id, id, customer_name, email, phone_number, cic_number, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        branch_id,
        bank_id,
        customer_id,
        validated_customer_name,
        validated_email,
        validated_phone_number,
        validated_cic_number,
    )
    .fetch_one(pool)
    .await
    .map_err(CustomerErrorReps::DatabaseError)?;

    Ok(Customer {
        branch_id: customer.branch_id,
        bank_id: customer.bank_id,
        id: customer.id,
        customer_name: customer.customer_name,
        email: customer.email,
        phone_number: customer.phone_number,
        cic_number: customer.cic_number,
        inserted_at: customer.inserted_at,
        updated_at: customer.updated_at,
    })
}


pub async fn get_customer_by_cic_phone_name_and_bank_id(
    pool: &PgPool,
    cic_number: &str,
    phone_number: &str,
    customer_name: &str,
    bank_id: Uuid,
) -> Result<Customer, CustomerErrorReps> {
    let validated_customer_name = validate_input(customer_name.clone(), validate_customer_name)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let validated_phone_number = validate_input(phone_number.clone(), validate_phone_number)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let validated_cic_number = validate_input(cic_number.clone(), validate_cic_number)
        .map_err(|validation_result| CustomerErrorReps::InvalidInput(validation_result.error_message.unwrap()))?;

    let customer = sqlx::query_as!(
        Customer,
        "
        SELECT *
        FROM customers
        WHERE cic_number = $1 AND phone_number = $2 AND customer_name = $3 AND bank_id = $4
        ",
        cic_number,
        phone_number,
        customer_name,
        bank_id,
    )
    .fetch_optional(pool)
    .await?;

    customer.ok_or(CustomerErrorReps::NotFound)
}


