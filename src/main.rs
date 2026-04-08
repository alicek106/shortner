mod pages;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use cached::proc_macro::cached;
use rand::distr::{Alphanumeric, SampleString};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Clone)]
struct AppState {
    conn: redis::aio::MultiplexedConnection,
    redis_random_key_conflict_max_retry: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let redis_addr = env::var("REDIS_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let redis_random_key_conflict_max_retry: i32 = env::var("REDIS_RANDOM_KEY_CONFLICT_MAX_RETRY")
        .unwrap_or_else(|_| "10".to_string())
        .parse()?;

    let client = redis::Client::open(format!("redis://{}", redis_addr))?;
    let conn = client.get_multiplexed_async_connection().await?;
    let state = AppState {
        conn,
        redis_random_key_conflict_max_retry,
    };

    let app = Router::new()
        .route("/", get(pages::root_handler))
        .route("/new", post(new_handler))
        .route("/{*rest}", get(route_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct UrlMapping {
    short_path: Option<String>,
    dest_url: String,
}

#[derive(Clone)]
enum AppError {
    Redis(redis::RedisError),
    NotFound,
    BadRequest(&'static str),
    RedisKeyConflict, // random generated key를 했는데 중복으로 인해 실패한 경우. 보통은 발생하지 않아야 한다.
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::Redis(e) => {
                tracing::error!("Redis error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "URL mapping not found"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::RedisKeyConflict => {
                (StatusCode::CONFLICT, "Generated short path not allocated.")
            }
        };
        (status, Json(json!({"error": msg}))).into_response()
    }
}

async fn new_handler(
    // 두 개 순서가 뒤바뀌면 trait bound 에러가 발생하니 주의....
    State(state): State<AppState>,
    Json(payload): Json<UrlMapping>,
) -> Result<impl IntoResponse, AppError> {
    if url::Url::parse(&payload.dest_url).is_err() {
        tracing::warn!("Received invalid dest_url: {}", payload.dest_url);
        return Err(AppError::BadRequest("dest_url must be a valid URL"));
    }

    let final_short_path = match payload.short_path {
        Some(short_path) => {
            if short_path.trim().is_empty() || short_path.eq("/") {
                tracing::warn!("Received invalid short_path: {:?}", short_path);
                return Err(AppError::BadRequest("Invalid short path."));
            }

            match short_path {
                s if s.starts_with('/') => format!("/{}", s),
                _ => short_path,
            }
        }
        None => generate_random_short_path(&state).await?,
    };

    state
        .conn
        .clone()
        .set::<_, _, ()>(&final_short_path, payload.dest_url)
        .await
        .map_err(AppError::Redis)?;

    Ok(Json(
        json!({"message": "URL mapping received", "short_url": final_short_path}),
    ))
}

async fn generate_random_short_path(app_state: &AppState) -> Result<String, AppError> {
    for _ in 0..app_state.redis_random_key_conflict_max_retry {
        let generated = format!("/{}", Alphanumeric.sample_string(&mut rand::rng(), 16));
        let exist: bool = app_state
            .conn
            .clone()
            .exists::<String, bool>(generated.clone())
            .await
            .map_err(AppError::Redis)?;
        if !exist {
            return Ok(generated);
        }
    }
    Err(AppError::RedisKeyConflict)
}

async fn route_handler(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result = get_dest_url(&format!("/{}", path), &state).await?;
    Ok(Redirect::to(&result))
}

#[cached(
    result = false, // 존재하지 않는 url로 여러 번 요청하면 캐싱하도록
    key = "String",
    convert = r#"{ path.to_string() }"#,
    time = 60
)]
async fn get_dest_url(path: &str, state: &AppState) -> Result<String, AppError> {
    let url: Option<String> = state
        .conn
        .clone()
        .get(path)
        .await
        .map_err(AppError::Redis)?;
    match url {
        Some(u) => Ok(u),
        None => Err(AppError::NotFound),
    }
}
