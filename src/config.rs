use std::env;

pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://postgres:postgres@localhost:5432/fastestapi".to_string()
            }),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        }
    }
}

