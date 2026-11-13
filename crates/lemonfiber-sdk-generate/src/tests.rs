use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::artefact::Artefact;
use crate::emit::{formatted, paged};
use crate::schema::shaped;
use crate::{run, Refused, CONTRACT, GENERATED, STAMP};

type Outcome = Result<(), Box<dyn std::error::Error>>;

const REVISION: &str = "0123456789abcdef0123456789abcdef01234567";

fn words(command: &[&str]) -> Vec<String> {
    command.iter().map(|word| (*word).to_owned()).collect()
}

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn envelope(data: &Value) -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Envelope",
        "type": "object",
        "properties": {
            "api_version": { "type": "integer", "format": "uint32" },
            "data": data,
            "kind": { "type": "string" },
        },
        "required": ["api_version", "kind", "data"],
    })
}

fn shapes(put: &mut impl FnMut(&str, Value)) {
    put(
        "defs/Level.json",
        json!({ "oneOf": [
            { "description": "All is well.", "type": "string", "const": "fine" },
            { "type": "string", "enum": ["not-fine"] },
        ] }),
    );
    put(
        "defs/Panel.json",
        json!({ "oneOf": [
            { "type": "object", "properties": { "panel": { "const": "ready" }, "data": { "type": "object", "properties": { "reason": { "type": "string" } }, "required": ["reason"] } }, "required": ["panel", "data"] },
            { "type": "object", "properties": { "panel": { "const": "empty" } }, "required": ["panel"] },
        ] }),
    );
    put(
        "defs/Value.json",
        json!({ "anyOf": [
            { "type": "string" },
            { "type": "integer" },
            { "type": "number" },
            { "type": "boolean" },
            { "type": "array", "items": { "type": "string" } },
            { "$ref": "Level.json" },
            { "type": "object", "properties": { "y": { "type": "integer" } } },
        ] }),
    );
    put(
        "defs/Choice.json",
        json!({ "oneOf": [
            { "type": "string", "const": "none" },
            { "description": "A choice of its own.", "type": "object", "properties": { "custom": { "type": "string" } }, "required": ["custom"], "additionalProperties": false },
            { "type": "object", "properties": { "pinned": { "description": "Pinned.", "type": "boolean" } }, "required": ["pinned"], "additionalProperties": false },
        ] }),
    );
    put("defs/Code.json", json!({ "type": "string" }));
    put(
        "defs/Side.json",
        json!({ "oneOf": [{ "type": "string", "const": "left" }, { "type": "string", "const": "right" }] }),
    );
}

fn lists(put: &mut impl FnMut(&str, Value)) {
    put(
        "actions/restart.json",
        json!({ "action": "restart", "arguments": [
            { "name": "forms", "type": { "default": [], "type": "array", "items": { "type": "string" } } },
            { "name": "offer", "type": { "default": null, "type": ["string", "null"] } },
            { "name": "level", "type": { "default": "fine", "$ref": "../defs/Level.json" } },
            { "name": "sides", "type": { "default": null, "type": "array", "prefixItems": [{ "$ref": "../defs/Side.json" }], "minItems": 1, "maxItems": 1 } },
        ], "consent": ["offer"], "rehearsal": true }),
    );
    put(
        "actions/diagnose.json",
        json!({ "action": "diagnose", "arguments": [
            { "name": "disruptive", "type": { "type": "boolean" } },
            { "name": "dry_run", "type": { "default": false, "type": "boolean" } },
        ], "consent": [], "rehearsal": true }),
    );
    put(
        "reads.json",
        json!([
            { "path": "/api/status", "parameters": [{ "name": "form", "repeatable": true }, { "name": "type", "repeatable": false }, { "name": "self", "repeatable": false }], "kinds": ["status"], "file": false },
            { "path": "/api/services", "parameters": [], "kinds": ["status"], "file": false },
            { "path": "/api/held/{id}", "parameters": [{ "name": "member", "repeatable": false }], "kinds": ["status", "pull"], "file": false },
            { "path": "/api/pulled", "parameters": [], "kinds": ["pull"], "file": false },
            { "path": "/api/bundle/{name}", "parameters": [], "kinds": [], "file": true },
        ]),
    );
    put(
        "key-callable.json",
        json!([
            { "action": "restart", "disturbs": true, "rehearsal": true, "idempotent": true, "moved": "LIFE-10" },
            { "action": "diagnose", "disturbs": true, "rehearsal": false, "idempotent": false },
        ]),
    );
    put(
        "refusals/LIFE.json",
        json!({
            "LIFE-10": { "name": "OFFER_MOVED", "status": 409, "description": "Raised when the offer moved." },
            "LIFE-9": { "name": "BUSY", "status": 409, "description": "Raised when the stack is busy." },
        }),
    );
}

fn tree() -> BTreeMap<String, Value> {
    let mut files = BTreeMap::new();
    let mut put = |path: &str, value: Value| {
        files.insert(path.to_owned(), value);
    };
    put(
        "index.json",
        json!({
            "api_version": 1,
            "kinds": { "status": "kinds/status.json", "pull": "kinds/pull.json" },
            "actions": { "restart": "actions/restart.json", "diagnose": "actions/diagnose.json" },
            "bodies": { "/api/setup/answer": "bodies/setup-answer.json" },
            "key_callable": "key-callable.json",
            "reads": "reads.json",
            "refusals": { "LIFE": "refusals/LIFE.json" },
        }),
    );
    put(
        "kinds/status.json",
        envelope(
            &json!({ "description": "The payload.", "$ref": "../defs/Status_of_Things.json" }),
        ),
    );
    put("kinds/pull.json", envelope(&json!({ "type": "string" })));
    put(
        "defs/Status_of_Things.json",
        json!({
            "description": "How the stack is.\n\n    indented, and not code",
            "type": "object",
            "properties": {
                "as": { "type": "string" },
                "self": { "type": "boolean" },
                "data-location": { "type": ["string", "null"] },
                "small": { "type": "integer", "format": "uint8" },
                "medium": { "type": "integer", "format": "uint16" },
                "large": { "type": "integer", "format": "uint32" },
                "huge": { "type": "integer", "format": "uint64" },
                "size": { "type": "integer", "format": "uint" },
                "signed": { "type": "integer", "format": "int32" },
                "wide": { "type": "integer", "format": "int64" },
                "plain": { "type": "integer" },
                "ratio": { "type": "number" },
                "nothing": { "type": "null" },
                "anything": { "description": "" },
                "pair": { "type": "array", "prefixItems": [{ "type": "integer" }, { "type": "string" }] },
                "names": { "type": "array", "items": { "type": "string" } },
                "rows": { "type": "array", "items": { "type": "object", "properties": { "x": { "type": "string" } } } },
                "by_name": { "type": "object", "additionalProperties": { "$ref": "Level.json" } },
                "loose": { "type": "object" },
                "inner": { "type": "object", "properties": { "x": { "type": "string" } }, "required": ["x"] },
                "level": { "$ref": "./Level.json", "description": "Its level." },
                "maybe": { "anyOf": [{ "$ref": "Level.json" }, { "type": "null" }] },
                "either": { "anyOf": [{ "$ref": "Level.json" }, { "type": "string" }, { "type": "null" }] },
                "panel": { "$ref": "Panel.json" },
                "value": { "$ref": "Value.json" },
                "choice": { "$ref": "Choice.json" },
                "code": { "$ref": "Code.json" },
                "cause": { "anyOf": [{ "$ref": "Status_of_Things.json" }, { "type": "null" }] },
                "default": { "$ref": "Code.json" },
                "mode": { "oneOf": [{ "type": "string", "const": "a" }, { "type": "string", "const": "b" }] },
            },
            "required": ["as", "level", "panel"],
        }),
    );
    shapes(&mut put);
    put(
        "bodies/setup-answer.json",
        json!({ "title": "Answer", "oneOf": [
            { "type": "object", "properties": { "vpn": { "type": "boolean" } }, "required": ["vpn"], "additionalProperties": false },
        ] }),
    );
    lists(&mut put);
    files
}

fn laid_out(
    files: &BTreeMap<String, Value>,
) -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    for (path, value) in files {
        let at = root.path().join(CONTRACT).join(path);
        fs::create_dir_all(at.parent().ok_or("a file has no folder")?)?;
        fs::write(at, serde_json::to_string_pretty(value)?)?;
    }
    fs::write(root.path().join(STAMP), format!("{REVISION}\n"))?;
    Ok(root)
}

fn edited(
    edit: impl FnOnce(&mut BTreeMap<String, Value>),
) -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let mut files = tree();
    edit(&mut files);
    laid_out(&files)
}

fn at<'a>(files: &'a mut BTreeMap<String, Value>, file: &str, pointer: &str) -> &'a mut Value {
    files
        .get_mut(file)
        .and_then(|value| value.pointer_mut(pointer))
        .unwrap_or_else(|| unreachable!("the fixture holds {file}{pointer}"))
}

fn set(
    file: &'static str,
    pointer: &'static str,
    value: Value,
) -> impl FnOnce(&mut BTreeMap<String, Value>) {
    move |files| {
        let (parent, key) = pointer.rsplit_once('/').unwrap_or_default();
        let key = key.replace("~1", "/");
        match at(files, file, parent) {
            Value::Object(fields) => {
                fields.insert(key, value);
            }
            Value::Array(items) => {
                if let Some(item) = key
                    .parse::<usize>()
                    .ok()
                    .and_then(|place| items.get_mut(place))
                {
                    *item = value;
                }
            }
            _ => unreachable!("{file}{parent} holds fields or items"),
        }
    }
}

fn refused_with(root: &Path, command: &[&str]) -> String {
    run(&words(command), root)
        .err()
        .map(|refused| refused.to_string())
        .unwrap_or_default()
}

#[test]
fn the_vendored_artefact_generates_what_is_committed() -> Outcome {
    let said = run(&words(&["check"]), &repository())?;
    assert!(said.contains("is what the contract generates"));
    Ok(())
}

#[test]
fn a_command_this_does_not_know_is_refused_naming_every_one_it_does() {
    let said = refused_with(Path::new("."), &["publish"]);
    assert!(said.contains("generate") && said.contains("take"));
    assert_eq!(Refused::new("why").said(), "why");
}

#[test]
fn an_enum_an_answer_can_carry_is_open_and_one_a_request_reaches_is_closed() -> Outcome {
    let root = laid_out(&tree())?;
    run(&words(&["generate"]), root.path())?;
    let generated = root.path().join(GENERATED);
    let shapes = fs::read_to_string(generated.join("shapes_s.rs"))?;
    let module = fs::read_to_string(generated.join("mod.rs"))?;
    let open = "#[non_exhaustive]";
    let panel = fs::read_to_string(generated.join("shapes_p.rs"))?;
    assert!(panel.contains(&format!(
        "#[serde(tag = \"panel\")]\n{open}\npub enum Panel"
    )));
    assert!(panel.contains(
        "#[serde(untagged, deserialize_with = \"crate::open::unrecognised::<Panel, _>\")]\n    Unrecognised(serde_json::Value),"
    ));
    assert!(panel.contains("const TAG: Option<&str> = Some(\"panel\");"));
    assert!(panel.contains("const KNOWN: &[&str] = &[\"ready\", \"empty\"];"));
    let choice = fs::read_to_string(generated.join("shapes_c.rs"))?;
    assert!(choice.contains(&format!("{open}\npub enum Choice")));
    assert!(choice.contains(
        "#[serde(untagged, deserialize_with = \"crate::open::unrecognised::<Choice, _>\")]\n    Unrecognised(serde_json::Value),"
    ));
    assert!(choice.contains("const TAG: Option<&str> = None;"));
    let value = fs::read_to_string(generated.join("shapes_v.rs"))?;
    assert!(value.contains(&format!("#[serde(untagged)]\n{open}\npub enum Value")));
    assert!(value.contains("    Unrecognised(serde_json::Value),"));
    let level = fs::read_to_string(generated.join("shapes_l.rs"))?;
    assert!(level.contains("pub enum Level") && !level.contains(open));
    assert!(!level.contains("Unrecognised"));
    let side = fs::read_to_string(generated.join("shapes_s.rs"))?;
    assert!(side.contains("pub enum Side") && !side.contains(&format!("{open}\npub enum Side")));
    assert!(shapes.contains(&format!("{open}\npub enum StatusOfThingsMode")));
    assert!(shapes.contains("#[serde(untagged)]\n    Unrecognised(String),"));
    assert!(module.contains(&format!("{open}\npub enum Payload")));
    Ok(())
}

#[test]
fn what_is_generated_is_held_to_what_is_committed_file_by_file() -> Outcome {
    let root = laid_out(&tree())?;
    let said = run(&words(&["generate"]), root.path())?;
    assert!(said.contains(REVISION));
    run(&words(&["check"]), root.path())?;
    let generated = root.path().join(GENERATED);
    let shapes = fs::read_to_string(generated.join("shapes_s.rs"))?;
    for written in [
        "pub r#as: String,",
        "#[serde(rename = \"self\")]",
        "pub data_location: Option<String>,",
        "pub small: Option<u8>,",
        "pub size: Option<u64>,",
        "pub ratio: Option<f64>,",
        "pub nothing: Option<()>,",
        "pub anything: Option<serde_json::Value>,",
        "pub pair: Option<(i64, String)>,",
        "pub cause: Option<Box<super::StatusOfThings>>,",
        "pub by_name: Option<std::collections::BTreeMap<String, super::Level>>,",
        "pub loose: Option<std::collections::BTreeMap<String, serde_json::Value>>,",
        "pub inner: Option<super::StatusOfThingsInner>,",
        "pub either: Option<super::StatusOfThingsEither>,",
        "pub default: Option<super::Code>,",
        "pub mode: Option<super::StatusOfThingsMode>,",
    ] {
        assert!(shapes.contains(written), "{written}");
    }
    let module = fs::read_to_string(generated.join("mod.rs"))?;
    assert!(module.contains("Pull(Box<String>)") && module.contains("Status(Box<StatusOfThings>)"));
    for declared in [
        "mod refusals_life;",
        "pub use super::refusals_life::*;",
        "pub use super::action_names::NAMES;",
        "pub use super::refusal_list::ALL;",
        "pub use super::reads_b::*;",
    ] {
        assert!(module.contains(declared), "{declared}");
    }
    let status = fs::read_to_string(generated.join("reads_s.rs"))?;
    assert!(status.contains("#[serde(rename = \"self\")]"));
    assert_eq!(
        status
            .matches("impl crate::Answer for super::StatusOfThings")
            .count(),
        1
    );
    let bundle = fs::read_to_string(generated.join("reads_b.rs"))?;
    assert!(bundle.contains("impl crate::FileRead for BundleName"));
    let held = fs::read_to_string(generated.join("reads_h.rs"))?;
    assert!(held.contains("pub enum HeldIdAnswer"));
    let pulled = fs::read_to_string(generated.join("reads_p.rs"))?;
    assert!(pulled.contains("pub enum PulledAnswer"));
    let restart = fs::read_to_string(generated.join("actions_r.rs"))?;
    assert!(restart.contains(
        "#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]\npub struct Restart"
    ));
    assert_eq!(restart.matches("pub dry_run: Option<bool>,").count(), 1);
    let diagnose = fs::read_to_string(generated.join("actions_d.rs"))?;
    assert_eq!(diagnose.matches("pub dry_run").count(), 1);
    let refusals = fs::read_to_string(generated.join("refusal_list.rs"))?;
    assert!(refusals.find("BUSY") < refusals.find("OFFER_MOVED"));
    let callable = fs::read_to_string(generated.join("key_callable.rs"))?;
    assert!(
        callable.contains("Some(&super::refusals::OFFER_MOVED)")
            && callable.contains("moved: None")
    );

    fs::write(generated.join("actions_r.rs"), "edited")?;
    fs::remove_file(generated.join("refusals_life.rs"))?;
    fs::write(generated.join("stray.rs"), "")?;
    let said = refused_with(root.path(), &["check"]);
    for problem in [
        "actions_r.rs differs",
        "refusals_life.rs is not committed",
        "stray.rs is generated by nothing",
    ] {
        assert!(said.contains(problem), "{said}");
    }
    run(&words(&["generate"]), root.path())?;
    assert!(!generated.join("stray.rs").exists());
    run(&words(&["check"]), root.path())?;
    Ok(())
}

#[test]
fn a_version_this_does_not_implement_is_refused_naming_both_and_nothing_is_written() -> Outcome {
    let root = edited(set("index.json", "/api_version", json!(2)))?;
    let said = refused_with(root.path(), &["generate"]);
    assert!(said.contains("api_version 2") && said.contains("implements 1"));
    assert!(!root.path().join(GENERATED).exists());
    let root = edited(|files| {
        if let Some(index) = files.get_mut("index.json").and_then(Value::as_object_mut) {
            index.remove("api_version");
        }
    })?;
    assert!(refused_with(root.path(), &["generate"]).contains("api_version none"));
    Ok(())
}

fn refuses(edit: impl FnOnce(&mut BTreeMap<String, Value>), saying: &str) -> Outcome {
    let root = edited(edit)?;
    let said = refused_with(root.path(), &["generate"]);
    assert!(said.contains(saying), "expected {saying:?} in {said:?}");
    assert!(!root.path().join(GENERATED).exists());
    Ok(())
}

fn remove(
    file: &'static str,
    pointer: &'static str,
    key: &'static str,
) -> impl FnOnce(&mut BTreeMap<String, Value>) {
    move |files| {
        if let Some(object) = at(files, file, pointer).as_object_mut() {
            object.remove(key);
        }
    }
}

#[test]
fn a_reference_that_reaches_nothing_or_reads_two_ways_is_refused_naming_where() -> Outcome {
    for (reference, saying) in [
        ("Missing.json", "`Missing.json` reaches no definition"),
        ("../../../Level.json", "reaches no definition"),
        ("../kinds/status.json", "reaches no definition"),
        ("Level", "reaches no definition"),
        ("lower.json", "reaches no definition"),
        ("x/y/Level.json", "reaches no definition"),
    ] {
        refuses(
            set(
                "defs/Status_of_Things.json",
                "/properties/level/$ref",
                json!(reference),
            ),
            saying,
        )?;
    }
    refuses(
        set(
            "defs/Status_of_Things.json",
            "/properties/level",
            json!({ "$ref": "Level.json", "type": "string" }),
        ),
        "has type beside it",
    )
}

#[test]
fn an_index_or_list_this_does_not_read_is_refused() -> Outcome {
    refuses(
        set("index.json", "/events", json!("events.json")),
        "the index carries events",
    )?;
    refuses(
        set("index.json", "/kinds", json!({})),
        "the index lists no kinds",
    )?;
    refuses(
        set("index.json", "/kinds/status", json!(1)),
        "the kind status names no file",
    )?;
    refuses(
        set("index.json", "/kinds/Status", json!("kinds/status.json")),
        "is not lower-case words",
    )?;
    refuses(
        set("kinds/pull.json", "/properties", json!({})),
        "is not an envelope",
    )?;
    refuses(
        remove("index.json", "", "reads"),
        "names no file for the reads",
    )?;
    refuses(
        remove("index.json", "", "key_callable"),
        "names no file for the actions a key may call",
    )?;
    refuses(
        set("index.json", "/reads", json!("missing.json")),
        "missing.json could not be read",
    )?;
    let root = edited(|files| {
        files.insert("index.json".to_owned(), json!([]));
    })?;
    assert!(refused_with(root.path(), &["generate"]).contains("the index is not an object"));
    let root = edited(|_| {})?;
    fs::write(root.path().join(CONTRACT).join("reads.json"), "[")?;
    assert!(refused_with(root.path(), &["generate"]).contains("reads.json is not JSON"));
    Ok(())
}

#[test]
fn an_action_a_body_or_a_refusal_this_cannot_write_is_refused() -> Outcome {
    refuses(
        set("index.json", "/actions/restart", json!(1)),
        "the action restart names no file",
    )?;
    refuses(
        set("actions/restart.json", "/action", json!("other")),
        "does not describe the action restart",
    )?;
    refuses(
        set(
            "actions/restart.json",
            "/arguments/0",
            json!({ "name": "" }),
        ),
        "an argument names no name and type",
    )?;
    refuses(
        set("actions/restart.json", "/consent", json!([1])),
        "consent is not a list of words",
    )?;
    refuses(
        set("actions/restart.json", "/rehearsal", json!("yes")),
        "rehearsal is not true or false",
    )?;
    refuses(
        set("actions/restart.json", "/extra", json!(1)),
        "carries extra",
    )?;
    refuses(
        set("index.json", "/actions/restart", json!("reads.json")),
        "reads.json is not an object",
    )?;
    refuses(
        set("index.json", "/bodies/~1api~1setup~1answer", json!(1)),
        "names no file",
    )?;
    refuses(
        set(
            "index.json",
            "/bodies/Setup",
            json!("bodies/setup-answer.json"),
        ),
        "is not taken at a route of words",
    )?;
    refuses(
        set("index.json", "/refusals/LIFE", json!(1)),
        "the refusals of LIFE name no file",
    )?;
    refuses(
        set("refusals/LIFE.json", "/LIFE-9/name", json!("busy")),
        "is not a code of LIFE",
    )?;
    refuses(
        set("refusals/LIFE.json", "/LIFE-9/status", json!(200)),
        "its status is not a refusal's",
    )?;
    refuses(
        set("refusals/LIFE.json", "/LIFE-9/name", json!("OFFER_MOVED")),
        "the name OFFER_MOVED is another code's too",
    )?;
    refuses(
        set("refusals/LIFE.json", "/LIFE-9", json!(1)),
        "LIFE-9 is not an object",
    )?;
    refuses(
        set("refusals/LIFE.json", "/LIFE-9/extra", json!(1)),
        "carries extra",
    )?;
    refuses(
        set(
            "refusals/LIFE.json",
            "/ADMIT-1",
            json!({ "name": "X", "status": 400, "description": "Raised." }),
        ),
        "is not a code of LIFE",
    )
}

#[test]
fn an_enum_a_stack_answers_with_cannot_take_the_name_written_for_a_new_value() -> Outcome {
    refuses(
        set(
            "defs/Panel.json",
            "/oneOf/1/properties/panel/const",
            json!("Unrecognised"),
        ),
        "Panel holds a variant named Unrecognised",
    )
}

#[test]
fn a_family_of_refusals_that_could_not_name_a_file_is_refused() -> Outcome {
    refuses(
        |files| {
            set(
                "index.json",
                "/refusals/..~1LIFE",
                json!("refusals/outside.json"),
            )(files);
            files.insert(
                "refusals/outside.json".to_owned(),
                json!({ "../LIFE-1": { "name": "OUTSIDE", "status": 400, "description": "Raised." } }),
            );
        },
        "is not a code of ../LIFE",
    )
}

#[test]
fn a_group_too_long_for_one_file_is_split_between_its_items() -> Outcome {
    let items: Vec<String> = (0..400)
        .map(|at| {
            format!("/// Item {at}.\npub struct Item{at} {{\n/// Held.\npub held: u8,\n}}\n\n")
        })
        .collect();
    let groups = BTreeMap::from([
        ("i".to_owned(), items),
        (
            "s".to_owned(),
            vec!["/// Short.\npub struct Short;\n".to_owned()],
        ),
    ]);
    let pages = paged("shapes", &groups)?;
    let names: Vec<&str> = pages.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(
        names,
        ["shapes_i_1", "shapes_i_2", "shapes_i_3", "shapes_s"]
    );
    let mut every = String::new();
    for (_, page) in &pages {
        assert!(page.lines().count() <= 900);
        assert!(!page.contains("// cut"));
        assert_eq!(&formatted(page)?, page);
        every.push_str(page);
    }
    for at in 0..400 {
        assert!(every.contains(&format!("pub struct Item{at} {{")), "{at}");
    }
    Ok(())
}

#[test]
fn a_read_or_a_key_callable_action_this_cannot_write_is_refused() -> Outcome {
    refuses(
        set("index.json", "/reads", json!("index.json")),
        "the reads are not a list",
    )?;
    refuses(set("reads.json", "/0", json!(1)), "read 0 is not an object")?;
    refuses(
        set("reads.json", "/0/path", json!("/elsewhere")),
        "is not a read's path",
    )?;
    refuses(
        set("reads.json", "/0/path", json!("/api/x/{Bad}")),
        "is not a read's path",
    )?;
    refuses(
        set("reads.json", "/0/parameters/0", json!({ "name": "form" })),
        "is not a name and whether it repeats",
    )?;
    refuses(
        set("reads.json", "/2/parameters/0/name", json!("id")),
        "is not a name and whether it repeats",
    )?;
    refuses(
        set("reads.json", "/0/kinds", json!(["nothing"])),
        "which the contract describes no kind as",
    )?;
    refuses(
        set("reads.json", "/0/file", json!("no")),
        "file is not true or false",
    )?;
    refuses(
        set("reads.json", "/0/file", json!(true)),
        "answers with a file and kinds, or with neither",
    )?;
    refuses(
        set("reads.json", "/1/path", json!("/api/status")),
        "/api/status is listed twice",
    )?;
    refuses(set("reads.json", "/0/extra", json!(1)), "carries extra")?;
    refuses(
        set("reads.json", "/1/path", json!("/api/held-id")),
        "which another read takes",
    )?;
    refuses(
        set("key-callable.json", "/0", json!(1)),
        "key-callable entry 0 is not an object",
    )?;
    refuses(
        set("key-callable.json", "/0/disturbs", json!(1)),
        "are not each true or false",
    )?;
    refuses(
        set("key-callable.json", "/0/action", json!("nothing")),
        "is not an action the contract lists",
    )?;
    refuses(
        set("key-callable.json", "/0/moved", json!("LIFE-1")),
        "moved LIFE-1 is not a refusal the contract lists",
    )
}

#[test]
fn a_shape_this_cannot_write_is_refused_naming_it() -> Outcome {
    let property = |schema: Value| set("defs/Status_of_Things.json", "/properties/as", schema);
    refuses(
        property(json!({ "type": ["string", "integer"] })),
        "a value of several types",
    )?;
    refuses(
        property(json!({ "type": "integer", "format": "int128" })),
        "an integer of a format",
    )?;
    refuses(
        property(json!({ "type": "decimal" })),
        "a type this generator does not know",
    )?;
    refuses(
        property(
            json!({ "type": "object", "properties": { "a": {} }, "additionalProperties": { "type": "string" } }),
        ),
        "an object with fields and further keys",
    )?;
    refuses(
        set("defs/Status_of_Things.json", "/properties/Self", json!({})),
        "is written as a name another field takes",
    )?;
    refuses(
        set("defs/Status_of_Things.json", "/properties/!", json!({})),
        "is written as a name another field takes",
    )?;
    refuses(
        set("defs/Level.json", "/oneOf/1/enum/0", json!("fine")),
        "two variants are written as \"Fine\"",
    )?;
    refuses(
        |files| {
            files.insert(
                "defs/StatusOfThingsInner.json".to_owned(),
                json!({ "type": "object", "properties": {} }),
            );
            *at(
                files,
                "defs/Status_of_Things.json",
                "/properties/level/$ref",
            ) = json!("StatusOfThingsInner.json");
        },
        "StatusOfThingsInner: a definition takes the name",
    )?;
    refuses(
        |files| {
            files.insert("defs/Payload.json".to_owned(), json!({ "type": "string" }));
            *at(files, "defs/Status_of_Things.json", "/properties/code/$ref") =
                json!("Payload.json");
        },
        "the definition Payload takes a name this generator writes itself; it is carried by status",
    )?;
    refuses(
        |files| {
            files.insert("defs/Payload.json".to_owned(), json!({ "type": "string" }));
            *at(files, "actions/restart.json", "/arguments/2/type/$ref") =
                json!("../defs/Payload.json");
        },
        "it is carried by no kind",
    )?;
    refuses(
        |files| {
            files.insert("defs/String.json".to_owned(), json!({ "type": "string" }));
            *at(files, "defs/Status_of_Things.json", "/properties/code/$ref") =
                json!("String.json");
        },
        "the definition String takes a name this generator writes itself; it is carried by status",
    )
}

fn written_outside_the_shapes() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(repository().join(GENERATED))? {
        let path = entry?.path();
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if stem.starts_with("shapes_") {
            continue;
        }
        for line in fs::read_to_string(&path)?.lines() {
            let declared = line
                .strip_prefix("pub struct ")
                .or_else(|| line.strip_prefix("pub enum "))
                .and_then(|rest| rest.split([' ', '{', '(', ';']).next());
            names.extend(declared.map(str::to_owned));
        }
    }
    Ok(names)
}

#[test]
fn every_type_written_beside_the_shapes_is_a_name_a_definition_is_refused() -> Outcome {
    let vendored = Artefact::read(&repository())?;
    let names = written_outside_the_shapes()?;
    assert!(names.iter().any(|name| name == "Payload"));
    for name in names {
        let mut taking = vendored.clone();
        taking
            .defs
            .insert(name.clone(), json!({ "type": "string" }));
        let said = shaped(&taking)
            .err()
            .map(|refused| refused.to_string())
            .unwrap_or_default();
        let expected = format!("the definition {name} takes a name this generator writes itself");
        assert!(
            said.contains(&expected),
            "expected {expected:?} in {said:?}"
        );
    }
    Ok(())
}

#[test]
fn a_revision_names_one_artefact_for_good() -> Outcome {
    let root = edited(|_| {})?;
    for (stamp, saying) in [
        ("abc123", "is not a release tag"),
        ("v1.2", "is not a release tag"),
    ] {
        fs::write(root.path().join(STAMP), stamp)?;
        assert!(refused_with(root.path(), &["generate"]).contains(saying));
    }
    fs::write(root.path().join(STAMP), "v1.20.3\n")?;
    assert!(run(&words(&["generate"]), root.path())?.contains("v1.20.3"));
    fs::remove_file(root.path().join(STAMP))?;
    assert!(refused_with(root.path(), &["generate"]).contains("contract/VERSION could not be read"));
    Ok(())
}

#[test]
fn a_contract_is_taken_whole_from_what_was_extracted_with_its_revision() -> Outcome {
    let from = laid_out(&tree())?;
    let extracted = from.path().join(CONTRACT);
    let into = edited(|files| {
        files.insert("defs/Stale.json".to_owned(), json!({}));
    })?;
    let said = run(
        &words(&["take", &extracted.to_string_lossy(), "v2.0.0"]),
        into.path(),
    )?;
    assert!(said.contains("v2.0.0") && said.contains("just generate"));
    assert!(!into.path().join(CONTRACT).join("defs/Stale.json").exists());
    assert_eq!(fs::read_to_string(into.path().join(STAMP))?, "v2.0.0\n");
    let fresh = tempfile::tempdir()?;
    fs::create_dir_all(fresh.path().join("contract"))?;
    run(
        &words(&["take", &extracted.to_string_lossy(), REVISION]),
        fresh.path(),
    )?;
    assert!(fresh.path().join(CONTRACT).join("index.json").exists());
    assert!(refused_with(
        into.path(),
        &["take", &extracted.to_string_lossy(), "abc123"]
    )
    .contains("abbreviated"));
    fs::remove_file(extracted.join("reads.json"))?;
    let said = refused_with(
        into.path(),
        &["take", &extracted.to_string_lossy(), REVISION],
    );
    assert!(said.contains("missing files its index names") && said.contains("reads.json"));
    Ok(())
}

fn page(reads: &str) -> String {
    format!("# Contract\n\n## Reading\n\nEach command:\n\n```\n{reads}\n```\n\n### A part of it\n\n```text\nGET /api/version\n```\n\n## Live state\n\n```\nGET /api/events\n```\n")
}

fn spec_with(text: &str) -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let spec = tempfile::tempdir()?;
    let at = spec.path().join("20-architecture/contracts");
    fs::create_dir_all(&at)?;
    fs::write(at.join("web-api.md"), text)?;
    Ok(spec)
}

#[test]
fn the_reads_this_client_reaches_are_held_to_the_contract_page_both_ways() -> Outcome {
    let listed: Value = serde_json::from_str(&fs::read_to_string(
        repository().join(CONTRACT).join("reads.json"),
    )?)?;
    let paths: Vec<String> = listed
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|read| read.get("path").and_then(Value::as_str))
        .map(|path| format!("GET {path}?…"))
        .collect();
    let spec = spec_with(&page(&paths.join("   ")))?;
    let spec_path = spec.path().to_string_lossy().into_owned();
    let said = run(&words(&["reads", "--spec", &spec_path]), &repository())?;
    assert!(said.contains(&format!("each of the {} reads", paths.len())));

    let fewer = spec_with(&page(
        &paths.iter().skip(2).cloned().collect::<Vec<_>>().join("\n"),
    ))?;
    let said = refused_with(
        &repository(),
        &["reads", "--spec", &fewer.path().to_string_lossy()],
    );
    assert!(said.contains("does not name them"), "{said}");
    let more = spec_with(&page(&format!("{}\nGET /api/new-read", paths.join("\n"))))?;
    let said = refused_with(
        &repository(),
        &["reads", "--spec", &more.path().to_string_lossy()],
    );
    assert!(said.contains("holds no read for them: /api/new-read"));
    let none = spec_with(&page("nothing fenced here"))?;
    assert!(refused_with(
        &repository(),
        &["reads", "--spec", &none.path().to_string_lossy()]
    )
    .contains("fewer than the 25"));
    let headless = spec_with("# Contract\n")?;
    assert!(refused_with(
        &repository(),
        &["reads", "--spec", &headless.path().to_string_lossy()]
    )
    .contains("has no `## Reading` heading"));
    let unindexed = edited(remove("index.json", "", "reads"))?;
    assert!(
        refused_with(unindexed.path(), &["reads", "--spec", &spec_path])
            .contains("names no file for the reads")
    );
    assert!(
        refused_with(&repository(), &["reads", "--spec", "/nowhere"]).contains("could not be read")
    );
    Ok(())
}

#[test]
fn a_generated_module_that_cannot_be_written_says_where() -> Outcome {
    let root = laid_out(&tree())?;
    fs::create_dir_all(root.path().join("crates/lemonfiber-sdk/src"))?;
    fs::write(root.path().join(GENERATED), "a file where the module goes")?;
    assert!(refused_with(root.path(), &["generate"]).contains("could not be written"));
    Ok(())
}

#[test]
fn what_is_generated_must_be_rust_or_nothing_is_written() -> Outcome {
    refuses(
        |files| {
            files.insert(
                "defs/Self.json".to_owned(),
                json!({ "type": "object", "properties": {} }),
            );
            *at(
                files,
                "defs/Status_of_Things.json",
                "/properties/level/$ref",
            ) = json!("Self.json");
        },
        "rustfmt could not format the generated module",
    )
}

#[test]
fn a_name_is_written_in_the_case_rust_writes_it() {
    use crate::names::{field, pascal, screaming, type_name};
    assert_eq!(pascal("frontDoor"), "FrontDoor");
    assert_eq!(pascal("4k-hdr"), "N4kHdr");
    assert_eq!(screaming("not-admitted"), "NOT_ADMITTED");
    assert_eq!(field("2fa"), "2fa_");
    assert_eq!(field("type"), "r#type");
    assert_eq!(type_name("PanelArray_of_Queue"), "PanelArrayOfQueue");
    assert_eq!(type_name("A__b"), "AB");
}
