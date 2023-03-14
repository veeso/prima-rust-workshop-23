//! # GraphQL schema

use super::{
    resolvers::{
        Articles as ArticlesResolver, Orders as OrdersResolver, SubmitOrder as SubmitOrderResolver,
        UNAUTHORIZED,
    },
    types::{Article, Order, OrderArticle},
    GraphqlRequestParams,
};

use async_graphql::{Context, EmptySubscription, Object, Schema};

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn articles<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query: Option<String>,
        page: u64,
        count: u64,
    ) -> async_graphql::Result<Vec<Article>> {
        let resolver = ctx.data_unchecked::<ArticlesResolver>();
        resolver.resolve(query, page, count).await
    }

    async fn orders<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        page: u64,
        count: u64,
    ) -> async_graphql::Result<Vec<Order>> {
        let resolver = ctx.data_unchecked::<OrdersResolver>();
        let request_params = ctx.data_unchecked::<GraphqlRequestParams>();
        if let Some(user_id) = request_params.user_id {
            resolver.resolve(user_id, page, count).await
        } else {
            Err(async_graphql::Error::new(UNAUTHORIZED))
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn submit_order<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        articles: Vec<OrderArticle>,
    ) -> async_graphql::Result<Order> {
        let resolver = ctx.data_unchecked::<SubmitOrderResolver>();
        let request_params = ctx.data_unchecked::<GraphqlRequestParams>();
        if let Some(user_id) = request_params.user_id {
            resolver.resolve(user_id, articles).await
        } else {
            Err(async_graphql::Error::new(UNAUTHORIZED))
        }
    }
}
