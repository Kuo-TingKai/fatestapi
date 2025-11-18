use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

pub fn rate_limit_layer() -> GovernorLayer {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(100) // Allow 100 requests per second per IP
            .burst_size(200) // Allow burst of 200 requests
            .finish()
            .unwrap(),
    );

    GovernorLayer::new(config)
}

