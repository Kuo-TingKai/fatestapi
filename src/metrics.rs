use axum::{
    http::StatusCode,
    response::Response,
};
use prometheus::{
    register_counter, register_histogram, register_int_counter, Counter, Histogram, IntCounter,
    Encoder, TextEncoder,
};
use std::sync::Arc;
use std::time::Instant;

pub struct Metrics {
    requests_total: Counter,
    requests_by_endpoint: IntCounter,
    cache_hits: IntCounter,
    cache_misses: IntCounter,
    request_duration: Histogram,
    start_time: Instant,
}

impl Metrics {
    pub fn new() -> Self {
        let requests_total = register_counter!(
            "http_requests_total",
            "Total number of HTTP requests"
        )
        .unwrap();

        let requests_by_endpoint = register_int_counter!(
            "http_requests_by_endpoint_total",
            "Total number of HTTP requests by endpoint"
        )
        .unwrap();

        let cache_hits = register_int_counter!(
            "cache_hits_total",
            "Total number of cache hits"
        )
        .unwrap();

        let cache_misses = register_int_counter!(
            "cache_misses_total",
            "Total number of cache misses"
        )
        .unwrap();

        let request_duration = register_histogram!(
            "http_request_duration_seconds",
            "HTTP request duration in seconds"
        )
        .unwrap();

        Self {
            requests_total,
            requests_by_endpoint,
            cache_hits,
            cache_misses,
            request_duration,
            start_time: Instant::now(),
        }
    }

    pub fn record_request(&self, endpoint: &str) {
        self.requests_total.inc();
        self.requests_by_endpoint.inc();
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.inc();
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.inc();
    }

    pub fn record_duration(&self, duration: f64) {
        self.request_duration.observe(duration);
    }

    pub fn get_stats(&self) -> MetricsStats {
        let uptime = self.start_time.elapsed().as_secs_f64();
        let total_requests = self.requests_total.get() as u64;
        let requests_per_second = if uptime > 0.0 {
            total_requests as f64 / uptime
        } else {
            0.0
        };

        MetricsStats {
            cache_hits: self.cache_hits.get(),
            cache_misses: self.cache_misses.get(),
            requests_per_second,
        }
    }
}

pub struct MetricsStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub requests_per_second: f64,
}

pub async fn metrics_handler() -> Result<Response<String>, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = String::from_utf8(buffer).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", encoder.format_type())
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

