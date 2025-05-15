use super::*;
use crate::utils::check_id_slug;
use crate::{
    structs::projects::Project,
    utils::{RequestBuilderCustomSend, UrlJoinAll},
};

impl ModrinthAPI {
    pub async fn get_project_by_id(&self, project_id: &str) -> Result<Project> {
        check_id_slug(&[project_id])?;
        self.client
            .get(BASE_URL.join_all(vec!["project", project_id]))
            .custom_send_json()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_valid_project() -> Result<()> {
        let api = ModrinthAPI::default();
        // HVnmMxH1 -> Complementary Shaders - Reimagined
        let response = api.get_project_by_id("HVnmMxH1").await?;
        assert_eq!(response.title, "Complementary Shaders - Reimagined");
        Ok(())
    }
}
