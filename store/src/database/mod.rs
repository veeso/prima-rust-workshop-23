//! # Database connector

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use thiserror::Error;

mod tables;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
type PgPool = Pool<Postgres>;

pub use tables::{Article, Customer, CustomerOrder, OrderArticle};

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("unexpected inserts count")]
    TooManyInserts,
    #[error("database error: {0}")]
    Db(sqlx::Error),
    #[error("migrate error: {0}")]
    Migrate(sqlx::migrate::MigrateError),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(e: sqlx::Error) -> Self {
        Self::Db(e)
    }
}

impl From<sqlx::migrate::MigrateError> for DatabaseError {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Self::Migrate(e)
    }
}

/// Store database interface
#[derive(Clone)]
pub struct StoreDb {
    pool: Arc<PgPool>,
}

impl StoreDb {
    /// Connect to database
    pub async fn connect(database_url: &str) -> DatabaseResult<Self> {
        let pool = PgPoolOptions::new().connect(database_url).await?;
        debug!("connecting to {database_url}");
        let db = Self {
            pool: Arc::new(pool),
        };
        db.migrate().await?;
        Ok(db)
    }

    /// Run migrations
    async fn migrate(&self) -> DatabaseResult<()> {
        debug!("running migrations");
        sqlx::migrate!()
            .run(self.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Get reference to pool
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}
