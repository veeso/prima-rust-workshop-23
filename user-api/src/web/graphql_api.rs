use super::SessionClient;
use crate::graphql::{
    resolvers::{
        Articles as ArticlesResolver, Orders as OrdersResolver, SubmitOrder as SubmitOrderResolver,
    },
    schema::{ApiSchema, MutationRoot, QueryRoot},
    GraphqlRequestParams,
};

use actix_session::Session;
use actix_web::{web, web::Data, Resource};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub fn service_factory(
    protobuf_url: &str,
) -> Resource<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
        Config = (),
        InitError = (),
    >,
> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(ArticlesResolver::new(protobuf_url))
        .data(OrdersResolver::new(protobuf_url))
        .data(SubmitOrderResolver::new(protobuf_url))
        .finish();

    web::resource("/graphql")
        .route(web::post().to(graphql_action))
        .route(web::get().to(graphql_action))
        .app_data(Data::new(schema))
}

async fn graphql_action(
    schema: Data<ApiSchema>,
    req: GraphQLRequest,
    session: Session,
) -> GraphQLResponse {
    let session = SessionClient::from(session);
    let user_id = session.get_user().map(|x| x.id);
    let graphql_request_params = GraphqlRequestParams { user_id };

    schema
        .execute(req.into_inner().data(graphql_request_params))
        .await
        .into()
}
