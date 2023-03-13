//! # Database connector

use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres};
use std::sync::Arc;
use thiserror::Error;

mod tables;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
type PgPool = Pool<Postgres>;

pub use tables::{Article, Customer, CustomerOrder, OrderArticle, OrderStatus};

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
#[derive(Clone, Debug)]
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
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl<'c> Executor<'c> for &StoreDb {
    type Database = Postgres;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> tonic::codegen::futures_core::stream::BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.pool().fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> tonic::codegen::futures_core::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
    >
    where
        'c: 'e,
        E: sqlx::Execute<'q, Self::Database>,
    {
        self.pool().fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> tonic::codegen::futures_core::future::BoxFuture<
        'e,
        Result<<Self::Database as sqlx::database::HasStatement<'q>>::Statement, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        self.pool().prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> tonic::codegen::futures_core::future::BoxFuture<
        'e,
        Result<sqlx::Describe<Self::Database>, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        self.pool().describe(sql)
    }
}
