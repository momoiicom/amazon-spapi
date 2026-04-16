//! Custom boolean type that deserializes from both native `bool` and string `"true"`/`"false"`.
//! Amazon SP-API notoriously returns boolean values as strings in many endpoints.

use serde::{Deserialize, Serialize, Deserializer};

/// A boolean that can be deserialized from either a native JSON bool or a string ("true"/"false").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringBool(#[serde(deserialize_with = "deserialize_string_bool")] pub bool);

impl From<bool> for StringBool {
    fn from(b: bool) -> Self {
        StringBool(b)
    }
}

impl From<StringBool> for bool {
    fn from(sb: StringBool) -> Self {
        sb.0
    }
}

impl Default for StringBool {
    fn default() -> Self {
        StringBool(false)
    }
}

/// Deserialize a value that may be a native bool or a string ("true"/"false").
pub fn deserialize_string_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrString {
        Bool(bool),
        String(String),
    }

    match BoolOrString::deserialize(deserializer)? {
        BoolOrString::Bool(b) => Ok(b),
        BoolOrString::String(s) => match s.to_lowercase().as_str() {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            other => Err(serde::de::Error::custom(format!(
                "expected boolean or boolean string, got '{}'",
                other
            ))),
        },
    }
}
