use modrinth_api::structs::search::Sort;
use modrinth_api::{Error, ModrinthAPI};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = ModrinthAPI::default();
    let result = api.search("xaeros", &Sort::Downloads, None).await?;

    println!("{:#?}", result);
    Ok(())
}
