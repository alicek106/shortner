mod handler;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use redis::{AsyncCommands, Client};
use serde::Deserialize;
use serde_json::json;

#[derive(Clone)]
struct AppState {
    client: Client,
}

// TODO: funcache 달아서 매번 redis 매번 안쳐도 되도록 만들기

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: config parser로 빼기
    let client = redis::Client::open("redis://10.200.0.4/")?;

    let state = AppState { client };

    let app = Router::new()
        .route("/", get(handler::handler))
        .route("/new", post(new_handler))
        .route("/{*rest}", get(route_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct UrlMapping {
    old_url: String,
    new_url: String,
}

async fn new_handler(
    // 두 개 순서가 뒤바뀌면 trait bound 에러가 발생하니 주의....
    State(state): State<AppState>,
    Json(payload): Json<UrlMapping>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!(
        "Setting mapping: {} -> {}",
        payload.old_url, payload.new_url
    );

    let mut conn = state
        .client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to get connection: {}", e)})),
            )
        })?;

    conn.set::<_, _, ()>(payload.old_url, payload.new_url)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to set key: {}", e)})),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({"message": "URL mapping received"})),
    ))
}

async fn route_handler(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = state
        .client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to get connection: {}", e)})),
            )
        })?;

    println!("Received request for path: /{}", path);
    let result: String = conn.get(format!("/{}", path)).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to get key: {}", e)})),
        )
    })?;

    if !(result.starts_with("http://") || result.starts_with("https://")) {
        println!("Redirecting to: {}", result);
        return Ok(Redirect::to(&format!("http://{}", result)));
    }

    Ok(Redirect::to(&result))
}
