use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::artefact::{object, unread};

const WORDS_FIELDS: &[&str] = &["names", "terms", "sizes", "marks", "declared", "examples"];

const EXAMPLE_FIELDS: &[&str] = &["text", "names"];

const WHAT: &str = "the household words";

const FILE: &str = "the file of household words";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Words {
    pub names: Vec<String>,
    pub terms: Vec<String>,
    pub sizes: Vec<String>,
    pub marks: Vec<String>,
    pub declared: BTreeMap<String, String>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Example {
    pub text: String,
    pub names: String,
}

fn said(node: Option<&Value>) -> Option<&str> {
    node.and_then(Value::as_str).filter(|word| !word.is_empty())
}

fn listed(fields: &Map<String, Value>, list: &str, problems: &mut Vec<String>) -> Vec<String> {
    let Some(entries) = fields.get(list).and_then(Value::as_array) else {
        problems.push(format!("{WHAT} hold no list of {list}"));
        return Vec::new();
    };
    let mut words = Vec::new();
    for (at, entry) in entries.iter().enumerate() {
        match said(Some(entry)) {
            Some(word) if !words.iter().any(|seen| seen == word) => words.push(word.to_owned()),
            Some(word) => problems.push(format!("{WHAT} list {word:?} twice in {list}")),
            None => problems.push(format!("entry {at} of the {list} in {WHAT} is not a word")),
        }
    }
    words
}

fn declared(fields: &Map<String, Value>, problems: &mut Vec<String>) -> BTreeMap<String, String> {
    let mut declared = BTreeMap::new();
    let Some(entries) = fields.get("declared").and_then(Value::as_object) else {
        problems.push(format!("{WHAT} hold no names lemonfiber declares"));
        return declared;
    };
    for (name, names) in entries {
        match said(Some(names)) {
            Some(names) if !name.is_empty() => {
                declared.insert(name.clone(), names.to_owned());
            }
            _ => problems.push(format!(
                "the declared name {name:?} in {WHAT} is not said with what it names"
            )),
        }
    }
    declared
}

fn examples(fields: &Map<String, Value>, problems: &mut Vec<String>) -> Vec<Example> {
    let Some(entries) = fields.get("examples").and_then(Value::as_array) else {
        problems.push(format!("{WHAT} hold no list of examples"));
        return Vec::new();
    };
    let mut examples = Vec::new();
    for (at, entry) in entries.iter().enumerate() {
        let what = format!("example {at} of {WHAT}");
        let Some(entry) = object(entry, &what, problems) else {
            continue;
        };
        unread(entry, EXAMPLE_FIELDS, &what, problems);
        match (said(entry.get("text")), said(entry.get("names"))) {
            (Some(text), Some(names)) => examples.push(Example {
                text: text.to_owned(),
                names: names.to_owned(),
            }),
            _ => problems.push(format!("{what} is not a text and what in it is named")),
        }
    }
    examples
}

pub fn read(published: &Value, problems: &mut Vec<String>) -> Words {
    let Some(fields) = object(published, FILE, problems) else {
        return Words::default();
    };
    unread(fields, WORDS_FIELDS, FILE, problems);
    Words {
        names: listed(fields, "names", problems),
        terms: listed(fields, "terms", problems),
        sizes: listed(fields, "sizes", problems),
        marks: listed(fields, "marks", problems),
        declared: declared(fields, problems),
        examples: examples(fields, problems),
    }
}

#[cfg(test)]
mod tests;
