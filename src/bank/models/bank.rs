use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::branchs::{get_branches_by_bank_id, Branch};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Bank {
    pub id: Uuid,
    pub bank_name: String,
    pub fee: i32,
    pub total_money: i32,
    pub total_debt_to_collect: i32,
    pub total_loans_given: i32,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_cards: i32,
    pub total_accounts: i32,
    pub total_transactions: i32,
    pub total_customers: i32,
}

pub async fn insert(pool: &PgPool, bank_name: String, fee: i32) -> Result<Uuid, sqlx::Error> {
    let bank_id = Uuid::new_v4();

    let bank = sqlx::query!(
        r#"
        INSERT INTO banks (id, bank_name, fee, total_money, total_debt_to_collect, total_loans_given, inserted_at, updated_at)
        VALUES ($1, $2, $3, 0, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        bank_id,
        bank_name,
        fee,
    )
    .fetch_one(pool)
    .await?;

    Ok(bank.id)
}

pub async fn get_bank_by_id(pool: &PgPool, bank_id: Uuid) -> Result<Option<Bank>, sqlx::Error> {
    let bank = sqlx::query_as!(
        Bank,
        r#"
        SELECT * FROM banks WHERE id = $1
        "#,
        bank_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(bank)
}

pub async fn get_by_bank_name(pool: &PgPool, bank_name: &str) -> Result<Option<Bank>, sqlx::Error> {
    let bank = sqlx::query_as!(
        Bank,
        r#"
        SELECT *
        FROM banks
        WHERE bank_name = $1
        LIMIT 1
        "#,
        bank_name
    )
    .fetch_optional(pool)
    .await?;

    Ok(bank)
}

pub async fn get_branches_info(pool: &PgPool, bank_id: Uuid) -> Result<Vec<Branch>, sqlx::Error> {
    let branches = get_branches_by_bank_id(pool, bank_id).await?;

    Ok(branches)
}

pub async fn get_total_customers_count(pool: &PgPool, bank_id: Uuid) -> Result<i64, sqlx::Error> {
    let total_customers: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(id) FROM customers WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    Ok(total_customers)
}
pub async fn update_total_money(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_money: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_money), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_money = $1
        WHERE id = $2
        "#,
        total_money,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_debt_to_collect(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_debt_to_collect: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(debt_to_collect), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_debt_to_collect = $1
        WHERE id = $2
        "#,
        total_debt_to_collect,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_loans_given(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_loans_given: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(loans_given), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_loans_given = $1
        WHERE id = $2
        "#,
        total_loans_given,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_cards(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_cards: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_cards), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_cards = $1
        WHERE id = $2
        "#,
        total_cards,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_accounts(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_accounts: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_accounts), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_accounts = $1
        WHERE id = $2
        "#,
        total_accounts,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_transactions(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_transactions: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_transactions), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_transactions = $1
        WHERE id = $2
        "#,
        total_transactions,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_customers(pool: &PgPool, bank_id: Uuid) -> Result<(), sqlx::Error> {
    let total_customers: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_customers), 0) FROM branches WHERE bank_id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE banks
        SET total_customers = $1
        WHERE id = $2
        "#,
        total_customers,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
