use std::fs;

use serde_json::json;

use crate::tests::{laid_out, refuses, set, tree, words, Outcome};
use crate::{run, GENERATED};

#[test]
fn the_fields_beside_a_union_join_every_branch() -> Outcome {
    let mut files = tree();
    set(
        "defs/Panel.json",
        "/properties",
        json!({ "title": { "type": "string" } }),
    )(&mut files);
    set("defs/Panel.json", "/required", json!(["title"]))(&mut files);
    let root = laid_out(&files)?;
    run(&words(&["generate"]), root.path())?;
    let mut panel = String::new();
    for file in fs::read_dir(root.path().join(GENERATED))? {
        let path = file?.path();
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(path)?;
        if let Some(at) = text.find("pub enum Panel {") {
            panel = text[at..]
                .split("\n}\n")
                .next()
                .unwrap_or_default()
                .to_owned();
        }
    }
    assert_eq!(panel.matches("title: String,").count(), 2, "{panel}");
    Ok(())
}

#[test]
fn fields_beside_a_branch_they_cannot_join_are_refused() -> Outcome {
    refuses(
        |files| {
            set(
                "defs/Panel.json",
                "/properties",
                json!({ "title": { "type": "string" } }),
            )(files);
            set(
                "defs/Panel.json",
                "/oneOf/1",
                json!({ "$ref": "Code.json" }),
            )(files);
        },
        "Panel: fields beside a branch they cannot join",
    )
}
