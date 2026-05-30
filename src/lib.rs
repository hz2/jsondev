pub mod content;
pub mod error;
pub mod routes;
pub mod state;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn build_router(state: Arc<state::AppState>) -> Router {
    let static_dir = state.static_dir.clone();
    Router::new()
        .route("/", get(routes::index))
        .route("/blog", get(routes::blog::list))
        .route("/blog/{year}/{month}/{slug}", get(routes::blog::post))
        .route("/notes", get(routes::notes::list))
        .route("/notes/{slug}", get(routes::notes::post))
        .route("/bookshelf", get(routes::bookshelf))
        .route("/workouts", get(routes::workouts::page))
        .route("/recipes", get(routes::recipes::list))
        .route("/recipes/{slug}", get(routes::recipes::detail))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state)
}
