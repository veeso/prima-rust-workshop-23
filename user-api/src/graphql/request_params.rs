//! # GraphQlRequest params

use uuid::Uuid;

pub struct GraphqlRequestParams {
    pub user_id: Option<Uuid>,
}
