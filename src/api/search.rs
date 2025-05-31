//! API functions to search items by query

use super::*;
use crate::structs::search::{ExtendedSearch, Response, Sort};
use crate::utils::{RequestBuilderCustomSend, UrlJoinAll, UrlWithQuery};

impl ModrinthAPI {
    /// Performs an extended search for projects on Modrinth, allowing for more granular control over the search
    /// results through various filtering options.
    ///
    /// This function builds upon the basic search by introducing `offset` and `facets`
    /// for more refined queries.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string.
    /// * `sort` - The sorting criteria for the search results (e.g., by downloads, recency).
    /// * `limit` - An optional maximum number of results to return. Defaults to 20 if `None`.
    /// * `extended_search` - A struct containing additional search parameters:
    ///     * `offset` - An optional starting point for the results. Defaults to 20 if `None`.
    ///     * `facets` - A vector of vectors of `Facet` enums, allowing for complex filtering.
    ///       Each inner vector represents a group of facets, where results must match
    ///       at least one facet from each inner group.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// * `Ok(Response)` - On successful retrieval of search results. The `Response` struct
    ///   contains a list of `Hit` objects representing the found projects.
    /// * `Err(Error)` - If an error occurs during the API call or data processing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use modrinth_api::{
    ///     ModrinthAPI, structs::search::{Sort, ExtendedSearch, Facet, Response}
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let api = ModrinthAPI::default();
    ///
    ///     // Example: Search for "fabric" mods, sorted by downloads,
    ///     // and filter for projects compatible with "1.19.2" and "Forge" loader.
    ///     let extended_search_params = ExtendedSearch {
    ///         offset: Some(0),
    ///         facets: vec![
    ///             vec![Facet::Versions("1.19.2".to_string())],
    ///             vec![Facet::Categories("fabric".to_string())],
    ///         ],
    ///     };
    ///
    ///     let response: Response = api.extended_search(
    ///         "fabric",
    ///         &Sort::Downloads,
    ///         Some(10),
    ///         Some(extended_search_params),
    ///     ).await?;
    ///
    ///     for hit in response.hits {
    ///         println!("Project: {}", hit.title);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn search(
        &self,
        query: &str,
        sort: &Sort,
        limit: Option<u32>,
        extended_search: Option<ExtendedSearch>,
    ) -> Result<Response> {
        let limit = limit.unwrap_or(20);

        let extended_search = extended_search.unwrap_or_default();
        let offset = &extended_search.offset.unwrap_or(0);
        let mut facets = extended_search.facets;

        let mut url = BASE_URL
            .join_all(vec!["search"])
            .with_query("query", query)
            .with_query("index", sort)
            .with_query("limit", limit)
            .with_query("offset", offset);

        facets.retain(|e| !e.is_empty());
        if !facets.is_empty() {
            url = url.with_query_json("facets", facets)?
        }

        self.client.get(url).custom_send_json().await
    }

    /// Performs an extended search for projects on Modrinth, allowing for more granular control over the search
    /// results through various filtering options.
    ///
    /// This function is a reference to [ModrinthAPI::search] function (backward compatibility)
    #[deprecated(
        since = "0.2.0",
        note = "This function is a backward compatibility with older versions of modrinth-api-rs. Please use ModrinthAPI::search instead"
    )]
    pub async fn extended_search(
        &self,
        query: &str,
        sort: &Sort,
        limit: Option<u32>,
        extended_search: Option<ExtendedSearch>,
    ) -> Result<Response> {
        self.search(query, sort, limit, extended_search).await
    }
}
