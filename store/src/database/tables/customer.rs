use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use super::{DatabaseError, DatabaseResult, StoreDb};

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Customer {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl Customer {
    /// Insert new `Customer` to database
    pub async fn insert(
        db: &StoreDb,
        email: impl ToString,
        password: impl ToString,
    ) -> DatabaseResult<Self> {
        let customer = Customer::new(email, password);
        debug!("inserting a new customer {} to repository", customer.id);
        let rows = sqlx::query(
            "INSERT INTO customer (id, email, password, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(customer.id)
        .bind(&customer.email)
        .bind(&customer.password)
        .bind(&customer.created_at)
        .execute(db.pool())
        .await
        .map_err(DatabaseError::from)?
        .rows_affected();
        if rows != 1 {
            return Err(DatabaseError::TooManyInserts);
        }

        Ok(customer)
    }

    /// Find `Customer` by `id`
    pub async fn find_by_id(db: &StoreDb, id: &Uuid) -> DatabaseResult<Option<Customer>> {
        sqlx::query_as(r#"SELECT * FROM customer WHERE id = $1"#)
            .bind(id)
            .fetch_optional(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Find `Customer` by `email`
    pub async fn find_by_email_and_password(
        db: &StoreDb,
        email: &str,
        password: &str,
    ) -> DatabaseResult<Option<Customer>> {
        sqlx::query_as(r#"SELECT * FROM customer WHERE email = $1 AND password = $2"#)
            .bind(email)
            .bind(password)
            .fetch_optional(db.pool())
            .await
            .map_err(DatabaseError::from)
    }

    /// Instantiates a new Customer
    fn new(email: impl ToString, password: impl ToString) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::env;

    #[tokio::test]
    async fn should_create_new_user() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");
        let new_customer = Customer::insert(&db, "bigluca@biglucainternational.com", "password123")
            .await
            .unwrap();
        assert_eq!(
            new_customer.email.as_str(),
            "bigluca@biglucainternational.com"
        );
        assert_eq!(new_customer.password.as_str(), "password123");
    }

    #[tokio::test]
    async fn should_get_user_by_email() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");
        let new_customer = Customer::insert(&db, "christian.visintin1997@gmail.com", "password123")
            .await
            .unwrap();
        let christian = Customer::find_by_email_and_password(
            &db,
            "christian.visintin1997@gmail.com",
            "password123",
        )
        .await
        .unwrap();
        assert_eq!(new_customer.id, christian.unwrap().id);
    }

    #[tokio::test]
    async fn should_not_find_any_user() {
        let db = StoreDb::connect(&env::var("DATABASE_URL").expect("DATABASE_URL not found"))
            .await
            .expect("failed to connect to database");
        assert!(
            Customer::find_by_email_and_password(&db, "test@prima.it", "abcdef")
                .await
                .unwrap()
                .is_none()
        );
    }
}
