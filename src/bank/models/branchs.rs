use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::{bank::{get_bank_by_id, update_total_money}, transactions};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub branch_name: String,
    pub bank_id: Uuid,
    pub pre_deposit_amount: i32,
    pub total_money: i32,
    pub debt_to_collect: i32,
    pub loans_given: i32,
    pub inserted_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_cards: i32,
    pub total_accounts: i32,
    pub total_transactions: i32,
    pub total_customers: i32,
}

pub async fn create_branch(
    pool: &PgPool,
    bank_id: Uuid,
    pre_deposit_amount: i32,
) -> Result<Branch, sqlx::Error> {
    // Validation
    if pre_deposit_amount < 300000 {
        return Err(sqlx::Error::InvalidInput(
            "Pre-deposit amount must be greater than or equal to 300000".into(),
        ));
    }

    let branch_id = Uuid::new_v4();
    let bank_name: String = sqlx::query_scalar!(
        r#"
        SELECT bank_name FROM banks WHERE id = $1
        "#,
        bank_id
    )
    .fetch_one(pool)
    .await?;

    let branch_name = format!(
        "{}-{}",
        bank_name,
        branch_id.to_string()[..4].to_uppercase()
    );

    let mut transaction = pool.begin().await?;

    // Calculate the total money for the branch
    let branch_total_money = pre_deposit_amount;

    // Insert the new branch with the calculated total_money
    let branch = sqlx::query!(
        r#"
        INSERT INTO branches (id, branch_name, bank_id, pre_deposit_amount, total_money, debt_to_collect, loans_given, inserted_at, updated_at, total_cards, total_accounts, total_transactions, total_customers)
        VALUES ($1, $2, $3, $4, $5, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, 0, 0, 0)
        RETURNING *
        "#,
        branch_id,
        branch_name,
        bank_id,
        pre_deposit_amount,
        branch_total_money
    )
    .fetch_one(&mut transaction)
    .await?;

    // Update the bank's total money
    update_total_money(&mut transaction, bank_id).await?;

    transaction.commit().await?; // Commit the transaction

    Ok(branch)
}

pub async fn get_branch_by_id(pool: &PgPool, branch_id: Uuid) -> Result<Branch, sqlx::Error> {
    let branch = sqlx::query_as!(
        Branch,
        r#"
        SELECT * FROM branches WHERE id = $1
        "#,
        branch_id
    )
    .fetch_optional(pool)
    .await?;

    match branch {
        Some(branch) => Ok(branch),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn get_branches_by_bank_id(
    pool: &PgPool,
    bank_id: Uuid,
) -> Result<Vec<Branch>, sqlx::Error> {
    let branches = sqlx::query!(
        r#"
        SELECT b.id, b.branch_name, b.bank_id, b.pre_deposit_amount, b.total_money, b.debt_to_collect, b.loans_given, b.inserted_at, b.updated_at, COUNT(c.id) AS customers_count,
        SUM(b.total_cards) AS total_cards, SUM(b.total_accounts) AS total_accounts, SUM(b.total_transactions) AS total_transactions, SUM(b.total_customers) AS total_customers
        FROM branches AS b
        LEFT JOIN customers AS c ON c.branch_id = b.id
        WHERE b.bank_id = $1
        GROUP BY b.id
        "#,
        bank_id
    )
    .fetch_all(pool)
    .await?;

    let branches: Vec<Branch> = branches
        .into_iter()
        .map(|info| Branch {
            id: info.id,
            branch_name: info.branch_name,
            bank_id: info.bank_id,
            pre_deposit_amount: info.pre_deposit_amount,
            total_money: info.total_money,
            debt_to_collect: info.debt_to_collect,
            loans_given: info.loans_given,
            inserted_at: info.inserted_at,
            updated_at: info.updated_at,
            total_cards: info.total_cards.unwrap_or(0),
            total_accounts: info.total_accounts.unwrap_or(0),
            total_transactions: info.total_transactions.unwrap_or(0),
            total_customers: info.total_customers.unwrap_or(0),
        })
        .collect();

    Ok(branches)
}

pub async fn update_total_money_on_deposit(
    pool: &PgPool,
    branch_id: Uuid,
    new_deposit: i32,
) -> Result<(), sqlx::Error> {
    let branch = get_branch_by_id(pool, branch_id).await?; // Retrieve the branch details
    let initial_deposit = branch.pre_deposit_amount; // Get the initial deposit from the branch

    // Calculate the gap between the new deposit and initial deposit
    let gap = new_deposit - initial_deposit;

    // Calculate the new total money for the branch
    let new_total_money = branch.total_money + gap;

    // Update the total money of the branch
    sqlx::query!(
        r#"
        UPDATE branches
        SET total_money = $1
        WHERE id = $2
        "#,
        new_total_money,
        branch_id
    )
    .execute(pool)
    .await?;

    let bank_id = branch.bank_id;
    let bank = get_bank_by_id(pool, bank_id).await?; // Retrieve the bank details

    // Calculate the new total money for the bank
    let new_bank_total_money = bank.total_money - gap;

    // Update the total money of the bank
    sqlx::query!(
        r#"
        UPDATE banks
        SET total_money = $1
        WHERE id = $2
        "#,
        new_bank_total_money,
        bank_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_total_customers_count(
    transaction: &PgPool,
    bank_id: Uuid,
) -> Result<(), sqlx::Error> {
    let total_customers: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(c.id) FROM customers AS c
        INNER JOIN branches AS b ON c.branch_id = b.id
        WHERE b.bank_id = $1
    "#,
        bank_id
    )
    .fetch_one(transaction)
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
    .execute(transaction)
    .await?;

    Ok(())
}

pub async fn update_branch_total_money(pool: &PgPool, branch_id: Uuid) -> Result<(), sqlx::Error> {
    let total_money: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(total_money), 0) FROM branches WHERE bank_id = (SELECT bank_id FROM branches WHERE id = $1)
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE branches
        SET total_money = $1
        WHERE id = $2
        "#,
        total_money,
        branch_id
    )
    .execute(pool)
    .await?;

    let bank_id: Uuid = sqlx::query_scalar!(
        r#"
        SELECT bank_id FROM branches WHERE id = $1
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await?;

    // Update the bank's total money
    update_total_money(pool, bank_id).await?;

    Ok(())
}

pub async fn update_total_debt_to_collect(pool: &PgPool, branch_id: Uuid) -> Result<(), sqlx::Error> {
    let total_debt_to_collect: i32 = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(SUM(debt_to_collect), 0) FROM branches WHERE bank_id = (SELECT bank_id FROM branches WHERE id = $1)
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE branches
        SET debt_to_collect = $1
        WHERE id = $2
        "#,
        total_debt_to_collect,
        branch_id
    )
    .execute(pool)
    .await?;

    let bank_id: Uuid = sqlx::query_scalar!(
        r#"
        SELECT bank_id FROM branches WHERE id = $1
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await?;

    // Update the bank's total debt to collect
    update_total_debt_to_collect(pool, bank_id).await?;

    Ok(())
}
