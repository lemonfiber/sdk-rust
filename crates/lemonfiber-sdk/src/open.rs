use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

const KNOWN_TAG: &str = "a value naming a tag this client knows did not read as that tag";

pub(crate) trait Tags {
    const TAG: Option<&str>;
    const KNOWN: &[&str];
}

pub(crate) fn unrecognised<'de, E: Tags, D: Deserializer<'de>>(
    found: D,
) -> Result<Value, D::Error> {
    let value = Value::deserialize(found)?;
    kept(value, E::TAG, E::KNOWN).map_err(D::Error::custom)
}

pub(crate) fn kept(value: Value, tag: Option<&str>, known: &[&str]) -> Result<Value, &'static str> {
    let named = match (tag, &value) {
        (Some(tag), found) => found.get(tag).and_then(Value::as_str),
        (None, Value::String(named)) => Some(named.as_str()),
        (None, Value::Object(fields)) if fields.len() == 1 => {
            fields.keys().next().map(String::as_str)
        }
        _ => None,
    };
    if named.is_some_and(|named| !known.contains(&named)) {
        Ok(value)
    } else {
        Err(KNOWN_TAG)
    }
}
