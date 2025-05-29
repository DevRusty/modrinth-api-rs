//! API functions related to files (versions)

use super::*;
use crate::{
    structs::versions::*,
    utils::{RequestBuilderCustomSend, UrlJoinAll, UrlWithQuery, check_id_slug},
};

impl ModrinthAPI {
    /// Retrieves a list of project versions with custom filtering.
    ///
    /// # Arguments
    /// * `project_id` - Project slug/id (`&str`).
    /// * `extra_options` - `Option<ProjectVersionsFilter<'_>>` ([`ProjectVersionsFilter`]): Optional parameters to filter the list of versions
    ///   or to retrieve a specific version.
    ///
    ///   If `extra_options` is `None`, all versions for the project will be returned without any filters.
    ///
    ///   Example usage of `extra_options`:
    ///   ```no_run
    ///   use modrinth_api::structs::versions::ProjectVersionsFilter;
    ///
    ///   let options_for_filtered_list = ProjectVersionsFilter {
    ///       loaders: Some(&["fabric"]),
    ///       game_versions: Some(&["1.20.1"]),
    ///       featured: Some(true),
    ///   };
    ///   ```
    ///
    /// # Returns
    /// `Result<Vec<Version>>`:
    /// - `Ok(Vec<Version>)`: A list of the [`Version`] structs. If `number` was specified in `extra_options`,
    ///   this vector will contain at most one [`Version`] struct.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing.
    pub async fn get_project_versions(
        &self,
        project_id: &str,
        extra_options: Option<ProjectVersionsFilter<'_>>,
    ) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;

        let mut url = BASE_URL.join_all(vec!["project", project_id, "version"]);

        match extra_options {
            Some(extra_options) => {
                url = BASE_URL.join_all(vec![
                    "project", project_id, "version",
                    // extra_options.number.unwrap_or(""),
                ]);
                url = url.add_optional_query_json("loaders", extra_options.loaders)?;
                url = url.add_optional_query_json("game_versions", extra_options.game_versions)?;
                url = url.add_optional_query_json("featured", extra_options.featured)?;
            }
            None => {}
        }

        self.client.get(url).custom_send_json().await
    }

    /// Get the version from the version id
    ///
    /// # Arguments
    /// * `version_id` - The ID of the version (`&str`)
    ///
    /// # Returns
    ///
    /// `Result<Version>`:
    /// - `Ok(Version)`: [`Version`] struct.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing.
    pub async fn get_version_by_id(&self, version_id: &str) -> Result<Version> {
        check_id_slug(&[version_id])?;
        self.client
            .get(BASE_URL.join_all(vec!["version", version_id]))
            .custom_send_json()
            .await
    }

    /// Retrieves a project [`Version`] with custom filtering.
    ///
    /// # Arguments
    /// * `project_id` - Project slug/id (`&str`).
    /// * `extra_options` - `Option<ProjectVersionParams<'_>>`: Optional parameters to filter the list of versions
    ///   or to retrieve a specific version.
    ///
    ///   If `extra_options` is `None`, all versions for the project will be returned without any filters.
    ///
    ///   Example usage of `extra_options`:
    ///   ```no_run
    ///   use modrinth_api::structs::versions::ProjectVersionParams;
    ///   
    ///   let options_for_filtered_list = ProjectVersionParams {
    ///       number: Some("mc1.20.1-0.5.13-fabric"), // or Some("OihdIimA")
    ///       loaders: Some(&["fabric"]),
    ///       game_versions: Some(&["1.20.1"]),
    ///       featured: Some(true),
    ///   };
    ///   ```
    ///
    /// # Returns
    /// `Result<Vec<Version>>`:
    /// - `Ok(Vec<Version>)`: A list of the [`Version`] structs. If `number` was specified in `extra_options`,
    ///   this vector will contain at most one [`Version`] struct.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing.
    pub async fn get_project_version(
        &self,
        project_id: &str,
        extra_options: Option<ProjectVersionParams<'_>>,
    ) -> Result<Version> {
        check_id_slug(&[project_id])?;

        let mut url = BASE_URL.join_all(vec!["project", project_id, "version"]);

        match extra_options {
            Some(extra_options) => {
                url = BASE_URL.join_all(vec![
                    "project",
                    project_id,
                    "version",
                    extra_options.number.unwrap(),
                ]);
                url = url.add_optional_query_json("loaders", extra_options.loaders)?;
                url = url.add_optional_query_json("game_versions", extra_options.game_versions)?;
                url = url.add_optional_query_json("featured", extra_options.featured)?;
            }
            None => {}
        }

        self.client.get(url).custom_send_json().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_version_from_project() -> Result<()> {
        let api = ModrinthAPI::default();
        let res = api.get_project_versions("AANobbMI", None).await?;

        let result = res.first();
        assert!(result.is_some());
        assert!(result.unwrap().name.contains("Sodium")); // assume that is Sodium...
        Ok(())
    }

    #[tokio::test]
    async fn get_version_from_project_extra() -> Result<()> {
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
    async fn get_single_version_from_project_extra() -> Result<()> {
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
    async fn get_single_version_from_project_with_wrong_id() -> Result<()> {
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
}
