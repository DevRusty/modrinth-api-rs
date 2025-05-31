use modrinth_api::ModrinthAPI;
use modrinth_api::structs::versions::{ProjectVersionParams, ProjectVersionsFilter};

#[tokio::test]
async fn get_version_from_project() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let res = api.get_project_versions("AANobbMI", None).await?;

    let result = res.first();
    assert!(result.is_some());
    assert!(result.unwrap().name.contains("Sodium")); // assume that is Sodium...
    Ok(())
}

#[tokio::test]
async fn get_version_from_project_extra() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let options = ProjectVersionsFilter {
        loaders: Some(&["fabric"]),
        game_versions: Some(&["1.20.1"]),
        featured: Some(true),
    };
    let res = api.get_project_versions("AANobbMI", Some(options)).await?;

    let result = res.first();
    assert!(result.is_some());
    assert!(result.unwrap().name.contains("Sodium")); // assume that is Sodium...
    Ok(())
}

#[tokio::test]
async fn get_single_version_from_project_extra() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let options = ProjectVersionParams {
        number: Some("mc1.20.1-0.5.13-fabric"),
        loaders: Some(&["fabric"]),
        game_versions: Some(&["1.20.1"]),
        featured: Some(true),
    };
    let result = api.get_project_version("AANobbMI", Some(options)).await?;

    assert!(result.name.contains("Sodium")); // assume that is Sodium...
    Ok(())
}

#[tokio::test]
async fn get_single_version_from_project_with_wrong_id() -> modrinth_api::Result<()> {
    let api = ModrinthAPI::default();
    let options = ProjectVersionParams {
        number: Some("2"),
        loaders: Some(&["fabric"]),
        game_versions: Some(&["1.20.1"]),
        featured: Some(true),
    };
    let result = api.get_project_version("AANobbMI", Some(options)).await;

    assert!(result.is_err());
    Ok(())
}
