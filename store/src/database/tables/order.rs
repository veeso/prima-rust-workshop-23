use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use super::{DatabaseError, DatabaseResult, StoreDb};

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct CustomerOrder {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub created_at: NaiveDateTime,
    pub status: OrderStatus,
    pub transaction_id: Option<String>,
}

/// Order status
#[derive(Debug, Clone, Copy, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Created,
    Preparing,
    PaymentRefused,
    Shipped,
}

impl CustomerOrder {
    /// Find `Order` by `id`
    pub async fn find_by_id(db: &StoreDb, id: &Uuid) -> DatabaseResult<Option<CustomerOrder>> {
        sqlx::query_as(r#"SELECT * FROM customer_order WHERE id = $1"#)
            .bind(id)
            .fetch_optional(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Find `Order` by customer id
    pub async fn find_by_customer(
        db: &StoreDb,
        customer_id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> DatabaseResult<Vec<CustomerOrder>> {
        sqlx::query_as(r#"SELECT * FROM customer_order WHERE customer_id = $1 OFFSET $2 LIMIT $3"#)
            .bind(customer_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Insert a new order in the database
    pub async fn insert_order(
        db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
        customer_id: &Uuid,
    ) -> DatabaseResult<Self> {
        let order = Self::new(customer_id);
        debug!("inserting a new order {} to repository", order.id);
        let rows = sqlx::query(
            "INSERT INTO customer_order (id, customer_id, created_at, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.customer_id)
        .bind(order.created_at)
        .bind(order.status)
        .execute(db)
        .await
        .map_err(DatabaseError::from)?
        .rows_affected();
        if rows != 1 {
            return Err(DatabaseError::TooManyInserts);
        }

        Ok(order)
    }

    /// Update order status
    pub async fn update_status(
        db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
        order_id: &Uuid,
        status: OrderStatus,
    ) -> DatabaseResult<()> {
        debug!("updating status to {:?} to rder {order_id} ", status);
        let rows = sqlx::query("UPDATE customer_order SET status = $1 WHERE id = $2")
            .bind(status)
            .bind(order_id)
            .execute(db)
            .await
            .map_err(DatabaseError::from)?
            .rows_affected();
        if rows != 1 {
            return Err(DatabaseError::TooManyInserts);
        }

        Ok(())
    }

    pub async fn update_transaction_id(
        db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
        order_id: &Uuid,
        transaction_id: &str,
    ) -> DatabaseResult<()> {
        debug!("updating transaction_id to {transaction_id} to rder {order_id} ");
        let rows = sqlx::query("UPDATE customer_order SET transaction_id = $1 WHERE id = $2")
            .bind(transaction_id)
            .bind(order_id)
            .execute(db)
            .await
            .map_err(DatabaseError::from)?
            .rows_affected();
        if rows != 1 {
            return Err(DatabaseError::TooManyInserts);
        }

        Ok(())
    }

    fn new(customer_id: &Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            customer_id: *customer_id,
            created_at: Utc::now().naive_utc(),
            status: OrderStatus::Created,
            transaction_id: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::Customer;

    use pretty_assertions::assert_eq;
    use std::env;

    #[tokio::test]
    async fn should_create_order() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_create_order@prima.it", "abcdef")
            .await
            .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();
        assert_eq!(order.customer_id, customer.id);
    }

    #[tokio::test]
    async fn should_update_order_status() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_update_order_status@prima.it", "abcdef")
            .await
            .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();

        assert!(
            CustomerOrder::update_status(&db, &order.id, OrderStatus::Shipped)
                .await
                .is_ok()
        );

        assert_eq!(
            CustomerOrder::find_by_id(&db, &order.id)
                .await
                .unwrap()
                .unwrap()
                .status,
            OrderStatus::Shipped
        );
    }

    #[tokio::test]
    async fn should_update_transaction_id() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_update_transaction_id@prima.it", "abcdef")
            .await
            .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();

        assert!(
            CustomerOrder::update_transaction_id(&db, &order.id, "dummy")
                .await
                .is_ok()
        );

        assert_eq!(
            CustomerOrder::find_by_id(&db, &order.id)
                .await
                .unwrap()
                .unwrap()
                .transaction_id
                .as_deref()
                .unwrap(),
            "dummy"
        );
    }

    #[tokio::test]
    async fn should_find_order_by_id() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_find_order_by_id@prima.it", "abcdef")
            .await
            .unwrap();
        let order = CustomerOrder::insert_order(&db, &customer.id)
            .await
            .unwrap();

        assert_eq!(
            order,
            CustomerOrder::find_by_id(&db, &order.id)
                .await
                .unwrap()
                .unwrap()
        );
    }

    #[tokio::test]
    async fn should_find_orders_by_customer() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");

        let customer = Customer::insert(&db, "should_find_orders_by_customer@prima.it", "abcdef")
            .await
            .unwrap();
        let customer_2 =
            Customer::insert(&db, "should_find_orders_by_customer2@prima.it", "abcdef")
                .await
                .expect("failed to insert customer");

        CustomerOrder::insert_order(&db, &customer.id)
            .await
            .expect("failed to insert order");
        CustomerOrder::insert_order(&db, &customer.id)
            .await
            .expect("failed to insert order");

        CustomerOrder::insert_order(&db, &customer_2.id)
            .await
            .expect("failed to insert order");

        assert_eq!(
            CustomerOrder::find_by_customer(&db, &customer.id, 0, 256)
                .await
                .unwrap()
                .len(),
            2
        );
    }
}
