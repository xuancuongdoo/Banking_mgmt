use super::types::{CardStatus, CardType};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub card_number: String,
    pub account_number: String,
    pub expiration_date: NaiveDate,
    pub cvv: String,
    pub issued_date: NaiveDate,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub balance: i32,
    pub card_status: CardStatus,
    pub card_type: CardType,
    pub bank_id: Uuid,
    pub branch_id: Uuid,
}

pub async fn insert_card(
    pool: &PgPool,
    bank_id: Uuid,
    branch_id: Uuid,
    card_number: String,
    account_number: String,
    expiration_date: NaiveDate,
    cvv: String,
    issued_date: NaiveDate,
    balance: i32,
    card_status: CardStatus,
    card_type: CardType,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();

    let card_id = sqlx::query!(
        r#"
        INSERT INTO cards (id, card_number, account_number, expiration_date, cvv, issued_date, inserted_at, updated_at, balance, card_status, card_type, bank_id, branch_id)
        VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $7, $8, $9, $10, $11)
        RETURNING id
        "#,
        id,
        card_number,
        account_number,
        expiration_date,
        cvv,
        issued_date,
        balance,
        card_status as CardStatus,
        card_type as CardType,
        bank_id,
        branch_id,
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(card_id)
}

pub async fn get_by_card_number(
    pool: &PgPool,
    card_number: &str,
) -> Result<Option<Card>, sqlx::Error> {
    let card = sqlx::query_as!(
        Card,
        r#"
        SELECT id, card_number, account_number, expiration_date, cvv, issued_date, inserted_at, updated_at, balance, card_status as "card_status: CardStatus", card_type as "card_type: CardType", bank_id, branch_id
        FROM cards
        WHERE card_number = $1
        LIMIT 1
        "#,
        card_number
    )
    .fetch_optional(pool)
    .await?;

    Ok(card)
}
