//! API functions to get project by id/slug

use super::*;
use crate::{
    structs::projects::Project,
    utils::{RequestBuilderCustomSend, UrlJoinAll, check_id_slug},
};

impl ModrinthAPI {
    /// Get information about Project ([Project] struct) by project slug (id)
    ///
    /// # Arguments
    ///
    /// * `project_id` - A project slug/id (`&str`)
    ///
    /// # Returns
    ///
    /// `Result<Project>`:
    /// - `Ok(Project)`: The [`Project`] data.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing.
    pub async fn get_project_by_id(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.client
            .get(BASE_URL.join_all(vec!["project", project_id]))
            .custom_send_json()
            .await
    }
}
