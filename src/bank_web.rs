use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::bank::accounts::AccountService;
mod accounts;
mod cards;
mod customer;
mod payments;
mod refunds;
#[derive(Clone)]
pub struct BankWeb<T> {
    pool: PgPool,
    account_service: T,
}

impl<T: AccountService> BankWeb<T> {
    pub fn new(pool: PgPool, account_service: T) -> Self {
        Self {
            pool,
            account_service,
        }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/api/customers", post(customer::post::<T>))
            .route("/api/customers/:customer_id", get(customer::get::<T>))
            .route(
                "/api/payments/:payment_id/refunds",
                post(refunds::post::<T>),
            )
            .route(
                "/api/payments/:payment_id/refunds/:refund_id",
                get(refunds::get::<T>),
            )
            .route("/api/cards", post(cards::post::<T>))
            .route("/api/cards/:card_number", post(cards::get::<T>))
            .route("/api/account/", post(accounts::create_account::<T>))
            .layer(axum_tracing_opentelemetry::opentelemetry_tracing_layer())
            .with_state(self)
            .with_state(())
    }
}
