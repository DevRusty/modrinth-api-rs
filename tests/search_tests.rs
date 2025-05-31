use modrinth_api::ModrinthAPI;
use modrinth_api::structs::projects::ProjectType;
use modrinth_api::structs::search::{ExtendedSearch, Facet, Sort};

#[tokio::test]
async fn get_valid_project() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    // HVnmMxH1 -> Complementary Shaders - Reimagined
    let response = api.get_project_by_id("HVnmMxH1").await?;
    assert_eq!(response.title, "Complementary Shaders - Reimagined");
    Ok(())
}

#[tokio::test]
async fn asrt_slug_error() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let response = api.get_project_by_id("dffdsfdsfsdfdsf").await;
    assert!(response.is_err());
    Ok(())
}

#[tokio::test]
async fn search_project() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let response = api
        .search(
            "xaeros",
            &Sort::Downloads,
            None,
            Some(ExtendedSearch {
                offset: None,
                facets: vec![vec![Facet::ProjectType(ProjectType::Mod)]],
            }),
        )
        .await?;

    let response = response.hits.first().unwrap();
    let title = response.slug.as_ref();

    assert!(title.is_some());
    assert_eq!(title.ok_or(0), Ok(&String::from("xaeros-minimap")));
    Ok(())
}

#[tokio::test]
async fn test_fetching_project_with_mut() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let response = api
        .search(
            "xaeros",
            &Sort::Downloads,
            None,
            Some(ExtendedSearch {
                offset: None,
                facets: vec![vec![Facet::ProjectType(ProjectType::Mod)]],
            }),
        )
        .await?;

    let response = response.hits.first().unwrap();
    let title = response.slug.as_ref();

    assert!(title.is_some());
    assert_eq!(title.ok_or(0), Ok(&String::from("xaeros-minimap")));

    let hit = response.to_owned().fetch_project(&api).await?;

    assert_eq!(hit.slug.as_ref().unwrap(), &String::from("xaeros-minimap"));
    Ok(())
}

#[tokio::test]
async fn test_fetching_project_without_mut() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let response = api
        .search(
            "xaeros",
            &Sort::Downloads,
            None,
            Some(ExtendedSearch {
                offset: None,
                facets: vec![vec![Facet::ProjectType(ProjectType::Mod)]],
            }),
        )
        .await?;

    let response = response.hits.first().unwrap();
    let title = response.slug.as_ref();

    assert!(title.is_some());
    assert_eq!(title.ok_or(0), Ok(&String::from("xaeros-minimap")));

    let hit = response.get_full_project(&api).await?;

    assert_eq!(hit.slug, String::from("xaeros-minimap"));
    Ok(())
}
