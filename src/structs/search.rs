use super::*;

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

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Facet {
    // TODO: read #7 in TOOD.md file (structs\projects section)
    // ProjectType(projects::ProjectType),
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
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = match self {
            // Facet::ProjectType(project_type) => {
            //     format!("project_type:{project_type:?}",)
            // }
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
            // _ => "".to_string(),
        };
        serializer.collect_str(&output)
    }
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchHit {
    /// The project's slug, used for vanity URLs.
    pub slug: Option<String>,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    // TODO: read #1 in TOOD.md file (structs\projects section)
    // pub client_side: projects::ProjectSupportRange,
    // pub server_side: projects::ProjectSupportRange,

    // TODO: read #7 in TOOD.md file (structs\projects section)
    // pub project_type: projects::ProjectType,
    pub downloads: usize,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub icon_url: Option<Url>,
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
}
