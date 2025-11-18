use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, instrument};

mod cache;
mod config;
mod db;
mod error;
mod metrics;
mod rate_limit;

use cache::Cache;
use config::AppConfig;
use error::AppError;
use metrics::Metrics;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: uuid::Uuid,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct StatsResponse {
    total_users: i64,
    cache_hits: u64,
    cache_misses: u64,
    requests_per_second: f64,
}

// Application state
#[derive(Clone)]
struct AppState {
    db: PgPool,
    cache: Arc<Cache>,
    metrics: Arc<Metrics>,
}

#[instrument]
async fn health_check() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION"),
    }))
}

#[instrument(skip(state))]
async fn get_user(
    Path(user_id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    state.metrics.record_request("get_user");

    // Try cache first
    let cache_key = format!("user:{}", user_id);
    if let Some(user) = state.cache.get::<User>(&cache_key).await? {
        state.metrics.record_cache_hit();
        return Ok(Json(user));
    }

    state.metrics.record_cache_miss();

    // Fetch from database
    let user = db::get_user_by_id(&state.db, user_id).await?;

    // Cache the result
    state.cache.set(&cache_key, &user, 300).await?;

    Ok(Json(user))
}

#[instrument(skip(state))]
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    state.metrics.record_request("create_user");

    let user = db::create_user(&state.db, &payload.name, &payload.email).await?;

    // Invalidate cache if needed
    let cache_key = format!("user:{}", user.id);
    state.cache.delete(&cache_key).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[instrument(skip(state))]
async fn list_users(
    Query(params): Query<std::collections::HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, AppError> {
    state.metrics.record_request("list_users");

    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(100);

    let offset = params
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);

    // Try cache first
    let cache_key = format!("users:limit:{}:offset:{}", limit, offset);
    if let Some(users) = state.cache.get::<Vec<User>>(&cache_key).await? {
        state.metrics.record_cache_hit();
        return Ok(Json(users));
    }

    state.metrics.record_cache_miss();

    let users = db::list_users(&state.db, limit, offset).await?;

    // Cache with shorter TTL for list queries
    state.cache.set(&cache_key, &users, 60).await?;

    Ok(Json(users))
}

#[instrument(skip(state))]
async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, AppError> {
    let total_users = db::count_users(&state.db).await?;
    let metrics = state.metrics.get_stats();

    Ok(Json(StatsResponse {
        total_users,
        cache_hits: metrics.cache_hits,
        cache_misses: metrics.cache_misses,
        requests_per_second: metrics.requests_per_second,
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fastestapi=info,tower_http=info".into()),
        )
        .json()
        .init();

    // Load configuration
    let config = AppConfig::from_env();

    // Initialize database connection pool
    info!("Connecting to database...");
    let db = db::create_pool(&config.database_url).await?;
    db::run_migrations(&db).await?;

    // Initialize Redis cache
    info!("Connecting to Redis...");
    let cache = Arc::new(Cache::new(&config.redis_url).await?);

    // Initialize metrics
    let metrics = Arc::new(Metrics::new());

    // Create application state
    let state = AppState {
        db,
        cache,
        metrics: metrics.clone(),
    };

    // Build router with middleware
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user))
        .route("/api/stats", get(get_stats))
        .route("/metrics", get(metrics::metrics_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(CorsLayer::permissive())
                .layer(rate_limit::rate_limit_layer())
        )
        .with_state(state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

