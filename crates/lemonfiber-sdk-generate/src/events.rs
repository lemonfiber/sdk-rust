use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::artefact::{object, unread};

const EVENTS_FIELDS: &[&str] = &["heartbeat_seconds", "kinds"];

const KIND_FIELDS: &[&str] = &["schema", "nature", "audience"];

const NATURES: [(&str, bool); 2] = [("record", false), ("state", true)];

const AUDIENCES: [(&str, bool); 2] = [("anyone", false), ("operator", true)];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Events {
    pub heartbeat_seconds: u64,
    pub kinds: Vec<EventKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventKind {
    pub kind: String,
    pub replaces: bool,
    pub operator: bool,
}

fn one_of(said: Option<&Value>, known: &[(&str, bool)]) -> Option<bool> {
    let said = said.and_then(Value::as_str)?;
    known
        .iter()
        .find(|(name, _)| *name == said)
        .map(|(_, flag)| *flag)
}

fn names(known: &[(&str, bool)]) -> String {
    known
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(" or ")
}

pub fn read(
    index: &Map<String, Value>,
    kinds: &BTreeMap<String, Value>,
    problems: &mut Vec<String>,
) -> Events {
    let mut events = Events::default();
    let Some(listed) = index.get("events") else {
        problems.push("the index lists no event stream".to_owned());
        return events;
    };
    let Some(listed) = object(listed, "the event stream", problems) else {
        return events;
    };
    unread(listed, EVENTS_FIELDS, "the event stream", problems);
    match listed
        .get("heartbeat_seconds")
        .and_then(Value::as_u64)
        .filter(|seconds| *seconds > 0)
    {
        Some(seconds) => events.heartbeat_seconds = seconds,
        None => problems.push("the event stream names no heartbeat in whole seconds".to_owned()),
    }
    let files = index.get("kinds").and_then(Value::as_object);
    for (kind, entry) in listed
        .get("kinds")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        let what = format!("the event kind {kind}");
        let Some(entry) = object(entry, &what, problems) else {
            continue;
        };
        unread(entry, KIND_FIELDS, &what, problems);
        let described = files.and_then(|files| files.get(kind));
        if !kinds.contains_key(kind) || entry.get("schema") != described {
            problems.push(format!(
                "{what} is described by no kind the index lists, under the file it names"
            ));
        }
        let replaces = one_of(entry.get("nature"), &NATURES);
        let operator = one_of(entry.get("audience"), &AUDIENCES);
        if replaces.is_none() {
            problems.push(format!("{what} is of no nature: {}", names(&NATURES)));
        }
        if operator.is_none() {
            problems.push(format!("{what} has no audience: {}", names(&AUDIENCES)));
        }
        if let (Some(replaces), Some(operator)) = (replaces, operator) {
            events.kinds.push(EventKind {
                kind: kind.clone(),
                replaces,
                operator,
            });
        }
    }
    if events.kinds.is_empty() {
        problems.push("the event stream lists no kinds".to_owned());
    }
    events
}

#[cfg(test)]
mod tests;
