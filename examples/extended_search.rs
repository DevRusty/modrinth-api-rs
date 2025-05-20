use modrinth_api::structs::search::{ExtendedSearch, Sort};
use modrinth_api::{Error, ModrinthAPI};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = ModrinthAPI::default();
    let result = api
        .extended_search(
            "xaeros",
            &Sort::Downloads,
            Some(20),
            Some(ExtendedSearch {
                offset: None,   // The offset into the search. Skips this number of results
                facets: vec![], // Facets are an essential concept for understanding how to filter out results.
            }),
        )
        .await?;

    println!("{:#?}", result);
    Ok(())
}
