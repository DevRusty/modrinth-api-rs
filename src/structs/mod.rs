pub mod projects;
pub mod search;

use serde::{Deserialize, Serialize};
use url::Url;

pub type Date = chrono::DateTime<chrono::Utc>;

fn deserialise_optional_url<'de, D: serde::Deserializer<'de>>(
    de: D,
) -> Result<Option<Url>, D::Error> {
    use serde::de::{Error, Unexpected};
    use std::borrow::Cow;

    let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;
    match intermediate.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => Url::parse(s).map_or_else(
            |err| {
                Err(Error::invalid_value(
                    Unexpected::Str(s),
                    &err.to_string().as_str(),
                ))
            },
            |ok| Ok(Some(ok)),
        ),
    }
}
