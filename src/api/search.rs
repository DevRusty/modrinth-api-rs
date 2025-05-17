use super::*;
use crate::structs::search::{Facet, Response, Sort};
use crate::utils::UrlWithQuery;
use crate::utils::{RequestBuilderCustomSend, UrlJoinAll};

impl ModrinthAPI {
    pub async fn extended_search(
        &self,
        query: &str,
        sort: &Sort,
        limit: Option<u32>,
        offset: Option<u32>,
        mut facets: Vec<Vec<Facet>>,
    ) -> Result<Response> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(20);

        let mut url = BASE_URL
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort)
            .with_query("limit", &limit)
            .with_query("offset", &offset);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.client.get(url).custom_send_json().await
    }

    pub async fn search(&self, query: &str, sort: &Sort, limit: Option<u32>) -> Result<Response> {
        let limit = limit.unwrap_or(20);

        let url = BASE_URL
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort)
            .with_query("limit", &limit);

        self.client.get(url).custom_send_json().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn search_project() -> Result<()> {
        let api = ModrinthAPI::default();
        let response = api.search("xaeros", &Sort::Downloads, None).await?;

        let title = response.hits.first().unwrap().slug.as_ref();

        assert!(title.is_some());
        assert_eq!(title.unwrap(), "xaeros-minimap");
        Ok(())
    }
}
