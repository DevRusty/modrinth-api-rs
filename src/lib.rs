use std::sync::LazyLock;

use reqwest::{Client, header::InvalidHeaderValue};
use url::Url;

pub mod api;
pub mod structs;
pub mod utils;

pub static BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://api.modrinth.com/")
        .expect("Invalid base URL")
        .join(concat!("v", "2", "/")).expect("Invalid API base URL")
});

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    #[error("Invalid Modrinth ID or slug")]
    InvalidIDorSlug,
    #[error("Invalid SHA1 hash")]
    InvalidSHA1,
    #[error("You have been rate limited, please wait for {0} seconds")]
    RateLimitExceeded(usize),
    #[error("The API at {} is deprecated", *BASE_URL)]
    ApiDeprecated,
    ReqwestError(#[from] reqwest::Error),
    JSONError(#[from] serde_json::Error),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ModrinthAPI {
    client: Client,
}

impl Default for ModrinthAPI {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .user_agent(concat!(
                    env!("CARGO_CRATE_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .expect("Failed to initialize TLS backend"),
        }
    }
}

impl ModrinthAPI {
    fn client_builder(
        name: &str,
        version: Option<&str>,
        contact: Option<&str>,
    ) -> reqwest::ClientBuilder {
        Client::builder().user_agent(format!(
            "{}{}{}",
            name,
            version.map_or("".into(), |version| format!("/{}", version)),
            contact.map_or("".into(), |contact| format!(" ({})", contact))
        ))
    }

    pub fn new(name: &str, version: Option<&str>, contact: Option<&str>) -> Self {
        Self {
            client: Self::client_builder(name, version, contact)
                .build()
                .expect("Failed to initialise TLS backend"),
        }
    }
}
