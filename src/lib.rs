pub mod api;
pub mod model;
pub mod store;

use axum::{routing::{get, post}, Router};
use store::MemoryStore;

pub fn router(store: MemoryStore) -> Router {
    Router::new()
        .route("/healthz", get(api::health))
        .route("/readyz", get(api::ready))
        .route("/v1/claims", post(api::append_claim))
        .route("/v1/claims/{claim_id}/supersede", post(api::supersede_claim))
        .route("/v1/recall", post(api::recall))
        .with_state(store)
}
