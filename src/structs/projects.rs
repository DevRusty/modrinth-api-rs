//! Models related to projects
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/projects/)

use super::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// The slug of a project, used for vanity URLs
    pub slug: String,
    /// Title of the project
    pub title: String,
    /// A short description of the project
    pub description: String,
    /// A list of the categories that the project has
    pub categories: Vec<String>,
    /// The client side support of the project
    pub client_side: String, // TODO: read #1 in TOOD.md file (structs\projects section)
    /// The server side support of the project
    pub server_side: String, // TODO: read #1 in TODO.md file (structs\projects section)
    /// A long form description of the project
    pub body: String,
    /// The status of the project
    ///
    /// TODO: read #2 in TODO.md file (structs\projects section)
    pub status: String,
    /// The requested status when submitting for review or scheduling the project for release
    ///
    /// TODO: read #3 in TODO.md file (structs\projects section)
    pub requested_status: Option<String>,
    /// A list of categories which are searchable but non-primary
    pub additional_categories: Vec<String>,
    /// An optional link to where to submit bugs or issues with the project
    ///
    /// TODO: read #4 in TODO.md file (structs\projects section)
    pub issues_url: Option<String>,
    /// An optional link to the source code of the project
    ///
    /// TODO: read #4 in TODO.md file (structs\projects section)
    pub source_url: Option<String>,
    /// An optional link to the project’s wiki page or other relevant information
    ///
    /// TODO: read #4 in TODO.md file (structs\projects section)
    pub wiki_url: Option<String>,
    /// An optional invite link to the project’s discord
    ///
    /// TODO: read #4 in TODO.md file (structs\projects section)
    pub discord_url: Option<String>,
    /// Donation links
    ///
    /// TODO: read #5 in TODO.md file (structs\projects section)
    #[serde(skip)]
    pub donation_links: Option<Vec<String>>,
    pub project_type: String,
    pub downloads: usize,
    /// TODO: read #4 in TODO.md file (structs\projects section)
    pub icon_url: Option<String>,
    /// The RGB color of the project, automatically generated from the project icon
    pub color: Option<usize>,
    /// The ID of the moderation thread associated with this project
    pub thread_id: String,
    pub monetization_status: MonetizationStatus,
    pub id: String,
    /// The ID of the team that has ownership of this project
    pub team: String,
    pub published: Date,
    pub updated: Date,
    /// The date the project's status was approved
    pub approved: Option<Date>,
    pub queued: Option<Date>,
    pub followers: usize,
    /// TODO: read #6 in TODO.md file (structs\projects section)
    #[serde(skip)]
    pub license: Option<Vec<String>>, // placeholder
    /// A list of the version IDs of the project (will never be empty unless draft status)
    pub versions: Vec<String>,
    /// A list of all the game versions supported by the project
    pub game_versions: Vec<String>,
    /// A list of all the loaders supported by the project
    pub loaders: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}
