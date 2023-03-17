use crate::{graphql::types::Article, proto::StoreClient};

/// Articles query
pub struct Articles {
    store_server_url: String,
}

impl Articles {
    /// Instantiates a new `Articles`
    pub fn new(store_server_url: &str) -> Self {
        Self {
            store_server_url: store_server_url.to_string(),
        }
    }

    /// Resolve query articles
    pub async fn resolve(
        &self,
        query: Option<String>,
        page: u32,
        count: u32,
    ) -> async_graphql::Result<Vec<Article>> {
        let mut client = StoreClient::connect(self.store_server_url.clone()).await?;
        let articles = client
            .query_articles(query, page, count)
            .await?
            .into_iter()
            .map(Article::from)
            .collect();

        Ok(articles)
    }
}
