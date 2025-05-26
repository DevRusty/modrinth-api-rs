use super::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct ExtraOptions<'a> {
    // pub number: Option<&'a str>,
    pub loaders: Option<&'a [&'a str]>,
    pub game_versions: Option<&'a [&'a str]>,
    pub featured: Option<bool>,
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
