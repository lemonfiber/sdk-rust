use serde_json::json;

use super::unread;

#[test]
fn a_keyword_this_does_not_read_is_named_wherever_it_is_nested() {
    let schema = json!({
        "type": "object",
        "properties": { "spans": { "items": { "if": {} } } },
        "allOf": [{ "oneOf": [{ "not": { "then": {} } }] }],
        "$defs": { "Inner": { "additionalProperties": { "else": {} } } }
    });
    let mut problems = Vec::new();
    unread(&schema, "RunningCopy", &mut problems);

    assert_eq!(
        problems,
        [
            "RunningCopy uses allOf, which this generator does not read",
            "RunningCopy/$defs/Inner/additionalProperties uses else, which this generator does not read",
            "RunningCopy/allOf/0/oneOf/0 uses not, which this generator does not read",
            "RunningCopy/allOf/0/oneOf/0/not uses then, which this generator does not read",
            "RunningCopy/properties/spans/items uses if, which this generator does not read",
        ]
    );
}

#[test]
fn what_this_reads_is_not_named() {
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "description": "A backup or restore running now.",
        "type": "object",
        "properties": { "scope": { "$ref": "CopiedScope.json", "description": "What it copies." } },
        "oneOf": [{ "type": "object", "properties": { "state": { "const": "working" } }, "required": ["state"] }],
        "additionalProperties": false
    });
    let mut problems = Vec::new();
    unread(&schema, "RunningCopy", &mut problems);

    assert!(problems.is_empty(), "{problems:?}");
}
