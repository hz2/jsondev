use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use jsondev::{build_router, state::AppState};
use std::sync::Arc;
use tower::ServiceExt;

fn app() -> axum::Router {
    let state = Arc::new(AppState::new().expect("failed to build AppState"));
    build_router(state)
}

async fn get(app: axum::Router, uri: &str) -> (StatusCode, String) {
    let res = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&body).into_owned())
}

#[tokio::test]
async fn index_ok() {
    let (status, body) = get(app(), "/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("jsondev"));
}

#[tokio::test]
async fn blog_list_ok() {
    let (status, body) = get(app(), "/blog").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("blog"));
}

#[tokio::test]
async fn blog_post_ok() {
    let (status, body) = get(app(), "/blog/2025/12/fitness").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("fitness tracker"));
}

#[tokio::test]
async fn blog_post_missing_is_500() {
    let (status, _) = get(app(), "/blog/2025/01/does-not-exist").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn notes_ok() {
    let (status, _) = get(app(), "/notes").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn bookshelf_ok() {
    let (status, body) = get(app(), "/bookshelf").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Bookshelf"));
}

#[tokio::test]
async fn workouts_ok() {
    let (status, body) = get(app(), "/workouts").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("running"));
}

#[tokio::test]
async fn recipes_list_ok() {
    let (status, body) = get(app(), "/recipes").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("broccoli cheddar"));
}

#[tokio::test]
async fn recipe_detail_ok() {
    let (status, body) = get(app(), "/recipes/broccoli-cheddar-pasta-bake").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("broccoli cheddar pasta bake"));
}

#[tokio::test]
async fn unknown_route_is_404() {
    let (status, _) = get(app(), "/does-not-exist").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
