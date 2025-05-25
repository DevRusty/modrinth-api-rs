//! Models related to projects
//!
//! [documentation](https://docs.modrinth.com/api/operations/tags/projects/)

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub client_side: ProjectSupportRange,
    /// The server side support of the project
    pub server_side: ProjectSupportRange,
    /// A long form description of the project
    pub body: String,
    /// The status of the project
    pub status: ProjectStatus,
    /// The requested status when submitting for review or scheduling the project for release
    pub requested_status: Option<RequestedStatus>,
    /// A list of categories which are searchable but non-primary
    pub additional_categories: Vec<String>,
    /// An optional link to where to submit bugs or issues with the project
    pub issues_url: Option<String>,
    /// An optional link to the source code of the project
    pub source_url: Option<String>,
    /// An optional link to the project’s wiki page or other relevant information
    pub wiki_url: Option<String>,
    /// An optional invite link to the project’s discord
    pub discord_url: Option<String>,
    /// Donation links / urls
    pub donation_urls: Vec<DonationLink>,
    pub project_type: String,
    pub downloads: usize,
    /// Project icon URL
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
    pub license: License,
    /// A list of the version IDs of the project (will never be empty unless draft status)
    pub versions: Vec<String>,
    /// A list of all the game versions supported by the project
    pub game_versions: Vec<String>,
    /// A list of all the loaders supported by the project
    pub loaders: Vec<String>,
}

impl Project {
    /// Returns a read-only reference to the project's donation URLs.
    ///
    /// This method serves as an alias/getter for the [`Project::donation_urls`] field,
    /// providing direct access to the same underlying data.
    pub fn donation_links(&self) -> &Vec<DonationLink> {
        self.donation_urls.as_ref()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    /// Can be a mod, plugin, or data pack
    Project,
    Mod,
    Shader,
    Plugin,
    Modpack,
    Datapack,
    ResourcePack,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MonetizationStatus {
    Monetized,
    Demonetized,
    ForceDemonetized,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSupportRange {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Withheld,
    Scheduled,
    Private,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Approved,
    Archived,
    Unlisted,
    Private,
    Draft,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DonationLink {
    pub id: String,
    pub platform: String,
    pub url: String,
}
