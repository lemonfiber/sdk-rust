use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::{at, edited, refused_with, set, words, Outcome};
use crate::run;

#[test]
fn examples_are_read_and_one_naming_what_the_index_does_not_list_is_refused() -> Outcome {
    let examples = |files: &mut BTreeMap<String, Value>| {
        files.insert(
            "examples/kinds/status.json".to_owned(),
            json!({ "api_version": 1, "kind": "status", "data": {} }),
        );
        files.insert(
            "examples/actions/restart.json".to_owned(),
            json!({ "arguments": {}, "answer": {} }),
        );
        at(files, "index.json", "").as_object_mut().map(|index| {
            index.insert(
                "examples".to_owned(),
                json!({
                    "kinds": { "status": "examples/kinds/status.json" },
                    "actions": { "restart": "examples/actions/restart.json" }
                }),
            )
        });
    };
    let root = edited(examples)?;
    assert!(run(&words(&["generate"]), root.path()).is_ok());

    for (edit, said) in [
        (
            ("/examples/kinds/gone", json!("examples/kinds/status.json")),
            "an example of gone, which the index lists no kind of",
        ),
        (
            ("/examples/kinds/pull", json!("examples/kinds/status.json")),
            "is not an envelope of the kind pull",
        ),
        (
            (
                "/examples/actions/gone",
                json!("examples/actions/restart.json"),
            ),
            "an example of gone, which the index lists no action of",
        ),
        (
            ("/examples/kinds/status", json!(1)),
            "the example of the kind status names no file",
        ),
        (
            ("/examples/actions/restart", json!(1)),
            "the example of the action restart names no file",
        ),
        (("/examples/reads", json!({})), "the examples carries reads"),
    ] {
        let root = edited(|files| {
            examples(files);
            set("index.json", edit.0, edit.1)(files);
        })?;
        let refused = refused_with(root.path(), &["generate"]);
        assert!(refused.contains(said), "{said}: {refused}");
    }
    let root = edited(|files| {
        examples(files);
        set("index.json", "/examples", json!([]))(files);
    })?;
    assert!(refused_with(root.path(), &["generate"]).contains("the examples is not an object"));
    Ok(())
}
