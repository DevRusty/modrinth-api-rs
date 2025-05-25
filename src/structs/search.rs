use super::*;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct ExtendedSearch {
    pub offset: Option<u32>,
    pub facets: Vec<Vec<Facet>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub hits: Vec<SearchHit>,
    /// The number of results that were skipped by the query
    pub offset: usize,
    /// The number of results that were returned by the query
    pub limit: usize,
    /// The total number of results that match the query
    pub total_hits: usize,
}

impl Response {
    pub fn show_hits(&self) {
        self.hits.iter().for_each(|h| println!("{h}"));
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchHit {
    /// The project's slug, used for vanity URLs.
    pub slug: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: projects::ProjectSupportRange,
    pub server_side: projects::ProjectSupportRange,
    pub project_type: projects::ProjectType,
    pub downloads: usize,
    pub icon_url: Option<String>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<usize>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: Option<String>,
    pub monetization_status: Option<projects::MonetizationStatus>,
    pub project_id: String,
    /// Author
    pub author: String,
    /// A list of the project's primary/featured categories
    pub display_categories: Vec<String>,
    #[serde(rename = "versions")]
    /// A list of all the game versions supported by the project
    pub game_versions: Vec<String>,
    pub follows: usize,
    pub date_created: Date,
    pub date_modified: Date,
    /// The latest game version that this project supports
    pub latest_version: String,
    /// The SPDX license ID of a project
    pub license: String,
    pub gallery: Vec<Url>,
    pub featured_gallery: Option<Url>,

    #[serde(skip)]
    pub project_info: Option<Project>,
}

impl SearchHit {
    /// Fetches the full project details for this search hit from the Modrinth API.
    ///
    /// A `SearchHit` provides a summarized view of a project. This asynchronous method
    /// retrieves the complete [`Project`] data from the Modrinth API using the
    /// [`self.project_id`] and populates the [`self.project_info`] field.
    ///
    /// This method consumes `self` and returns a modified `SearchHit` instance
    /// with the `project_info` field updated.
    ///
    /// # Arguments
    ///
    /// * `api` - A reference to the [`ModrinthAPI`] client instance, used to perform the API request.
    ///
    /// # Returns
    ///
    /// `Result<Self>`:
    /// - `Ok(self)`: The updated `SearchHit` instance with `self.project_info` populated
    ///   with the full [`Project`] data.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing,
    ///   e.g., network issues, invalid project ID, rate limiting, or API response errors.
    ///
    pub async fn fetch_project(mut self, api: &ModrinthAPI) -> Result<Self> {
        let result = api.get_project_by_id(self.project_id.as_str()).await?;
        self.project_info = Some(result);
        Ok(self)
    }

    /// Retrieves the complete [`Project`] data for this search hit from the Modrinth API.
    ///
    /// This asynchronous method fetches the full [`Project`] object corresponding to
    /// the [`self.project_id`] without modifying the `SearchHit` instance itself.
    /// This is useful when you only need the full project data temporarily or in a
    /// context where `SearchHit` ownership is not desired.
    ///
    /// # Arguments
    ///
    /// * `api` - A reference to the [`ModrinthAPI`] client instance, used to perform the API request.
    ///
    /// # Returns
    ///
    /// `Result<Project>`:
    /// - `Ok(Project)`: The full [`Project`] data.
    /// - `Err(crate::error::Error)`: An error occurred during the API request or data processing.
    pub async fn get_full_project(&self, api: &ModrinthAPI) -> Result<Project> {
        api.get_project_by_id(self.project_id.as_str()).await
    }
}

impl Display for SearchHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {} ({} downloads)\nAuthor: {}\nCategories: {}",
            self.title,
            self.downloads,
            self.author,
            self.display_categories.join(",")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Relevance,
    /// Sorts matches by downloads
    Downloads,
    /// Sorts matches by followers
    Follows,
    /// Sorts by the time of initial creation
    Newest,
    /// Sorts by the time of the latest update
    Updated,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Facet {
    ProjectType(projects::ProjectType),
    /// Mod loader or category to filter
    Categories(String),
    /// Game versions to filter
    Versions(String),
    OpenSource(bool),
    /// License ID to filter
    License(String),
    /// Title
    Title(String),
    /// Author
    Author(String),
    /// Project ID
    ProjectID(String),
    Custom {
        /// The type of metadata to filter
        _type: String,
        /// The comparison to use
        ///
        /// Can be `=`/`:`, `!=`, `>`, `>=`, `<`, `<=`
        operation: String,
        /// The value to compare against
        value: String,
    },
}

impl Serialize for Facet {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = match self {
            Facet::ProjectType(project_type) => {
                format!("project_type:{project_type:?}",)
            }
            Facet::Categories(category) => format!("categories: {category}"),
            Facet::Versions(version) => format!("versions: {version}"),
            Facet::OpenSource(bool) => format!("open_source: {bool}"),
            Facet::License(license_id) => format!("license: {license_id}"),
            Facet::Title(title) => format!("title: {title}"),
            Facet::Author(author) => format!("author: {author}"),
            Facet::ProjectID(project_id) => format!("project_id: {project_id}"),
            Facet::Custom {
                _type,
                operation,
                value,
            } => format!("{_type} {operation} {value}"),
        };
        serializer.collect_str(&output)
    }
}

impl Display for Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = match self {
            Facet::ProjectType(project_type) => {
                format!("project_type:{project_type:?}",)
            }
            Facet::Categories(category) => format!("categories: {category}"),
            Facet::Versions(version) => format!("versions: {version}"),
            Facet::OpenSource(bool) => format!("open_source: {bool}"),
            Facet::License(license_id) => format!("license: {license_id}"),
            Facet::Title(title) => format!("title: {title}"),
            Facet::Author(author) => format!("author: {author}"),
            Facet::ProjectID(project_id) => format!("project_id: {project_id}"),
            Facet::Custom {
                _type,
                operation,
                value,
            } => format!("{_type} {operation} {value}"),
        };
        write!(f, "{}", str)
    }
}
