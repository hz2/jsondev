use axum::{routing::get, Router};
use std::{env, sync::Arc};
use tower_http::services::ServeDir;

mod content;
mod error;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jsondev=info,tower_http=info".into()),
        )
        .init();

    let state = Arc::new(state::AppState::new()?);

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/blog", get(routes::blog::list))
        .route("/blog/{year}/{month}/{slug}", get(routes::blog::post))
        .route("/bookshelf", get(routes::bookshelf))
        .route("/workouts", get(routes::workouts::page))
        .route("/recipes", get(routes::recipes::list))
        .route("/recipes/{slug}", get(routes::recipes::detail))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
