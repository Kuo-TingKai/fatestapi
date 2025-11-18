use crate::error::AppError;
use sqlx::PgPool;
use tracing::instrument;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    sqlx::migrate!("migrations").run(pool).await?;
    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[instrument(skip(pool))]
pub async fn get_user_by_id(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<crate::User, AppError> {
    let row = sqlx::query_as!(
        UserRow,
        "SELECT id, name, email, created_at FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(user) => Ok(crate::User {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }),
        None => Err(AppError::NotFound),
    }
}

#[instrument(skip(pool))]
pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<crate::User, AppError> {
    let id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO users (id, name, email, created_at) VALUES ($1, $2, $3, $4)",
        id,
        name,
        email,
        now
    )
    .execute(pool)
    .await?;

    Ok(crate::User {
        id,
        name: name.to_string(),
        email: email.to_string(),
        created_at: now,
    })
}

#[instrument(skip(pool))]
pub async fn list_users(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::User>, AppError> {
    let rows = sqlx::query_as!(
        UserRow,
        "SELECT id, name, email, created_at FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| crate::User {
            id: row.id,
            name: row.name,
            email: row.email,
            created_at: row.created_at,
        })
        .collect())
}

#[instrument(skip(pool))]
pub async fn count_users(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    Ok(count.unwrap_or(0))
}

