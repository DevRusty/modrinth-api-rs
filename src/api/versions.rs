//! API functions related to files (versions)

use super::*;
use crate::{
    structs::versions::*,
    utils::{RequestBuilderCustomSend, UrlJoinAll, UrlWithQuery, check_id_slug},
};

impl ModrinthAPI {
    /// Retrieves a list of project versions, optionally filtered, or a single specific version.
    ///
    /// # Arguments
    /// * `project_id` - Project slug/id (`&str`).
    /// * `extra_options` - `Option<ExtraOptions<'_>>`: Optional parameters to filter the list of versions
    ///   or to retrieve a specific version.
    ///
    ///   If `extra_options` is `None`, all versions for the project will be returned without any filters.
    ///
    ///   Fields of `ExtraOptions`:
    ///   - `number`: `Option<&str>` - If `Some`, this should be the specific version ID or a version number string (e.g., "1.0.0").
    ///     When `number` is provided, the function attempts to fetch that specific version. In this case,
    ///     the other filtering parameters (`loaders`, `game_versions`, `featured`) within `ExtraOptions`
    ///     will be ignored by the Modrinth API, as the request targets a single version endpoint
    ///     (`/project/{id}/version/{number}`). The returned `Vec<Version>` will contain at most one element.
    ///     Example: `number: Some("0.76.0+1.19.2")`
    ///
    ///   - `loaders`: `Option<&[&str]>` - A slice of loader IDs (e.g., `&["forge", "fabric"]`)
    ///     to filter the list of versions. This is only applied if `number` is `None`.
    ///     Example: `loaders: Some(&["fabric", "quilt"])`
    ///
    ///   - `game_versions`: `Option<&[&str]>` - A slice of game version IDs (e.g., `&["1.19.2", "1.20.1"]`)
    ///     to filter the list of versions. This is only applied if `number` is `None`.
    ///     Example: `game_versions: Some(&["1.20.1"])`
    ///
    ///   - `featured`: `Option<bool>` - If `Some(true)`, only featured versions will be returned.
    ///     If `Some(false)`, featured versions will be excluded. If `None`, both featured and
    ///     non-featured versions are included. This is only applied if `number` is `None`.
    ///     Example: `featured: Some(true)`
    ///
    ///   Example usage of `extra_options`:
    ///   ```no_run
    ///   # use modrinth_api::structs::versions::ExtraOptions;
    ///   let options_for_filtered_list = ExtraOptions {
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
        extra_options: Option<ExtraOptions<'_>>,
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
    pub async fn get_version(&self, version_id: &str) -> Result<Version> {
        check_id_slug(&[version_id])?;
        self.client
            .get(BASE_URL.join_all(vec!["version", version_id]))
            .custom_send_json()
            .await
    }

    //
    pub async fn version_list_filtered(
        &self,
        project_id: &str,
        loaders: Option<&[&str]>,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Vec<Version>> {
        check_id_slug(&[project_id])?;
        let mut url = BASE_URL.join_all(vec!["project", project_id, "version"]);
        if let Some(loaders) = loaders {
            url = url.with_query_json("loaders", loaders)?;
        }
        if let Some(game_versions) = game_versions {
            url = url.with_query_json("game_versions", game_versions)?;
        }
        if let Some(featured) = featured {
            url = url.with_query_json("featured", featured)?;
        }

        println!("{url}");
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
        let options = ExtraOptions {
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
}
