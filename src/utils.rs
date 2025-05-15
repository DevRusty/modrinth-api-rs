use crate::{Error, Result};
use lazy_regex::regex_is_match;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

pub fn check_id_slug<S: AsRef<str>>(inputs: &[S]) -> Result<()> {
    inputs.iter().try_for_each(|input| {
        if !regex_is_match!(r#"^[\w!@$()`.+,"\-']{3,64}$"#, input.as_ref()) {
            return Err(Error::InvalidIDorSlug);
        }
        Ok(())
    })
}

// From: https://github.com/gorilla-devs/ferinth/blob/master/src/request.rs
pub(crate) trait RequestBuilderCustomSend {
    /// Build and send `self`, and return the response
    async fn custom_send(self) -> Result<Response>;

    /// Build and send `self`, and deserialise the response to `T` and return it
    async fn custom_send_json<T: DeserializeOwned>(self) -> Result<T>;
}

impl RequestBuilderCustomSend for RequestBuilder {
    async fn custom_send(self) -> Result<Response> {
        Ok(check_rate_limit(self.send().await?)?.error_for_status()?)
    }

    async fn custom_send_json<T: DeserializeOwned>(self) -> Result<T> {
        let bytes = self.custom_send().await?.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

// From: https://github.com/gorilla-devs/ferinth/blob/master/src/request.rs
// Modified by nixxoq (if-else -> match)
fn check_rate_limit(response: Response) -> Result<Response> {
    match response.status() {
        StatusCode::GONE => Err(crate::Error::ApiDeprecated),
        StatusCode::TOO_MANY_REQUESTS => Err(crate::Error::RateLimitExceeded(
            response
                .headers()
                .get("X-Ratelimit-Reset")
                .map(|header| {
                    header
                        .to_str()
                        .expect("Corrupted ratelimit header")
                        .parse()
                        .expect("Corrupted ratelimit header")
                })
                .expect("Corrupted ratelimit header"),
        )),
        _ => Ok(response),
    }
}

// Extensions to `url::Url` to make it generally easier to use
// From: https://github.com/gorilla-devs/ferinth/blob/master/src/url_ext.rs
pub trait UrlJoinAll {
    /// [Url::join] all the provided `segments`
    fn join_all(&self, segments: Vec<impl Into<String>>) -> Self;
}

impl UrlJoinAll for Url {
    fn join_all(&self, mut segments: Vec<impl Into<String>>) -> Self {
        let mut url = self.clone();
        let last = segments.pop().expect("`segments` is empty");
        for segment in segments {
            let mut segment = segment.into();
            segment.push('/');
            url = url.join(&segment).expect("Invalid URL segment");
        }
        url.join(&last.into()).expect("Invalid URL segment")
    }
}

pub trait UrlWithQuery: Sized {
    type SerialiseResult<T>;

    /// Add the `name` and `value` query to `self` and return it
    fn with_query(self, name: impl AsRef<str>, value: impl ToString) -> Self;

    /// Serialise and add the `name` and `value` query to `self` and return it
    fn with_query_json(
        self,
        name: impl AsRef<str>,
        value: impl Serialize,
    ) -> Self::SerialiseResult<Self>;
}

impl UrlWithQuery for Url {
    type SerialiseResult<T> = serde_json::Result<T>;

    fn with_query(mut self, name: impl AsRef<str>, value: impl ToString) -> Self {
        self.query_pairs_mut()
            .append_pair(name.as_ref(), &value.to_string());
        self
    }

    fn with_query_json(
        mut self,
        name: impl AsRef<str>,
        value: impl Serialize,
    ) -> Self::SerialiseResult<Self> {
        self.query_pairs_mut()
            .append_pair(name.as_ref(), &serde_json::to_string(&value)?);
        Ok(self)
    }
}
