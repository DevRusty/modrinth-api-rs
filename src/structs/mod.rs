pub mod projects;
pub mod search;

use serde::{Deserialize, Serialize};
use url::Url;

pub type Date = chrono::DateTime<chrono::Utc>;
