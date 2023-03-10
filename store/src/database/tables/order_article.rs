use rust_decimal::Decimal;
use uuid::Uuid;

use super::{DatabaseError, DatabaseResult, StoreDb};

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct OrderArticle {
    pub id: Uuid,
    pub order_id: Uuid,
    pub article_id: Uuid,
    pub quantity: i32,
    pub unit_price: Decimal,
}

impl OrderArticle {
    /// Find `OrderArticle` by order id
    pub async fn find_by_order_id(
        db: &StoreDb,
        order_id: &Uuid,
    ) -> DatabaseResult<Vec<OrderArticle>> {
        sqlx::query_as(r#"SELECT * FROM order_article WHERE order_id = $1"#)
            .bind(order_id)
            .fetch_all(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Insert a new `OrderArticle` record in the database
    pub async fn insert(
        db: &StoreDb,
        order_id: &Uuid,
        article_id: &Uuid,
        quantity: i32,
        unit_price: Decimal,
    ) -> DatabaseResult<Self> {
        let order_article = Self::new(order_id, article_id, quantity, unit_price);
        debug!(
            "inserting a new order_article {} to repository",
            order_article.id
        );
        let rows = sqlx::query(
            "INSERT INTO order_article (id, quantity, unit_price, order_id, article_id) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(order_article.id)
        .bind(order_article.quantity)
        .bind(order_article.unit_price)
        .bind(order_article.order_id)
        .bind(order_article.article_id)
        .execute(db.pool())
        .await
        .map_err(DatabaseError::from)?
        .rows_affected();
        if rows != 1 {
            return Err(DatabaseError::TooManyInserts);
        }

        Ok(order_article)
    }

    fn new(order_id: &Uuid, article_id: &Uuid, quantity: i32, unit_price: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id: order_id.clone(),
            article_id: article_id.clone(),
            quantity,
            unit_price,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::database::{Article, Customer, CustomerOrder};
    use std::env;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_insert_order_article() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_insert_order_article@prima.it", "abcdef")
            .await
            .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();
        let article = insert_article(&db, "panzerotti").await;

        let order_article = OrderArticle::insert(
            &db,
            &order.id,
            &article.id,
            4,
            rust_decimal_macros::dec!(0.8),
        )
        .await
        .unwrap();

        assert_eq!(order_article.article_id, article.id);
        assert_eq!(order_article.order_id, order.id);
        assert_eq!(order_article.quantity, 4);
        assert_eq!(order_article.unit_price, rust_decimal_macros::dec!(0.8));
    }

    #[tokio::test]
    async fn should_find_order_articles_by_order_id() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(
            &db,
            "should_find_order_articles_by_order_id@prima.it",
            "abcdef",
        )
        .await
        .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();
        let panzerotti = insert_article(&db, "panzerotti").await;
        let donut = insert_article(&db, "donut").await;

        OrderArticle::insert(
            &db,
            &order.id,
            &panzerotti.id,
            2,
            rust_decimal_macros::dec!(0.8),
        )
        .await
        .expect("failed to insert article into order");
        OrderArticle::insert(&db, &order.id, &donut.id, 3, rust_decimal_macros::dec!(1.2))
            .await
            .expect("failed to insert article into order");

        assert_eq!(
            OrderArticle::find_by_order_id(&db, &order.id)
                .await
                .unwrap()
                .len(),
            2
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
