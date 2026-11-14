use std::fs;

use serde_json::json;

use crate::tests::{laid_out, refuses, remove, set, tree, words};
use crate::{run, GENERATED};

type Outcome = Result<(), Box<dyn std::error::Error>>;

#[test]
fn the_event_kinds_and_the_heartbeat_are_written_as_the_index_lists_them() -> Outcome {
    let root = laid_out(&tree())?;
    run(&words(&["generate"]), root.path())?;
    let generated = root.path().join(GENERATED);
    let events = fs::read_to_string(generated.join("events.rs"))?;
    assert!(events.contains("pub const HEARTBEAT_SECONDS: u64 = 15;"));
    assert!(events.contains(
        "crate::EventKind {\n        kind: \"pull\",\n        replaces: false,\n        operator: true,\n    },"
    ));
    assert!(events.contains(
        "crate::EventKind {\n        kind: \"status\",\n        replaces: true,\n        operator: false,\n    },"
    ));
    assert!(fs::read_to_string(generated.join("mod.rs"))?.contains("pub mod events;"));
    Ok(())
}

#[test]
fn an_event_stream_this_cannot_write_is_refused_naming_what() -> Outcome {
    refuses(
        remove("index.json", "", "events"),
        "the index lists no event stream",
    )?;
    refuses(
        set("index.json", "/events", json!("events.json")),
        "the event stream is not an object",
    )?;
    refuses(
        set("index.json", "/events/heartbeat_seconds", json!(0)),
        "the event stream names no heartbeat in whole seconds",
    )?;
    refuses(
        set("index.json", "/events/beat", json!(1)),
        "the event stream carries beat",
    )?;
    refuses(
        set("index.json", "/events/kinds", json!({})),
        "the event stream lists no kinds",
    )?;
    refuses(
        set("index.json", "/events/kinds/pull", json!("kinds/pull.json")),
        "the event kind pull is not an object",
    )?;
    refuses(
        set("index.json", "/events/kinds/pull/order", json!(1)),
        "the event kind pull carries order",
    )?;
    refuses(
        set(
            "index.json",
            "/events/kinds/pull/schema",
            json!("kinds/status.json"),
        ),
        "the event kind pull is described by no kind the index lists",
    )?;
    refuses(
        set(
            "index.json",
            "/events/kinds/gone",
            json!({ "schema": "kinds/gone.json", "nature": "state", "audience": "anyone" }),
        ),
        "the event kind gone is described by no kind the index lists",
    )?;
    refuses(
        set("index.json", "/events/kinds/pull/nature", json!("rumour")),
        "the event kind pull is of no nature: record or state",
    )?;
    refuses(
        set(
            "index.json",
            "/events/kinds/pull/audience",
            json!("members"),
        ),
        "the event kind pull has no audience: anyone or operator",
    )
}
