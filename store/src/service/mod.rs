//! # Service
//!
//! gRPC service

mod error;
pub mod store {
    tonic::include_proto!("store");
}
use crate::database::{Article, Customer, CustomerOrder, OrderArticle, OrderStatus, StoreDb};
pub use error::ServiceError;
use store::store_service_server::{
    StoreService as ProtobufStoreService, StoreServiceServer as ProtobufStoreServiceServer,
};

use email_address::EmailAddress;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tonic::{transport::Server as GrpcServer, Request, Response, Status};
use uuid::Uuid;

/// Result type for StoreService
pub type StoreResult<T> = Result<T, ServiceError>;

#[derive(Debug)]
pub struct StoreService {
    address: SocketAddr,
    database: StoreDb,
}

impl StoreService {
    /// Configure and initialize store service
    pub async fn configure(listener_address: &str, database_url: &str) -> StoreResult<Self> {
        debug!("parsing address {listener_address}...");
        let address = listener_address
            .parse()
            .map_err(|_| ServiceError::InvalidAddress)?;
        debug!("parsed address {:?}", address);
        debug!("connecting to database at {database_url}");
        let database = StoreDb::connect(database_url).await?;
        info!("store service initialized");
        Ok(Self { address, database })
    }

    /// Run store service server
    pub async fn run(self) -> StoreResult<()> {
        info!("running server...");
        let address = self.address;
        GrpcServer::builder()
            .add_service(ProtobufStoreServiceServer::new(self))
            .serve(address)
            .await?;

        warn!("server terminated!");
        Ok(())
    }

    fn hash_password(&self, s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[tonic::async_trait]
impl ProtobufStoreService for StoreService {
    async fn sign_in(
        &self,
        request: Request<store::SignInRequest>,
    ) -> Result<Response<store::AuthResponse>, Status> {
        let email = &request.get_ref().email;
        let password = &request.get_ref().password;
        let password = self.hash_password(password);
        debug!("got signin request with {email} and {password}");
        // sign in
        let customer =
            Customer::find_by_email_and_password(&self.database, email, &password).await?;

        let status = match customer {
            None => store::auth_response::Status::Error(2),
            Some(customer) => store::auth_response::Status::UserId(customer.id.to_string()),
        };
        Ok(Response::new(store::AuthResponse {
            status: Some(status),
        }))
    }

    async fn sign_up(
        &self,
        request: Request<store::SignUpRequest>,
    ) -> Result<Response<store::AuthResponse>, Status> {
        let email = &request.get_ref().email;
        let password = &request.get_ref().password;
        let password = self.hash_password(password);
        debug!("got signup request with {email} and {password}");
        // validate email
        if !EmailAddress::is_valid(email) {
            return Ok(Response::new(store::AuthResponse {
                status: Some(store::auth_response::Status::Error(1)),
            }));
        }
        // check whether email is already taken
        if Customer::find_by_email(&self.database, email)
            .await?
            .is_some()
        {
            debug!("a user with email {email} already exists");
            return Ok(Response::new(store::AuthResponse {
                status: Some(store::auth_response::Status::Error(0)),
            }));
        }
        // create user
        let customer = Customer::insert(&self.database, email, &password).await?;
        debug!("created new customer with id {}", customer.id);

        Ok(Response::new(store::AuthResponse {
            status: Some(store::auth_response::Status::UserId(
                customer.id.to_string(),
            )),
        }))
    }

    async fn query_orders(
        &self,
        request: Request<store::QueryOrdersRequest>,
    ) -> Result<Response<store::QueryOrdersResult>, Status> {
        let user_id = Uuid::parse_str(&request.get_ref().user_id)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        let page = request.get_ref().page_number as i64;
        let count = request.get_ref().results_per_page as i64;
        debug!("get orders for user {user_id} from {page}; {count} elements");
        let orders = CustomerOrder::find_by_customer(&self.database, &user_id, page, count).await?;
        debug!(
            "got {} orders; collecting articles for orders",
            orders.len()
        );
        let mut orders_with_article = Vec::with_capacity(orders.len());
        for order in orders.into_iter() {
            debug!("collecting articles for order {}", order.id);
            let order_articles = OrderArticle::find_by_order_id(&self.database, &order.id).await?;
            debug!("got {} articles in order", order_articles.len());
            // resolve article type
            let mut articles = Vec::with_capacity(order_articles.len());
            for order_article in order_articles.into_iter() {
                debug!("getting article details for article {}", order_article.id);
                if let Some(article) =
                    Article::find_by_id(&self.database, &order_article.article_id).await?
                {
                    articles.push(store::OrderArticle {
                        id: article.id.to_string(),
                        name: article.name,
                        description: article.description,
                        quantity: order_article.quantity as u32,
                        unit_price: Some(store::Decimal {
                            value: order_article.unit_price.to_string(),
                        }),
                    })
                } else {
                    warn!("could not find any article for {}", order_article.id);
                }
            }
            orders_with_article.push(store::Order {
                id: order.id.to_string(),
                created_at: Some(store::Iso8601 {
                    timestamp: order.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                }),
                transaction_id: order.transaction_id,
                status: match order.status {
                    OrderStatus::Created => 0,
                    OrderStatus::PaymentRefused => 2,
                    OrderStatus::Preparing => 1,
                    OrderStatus::Shipped => 3,
                },
                articles,
            });
        }
        debug!("returning {} orders", orders_with_article.len());
        Ok(Response::new(store::QueryOrdersResult {
            orders: orders_with_article,
        }))
    }

    async fn query_articles(
        &self,
        request: Request<store::QueryArticlesRequest>,
    ) -> Result<Response<store::QueryArticlesResult>, Status> {
        let query = &request.get_ref().query;
        let page = request.get_ref().page_number as i64;
        let count = request.get_ref().results_per_page as i64;
        debug!(
            "getting articles by query '{:?}' from {page}; {count} elements",
            query
        );
        let articles: Vec<store::Article> = match query {
            Some(q) => Article::find_by_name(&self.database, q, page, count).await,
            None => Article::get_all(&self.database, page, count).await,
        }?
        .into_iter()
        .map(|x| store::Article {
            id: x.id.to_string(),
            name: x.name,
            description: x.description,
            unit_price: Some(store::Decimal {
                value: x.unit_price.to_string(),
            }),
        })
        .collect();
        debug!("found {} articles", articles.len());

        Ok(Response::new(store::QueryArticlesResult { articles }))
    }

    async fn submit_order(
        &self,
        request: Request<store::SubmitOrderRequest>,
    ) -> Result<Response<store::SubmitOrderResponse>, Status> {
        let user_id = Uuid::parse_str(&request.get_ref().user_id)
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        let articles = &request.get_ref().articles;
        debug!("submitting order for customer with id {user_id}");
        // start transaction
        let mut transaction = self
            .database
            .pool()
            .begin()
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
        // insert order
        let order = CustomerOrder::insert_order(&mut transaction, &user_id).await?;
        debug!("inserted order with ID {}", order.id.to_string());
        // insert for each article a order-article in the database
        for article in articles.iter() {
            debug!(
                "inserting new article for order {}: {}",
                order.id.to_string(),
                article.article_id
            );
            let article_id = Uuid::parse_str(&article.article_id)
                .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
            // get current unit price for article
            let stock_article = match Article::find_by_id(&self.database, &article_id).await? {
                Some(a) => a,
                None => {
                    return Ok(Response::new(store::SubmitOrderResponse {
                        status: Some(store::submit_order_response::Status::Error(1)),
                    }))
                }
            };
            OrderArticle::insert(
                &mut transaction,
                &order.id,
                &article_id,
                article.quantity as i32,
                stock_article.unit_price,
            )
            .await?;
        }
        debug!("all articles have been stored in the database; committing transaction...");
        transaction
            .commit()
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(store::SubmitOrderResponse {
            status: Some(store::submit_order_response::Status::OrderId(
                order.id.to_string(),
            )),
        }))
    }

    async fn submit_order_payment(
        &self,
        request: Request<store::SubmitOrderPaymentRequest>,
    ) -> Result<Response<store::SubmitOrderResponse>, Status> {
        match &request.get_ref().status {
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "missing status".to_string(),
                ))
            }
            Some(store::submit_order_payment_request::Status::Failed(
                store::submit_order_payment_request::SubmitOrderPaymentFailedRequest { order_id },
            )) => {
                let order_id = Uuid::parse_str(order_id)
                    .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
                debug!("setting order status to PaymentRefused and for order {order_id}");
                CustomerOrder::update_status(
                    &self.database,
                    &order_id,
                    OrderStatus::PaymentRefused,
                )
                .await?;

                Ok(Response::new(store::SubmitOrderResponse {
                    status: Some(store::submit_order_response::Status::OrderId(
                        order_id.to_string(),
                    )),
                }))
            }
            Some(store::submit_order_payment_request::Status::Success(
                store::submit_order_payment_request::SubmitOrderPaymentSucceedRequest {
                    order_id,
                    transaction_id,
                },
            )) => {
                let order_id = Uuid::parse_str(order_id)
                    .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
                debug!("setting order status to Preparing and transaction id to {transaction_id} for order {order_id}");
                // create transaction
                let mut transaction = self
                    .database
                    .pool()
                    .begin()
                    .await
                    .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;
                // update both status and transaction id
                CustomerOrder::update_status(&mut transaction, &order_id, OrderStatus::Preparing)
                    .await?;
                CustomerOrder::update_transaction_id(&mut transaction, &order_id, transaction_id)
                    .await?;
                transaction
                    .commit()
                    .await
                    .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

                Ok(Response::new(store::SubmitOrderResponse {
                    status: Some(store::submit_order_response::Status::OrderId(
                        order_id.to_string(),
                    )),
                }))
            }
        }
    }
}
