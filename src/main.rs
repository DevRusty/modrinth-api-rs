use modrinth_api::{Error, ModrinthAPI};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = ModrinthAPI::default();
    let result = api.get_project_by_id("HVnmMxH1").await?;

    println!("{:#?}", result);
    Ok(())
}
