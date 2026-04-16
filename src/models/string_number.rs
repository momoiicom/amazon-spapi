//! Custom string type that deserializes from both native string and number.
//! Amazon SP-API returns some string identifiers as numbers (e.g. order_item_id).

use serde::{Deserialize, Deserializer, Serialize};

/// A string that can be deserialized from either a native JSON string or number.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringNumber(#[serde(deserialize_with = "deserialize_string_number")] pub String);

impl From<String> for StringNumber {
    fn from(s: String) -> Self {
        StringNumber(s)
    }
}

impl From<&str> for StringNumber {
    fn from(s: &str) -> Self {
        StringNumber(s.to_string())
    }
}

impl From<StringNumber> for String {
    fn from(sn: StringNumber) -> Self {
        sn.0
    }
}

/// Deserialize a value that may be a native string or a number.
pub fn deserialize_string_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(serde_json::Number),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(s),
        StringOrNumber::Number(n) => Ok(n.to_string()),
    }
}
