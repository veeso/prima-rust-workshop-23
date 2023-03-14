//! # Proto
//!
//! Protobuf client

pub mod store {
    tonic::include_proto!("store");
}
mod error;
pub use error::ProtobufError;
use store::store_service_client::StoreServiceClient;
use store::{
    QueryArticlesRequest, QueryOrdersRequest, SignInRequest, SignUpRequest,
    SubmitOrderPaymentRequest, SubmitOrderRequest,
};

use tonic::transport::Channel;

pub type ProtobufResult<T> = Result<T, ProtobufError>;

/// Protobuf client with different service client
pub struct ProtobufClient {
    store_client: StoreServiceClient<Channel>,
}

impl ProtobufClient {
    /// Connect to server
    pub async fn connect(server_url: String) -> ProtobufResult<Self> {
        debug!("connecting to grpc server: {server_url}...");
        let store_client = StoreServiceClient::connect(server_url).await?;
        debug!("established connection to protobuf server");
        Ok(Self { store_client })
    }
}
