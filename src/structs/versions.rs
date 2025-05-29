use super::*;
use serde::{Deserialize, Serialize};

/// The Version struct
///
/// Documentation: https://docs.modrinth.com/api/operations/getprojectversions/#200
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Vec<Dependencies>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: Option<Status>,
    pub requested_status: Option<RequestedStatus>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub date_published: Date,
    pub downloads: usize,
    pub files: Vec<File>,
}

/// Extra parameters for [ModrinthAPI::get_project_versions] function
///
/// Fields of `ProjectVersionsFilter`:
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
#[derive(Debug)]
pub struct ProjectVersionsFilter<'a> {
    pub loaders: Option<&'a [&'a str]>,
    pub game_versions: Option<&'a [&'a str]>,
    pub featured: Option<bool>,
}

impl<'a> Default for ProjectVersionsFilter<'a> {
    fn default() -> Self {
        ProjectVersionsFilter {
            loaders: None,
            game_versions: None,
            featured: None,
        }
    }
}

/// Extra parameters for [ModrinthAPI::get_project_version] function
///
/// Fields
///
///   - `number`: `Option<&str>` - Specific version ID or a version number string (e.g., "1.0.0").
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
#[derive(Debug)]
pub struct ProjectVersionParams<'a> {
    /// Get a version given a version number or ID
    ///
    /// Note:
    ///   * if the version number provided matches multiple versions, only the oldest matching version will be returned.
    pub number: Option<&'a str>,
    pub loaders: Option<&'a [&'a str]>,
    pub game_versions: Option<&'a [&'a str]>,
    pub featured: Option<bool>,
}

impl<'a> Default for ProjectVersionParams<'a> {
    fn default() -> Self {
        ProjectVersionParams {
            number: Some(""),
            loaders: None,
            game_versions: None,
            featured: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Dependencies {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    pub hashes: Hash,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: usize,
    pub file_type: Option<FileType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hash {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FileType {
    RequiredResourcePack,
    OptionalResourcePack,
}
