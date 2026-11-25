use serde_json::Value;

use crate::artefact::Artefact;

const READ: &[&str] = &[
    "$comment",
    "$defs",
    "$ref",
    "$schema",
    "additionalProperties",
    "anyOf",
    "const",
    "default",
    "description",
    "enum",
    "examples",
    "exclusiveMaximum",
    "exclusiveMinimum",
    "format",
    "items",
    "maxItems",
    "maxLength",
    "maximum",
    "minItems",
    "minLength",
    "minimum",
    "oneOf",
    "pattern",
    "prefixItems",
    "properties",
    "required",
    "title",
    "type",
    "uniqueItems",
];

const DESCENDED: &[&str] = &[
    "additionalProperties",
    "allOf",
    "anyOf",
    "items",
    "not",
    "oneOf",
    "prefixItems",
];

const NAMED: &[&str] = &["$defs", "properties"];

pub fn unread(schema: &Value, at: &str, problems: &mut Vec<String>) {
    let Some(fields) = schema.as_object() else {
        return;
    };
    let beyond: Vec<&str> = fields
        .keys()
        .map(String::as_str)
        .filter(|key| !READ.contains(key))
        .collect();
    if !beyond.is_empty() {
        problems.push(format!(
            "{at} uses {}, which this generator does not read",
            beyond.join(", ")
        ));
    }
    for (key, value) in fields {
        if NAMED.contains(&key.as_str()) {
            for (name, inner) in value.as_object().into_iter().flatten() {
                unread(inner, &format!("{at}/{key}/{name}"), problems);
            }
        } else if DESCENDED.contains(&key.as_str()) {
            match value {
                Value::Array(inners) => {
                    for (place, inner) in inners.iter().enumerate() {
                        unread(inner, &format!("{at}/{key}/{place}"), problems);
                    }
                }
                inner => unread(inner, &format!("{at}/{key}"), problems),
            }
        }
    }
}

pub fn unread_in(artefact: &Artefact, problems: &mut Vec<String>) {
    for (name, schema) in &artefact.defs {
        unread(schema, &format!("the definition {name}"), problems);
    }
    for (kind, schema) in &artefact.kinds {
        unread(schema, &format!("the kind {kind}"), problems);
    }
    for body in &artefact.bodies {
        unread(
            &body.schema,
            &format!("the body of {}", body.route),
            problems,
        );
    }
}

#[cfg(test)]
mod tests;
