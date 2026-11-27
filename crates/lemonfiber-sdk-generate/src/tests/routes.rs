use std::collections::BTreeMap;
use std::fs;

use serde_json::{json, Value};

use super::{edited, laid_out, refuses, set, tree, words, Outcome};
use crate::{run, GENERATED};

#[test]
fn a_body_taken_at_a_route_with_a_name_in_it_is_written_for_the_routes_words() -> Outcome {
    let root = edited(set(
        "index.json",
        "/bodies/~1api~1keys~1{name}",
        json!("bodies/setup-answer.json"),
    ))?;
    run(&words(&["generate"]), root.path())?;
    let mut every = String::new();
    for entry in fs::read_dir(root.path().join(GENERATED))? {
        every.push_str(&fs::read_to_string(entry?.path())?);
    }
    assert!(every.contains("pub enum KeysNameBody"), "{every}");
    Ok(())
}

#[test]
fn every_route_is_written_with_its_method_path_body_and_kinds() -> Outcome {
    let root = laid_out(&tree())?;
    run(&words(&["generate"]), root.path())?;
    let generated = root.path().join(GENERATED);
    let routes = fs::read_to_string(generated.join("routes.rs"))?;
    assert!(routes.contains(
        "crate::Route {\n        method: crate::Method::Delete,\n        path: \"/api/keys/{name}\",\n        body: true,\n        kinds: &[\"status\"],\n    },"
    ), "{routes}");
    assert!(routes.contains("method: crate::Method::Get,") && routes.contains("kinds: &[],"));
    assert!(
        routes.contains("method: crate::Method::Post,")
            && routes.contains("kinds: &[\"status\", \"pull\"],")
    );
    Ok(())
}

#[test]
fn a_route_this_cannot_write_is_refused() -> Outcome {
    refuses(
        |files: &mut BTreeMap<String, Value>| {
            files.insert("routes.json".to_owned(), json!({}));
        },
        "the routes are not a list",
    )?;
    refuses(
        set("routes.json", "/0", json!(1)),
        "route 0 is not an object",
    )?;
    refuses(
        set("routes.json", "/0/method", json!("PATCH")),
        "route 0: \"PATCH\" is not one of GET, POST, DELETE",
    )?;
    refuses(
        set("routes.json", "/0/path", json!("/keys")),
        "route 0: \"/keys\" is not a route's path",
    )?;
    refuses(
        set("routes.json", "/0/body", json!("yes")),
        "route 0: body is not true or false",
    )?;
    refuses(
        set("routes.json", "/0/kinds", json!(["nothing"])),
        "route 0: /api/keys/{name} answers with nothing",
    )?;
    refuses(
        set(
            "routes.json",
            "/1",
            json!({ "method": "DELETE", "path": "/api/keys/{name}", "body": true, "kinds": [] }),
        ),
        "route 1: DELETE /api/keys/{name} is listed twice",
    )?;
    refuses(set("routes.json", "/0/extra", json!(1)), "carries extra")?;
    Ok(())
}
