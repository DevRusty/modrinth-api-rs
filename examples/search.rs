use modrinth_api::structs::search::{ExtendedSearch, Sort};
use modrinth_api::{Error, ModrinthAPI};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = ModrinthAPI::default();
    let result = api
        .extended_search(
            "xaeros",         // Query
            &Sort::Downloads, // Sort
            Some(20),         // Maximum number of results to return.
            Some(ExtendedSearch {
                offset: None,   // The offset into the search. Skips this number of results
                facets: vec![], // Facets are an essential concept for understanding how to filter out results.
            }),
        )
        .await?;

    println!("{:#?}", result);
    Ok(())
}
