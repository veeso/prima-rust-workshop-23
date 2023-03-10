use rust_decimal::Decimal;
use uuid::Uuid;

use super::{DatabaseError, DatabaseResult, StoreDb};

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Article {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub unit_price: Decimal,
}

impl Article {
    /// Find `Article` by `id`
    pub async fn find_by_id(db: &StoreDb, id: &Uuid) -> DatabaseResult<Option<Article>> {
        sqlx::query_as(r#"SELECT * FROM article WHERE id = $1"#)
            .bind(id)
            .fetch_optional(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Find article by name
    pub async fn find_by_name(
        db: &StoreDb,
        name: &str,
        offset: i64,
        limit: i64,
    ) -> DatabaseResult<Vec<Article>> {
        sqlx::query_as(r#"SELECT * FROM article WHERE name LIKE $1 OFFSET $2 LIMIT $3"#)
            .bind(format!("%{name}"))
            .bind(offset)
            .bind(limit)
            .fetch_all(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Get all articles with pagination
    pub async fn get_all(db: &StoreDb, offset: i64, limit: i64) -> DatabaseResult<Vec<Article>> {
        sqlx::query_as(r#"SELECT * FROM article OFFSET $1 LIMIT $2"#)
            .bind(offset)
            .bind(limit)
            .fetch_all(db.pool())
            .await
            .map_err(DatabaseError::from)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::env;

    #[tokio::test]
    async fn should_find_article_by_id() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let article = insert_article(&db, "bubblegums").await;
        assert_eq!(
            article,
            Article::find_by_id(&db, &article.id)
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[tokio::test]
    async fn should_not_find_article_by_id() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        assert!(Article::find_by_id(&db, &Uuid::new_v4())
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn should_find_article_by_name() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");
        let cat = insert_article(&db, "cat").await;
        let dog = insert_article(&db, "dog").await;
        let maine_coon = insert_article(&db, "maine coon cat").await;
        assert_eq!(
            Article::find_by_name(&db, "cat", 0, 64)
                .await
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            Article::find_by_name(&db, "cat", 1, 64)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            Article::find_by_name(&db, "cat", 0, 1).await.unwrap().len(),
            1
        );
        assert_eq!(
            Article::find_by_name(&db, "cat", 2, 4).await.unwrap().len(),
            0
        );
    }

    async fn insert_article(db: &StoreDb, name: &str) -> Article {
        let article = Article {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: "Lorem Ipsum".to_string(),
            unit_price: rust_decimal_macros::dec!(23.04),
        };
        let rows = sqlx::query(
            "INSERT INTO article (id, name, description, unit_price) VALUES ($1, $2, $3, $4)",
        )
        .bind(article.id)
        .bind(&article.name)
        .bind(&article.description)
        .bind(article.unit_price)
        .execute(db.pool())
        .await
        .map_err(DatabaseError::from)
        .unwrap()
        .rows_affected();
        if rows != 1 {
            panic!("too many inserts");
        }

        article
    }
}
