use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::types::Status;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Refund {
    pub branch_id: Uuid,
    pub bank_id: Uuid,
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub refund_amount: i32,
    pub refund_date: NaiveDate,
    pub status: Status,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_by_transaction_id(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<Option<Refund>, sqlx::Error> {
    let refund = sqlx::query_as!(
        Refund,
        r#"
        SELECT id, transaction_id, refund_amount, refund_date, status as "status:_", inserted_at, updated_at
        FROM refunds
        WHERE transaction_id = $1
        "#,
        transaction_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(refund)
}

pub async fn create_refund(
    pool: &PgPool,
    transaction_id: Uuid,
    refund_amount: i32,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let current_date = Utc::now().naive_utc().date();

    sqlx::query!(
        r#"
        INSERT INTO refunds (id, transaction_id, refund_amount, refund_date, status, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id
        "#,
        id,
        transaction_id,
        refund_amount,
        current_date,
        Status::Pending as i32, // Assuming you have an enum `Status` with appropriate values
    )
    .fetch_one(pool)
    .await
    .map(|record| record.id)
}
