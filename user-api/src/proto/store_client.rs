//! # Store client

mod types;
pub mod store {
    tonic::include_proto!("store");
}
pub use self::types::{
    Article, AuthResponse, Order, OrderedArticle, SubmitOrderError, SubmitOrderResponse,
};

use super::ProtobufResult;
use store::store_service_client::StoreServiceClient;
use store::{
    QueryArticlesRequest, QueryOrdersRequest, SignInRequest, SignUpRequest, SubmitOrderRequest,
};

use tonic::transport::Channel;
use uuid::Uuid;

/// Protobuf client with store client
pub struct StoreClient {
    store_client: StoreServiceClient<Channel>,
}

impl StoreClient {
    /// Connect to server
    pub async fn connect(server_url: String) -> ProtobufResult<Self> {
        debug!("connecting to grpc server: {server_url}...");
        let store_client = StoreServiceClient::connect(server_url).await?;
        debug!("established connection to protobuf server");
        Ok(Self { store_client })
    }

    /// Sign in to store; returns user id in case of success
    pub async fn sign_in(&mut self, email: &str, password: &str) -> ProtobufResult<AuthResponse> {
        debug!("trying to sign in with email {email} and password {password}");
        let request = tonic::Request::new(SignInRequest {
            email: email.to_string(),
            password: password.to_string(),
        });
        let response = self.store_client.sign_in(request).await?;
        debug!("sign in request OK");
        Ok(AuthResponse::try_from(response.into_inner())?)
    }

    /// Sign up a new customer into the store
    pub async fn sign_up(&mut self, email: &str, password: &str) -> ProtobufResult<AuthResponse> {
        debug!("trying to sign up with email {email} and password {password}");
        let request = tonic::Request::new(SignUpRequest {
            email: email.to_string(),
            password: password.to_string(),
        });
        let response = self.store_client.sign_up(request).await?;
        debug!("sign up request OK");
        Ok(AuthResponse::try_from(response.into_inner())?)
    }

    /// Query orders for customer
    pub async fn query_orders(
        &mut self,
        user_id: Uuid,
        page_number: u32,
        results_per_page: u32,
    ) -> ProtobufResult<Vec<Order>> {
        debug!("trying collect order for {user_id} from {page_number} to {results_per_page}");
        let request = tonic::Request::new(QueryOrdersRequest {
            user_id: user_id.to_string(),
            page_number,
            results_per_page,
        });
        let response = self
            .store_client
            .query_orders(request)
            .await?
            .into_inner()
            .orders;

        let mut orders = Vec::with_capacity(response.len());
        for order in response.into_iter() {
            orders.push(Order::try_from(order)?);
        }

        debug!("got {} orders", orders.len());
        Ok(orders)
    }

    /// Query articles from store
    pub async fn query_articles(
        &mut self,
        query: Option<String>,
        page_number: u32,
        results_per_page: u32,
    ) -> ProtobufResult<Vec<Article>> {
        debug!(
            "trying collect order for {:?} from {page_number} to {results_per_page}",
            query
        );
        let request = tonic::Request::new(QueryArticlesRequest {
            query,
            page_number,
            results_per_page,
        });
        let response = self
            .store_client
            .query_articles(request)
            .await?
            .into_inner()
            .articles;

        let mut articles = Vec::with_capacity(response.len());
        for article in response.into_iter() {
            articles.push(Article::try_from(article)?);
        }

        debug!("got {} articles", articles.len());
        Ok(articles)
    }

    /// Submit order
    pub async fn submit_order(
        &mut self,
        user_id: Uuid,
        articles: Vec<OrderedArticle>,
    ) -> ProtobufResult<SubmitOrderResponse> {
        debug!(
            "submitting order for {user_id} for {} articles",
            articles.len()
        );
        let request = tonic::Request::new(SubmitOrderRequest {
            articles: articles
                .into_iter()
                .map(|x| store::submit_order_request::OrderArticle {
                    article_id: x.id.to_string(),
                    quantity: x.quantity,
                })
                .collect(),
            user_id: user_id.to_string(),
        });
        let response = self.store_client.submit_order(request).await?.into_inner();

        Ok(SubmitOrderResponse::try_from(response)?)
    }
}
