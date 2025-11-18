use crate::error::AppError;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Cache {
    client: Arc<Mutex<redis::Client>>,
}

impl Cache {
    pub async fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, AppError>
    where
        T: DeserializeOwned,
    {
        let client = self.client.lock().await;
        let mut conn = client.get_async_connection().await?;

        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl_seconds: usize) -> Result<(), AppError>
    where
        T: Serialize,
    {
        let client = self.client.lock().await;
        let mut conn = client.get_async_connection().await?;

        let serialized = serde_json::to_string(value)?;
        conn.set_ex(key, serialized, ttl_seconds).await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        let client = self.client.lock().await;
        let mut conn = client.get_async_connection().await?;

        conn.del::<_, ()>(key).await?;
        Ok(())
    }
}

