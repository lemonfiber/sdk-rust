use serde::Serialize;
use serde_json::Value;

use crate::Error;

const KEPT: &[u8] = b"-._~";
const UNKEPT: &[&str] = &["", ".", ".."];

pub(crate) fn segment(name: &str) -> Result<String, Error> {
    if UNKEPT.contains(&name) {
        return Err(Error::Unsendable(format!(
            "{name:?} is not a name that can be sent as one part of a path."
        )));
    }
    Ok(name
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || KEPT.contains(&byte) {
                char::from(byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect())
}

pub(crate) fn located<R: Serialize>(
    path: &str,
    segments: &[&str],
    asked: &R,
) -> Result<String, Error> {
    placed(path, segments, serde_json::to_value(asked).ok())
}

pub(crate) fn placed(path: &str, segments: &[&str], asked: Option<Value>) -> Result<String, Error> {
    let Some(Value::Object(mut fields)) = asked else {
        return Err(Error::Unsendable(
            "That read holds what cannot be sent as a query.".to_owned(),
        ));
    };
    let mut located = path.to_owned();
    for name in segments {
        let value = fields
            .remove(*name)
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_default();
        let written = segment(&value).map_err(|_| {
            Error::Unsendable(format!(
                "This read needs a `{name}` it can send, and was not given one."
            ))
        })?;
        located = located.replace(&format!("{{{name}}}"), &written);
    }
    let mut query = url::form_urlencoded::Serializer::new(String::new());
    let mut any = false;
    for (name, value) in &fields {
        let values = match value {
            Value::Array(values) => values.clone(),
            other => vec![other.clone()],
        };
        for value in values {
            let written = match value {
                Value::String(text) => text,
                Value::Null => continue,
                other => other.to_string(),
            };
            query.append_pair(name, &written);
            any = true;
        }
    }
    if any {
        located.push('?');
        located.push_str(&query.finish());
    }
    Ok(located)
}
