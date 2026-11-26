use std::path::Path;

use serde_json::{json, Value};

use super::unions::kept;
use super::Outcome;
use crate::contract::{InvitationApplied, Policy, ReaderDefault, SettingDefault, SettingMove};
use crate::open::Tags;

/// A definition of the vendored contract, as it writes it.
fn definition(name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let at = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../contract/web-api/defs")
        .join(format!("{name}.json"));
    Ok(serde_json::from_str(&std::fs::read_to_string(at)?)?)
}

fn required(definition: &Value) -> Vec<&str> {
    definition
        .pointer("/required")
        .and_then(Value::as_array)
        .map(|names| names.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn said(fallback: &SettingDefault) -> Value {
    match fallback {
        SettingDefault::Core { value } => json!(format!("core: {value}")),
        SettingDefault::Declared { value } => json!(format!("declared: {value}")),
        SettingDefault::PerReader { readers } => readers
            .iter()
            .map(|reader: &ReaderDefault| {
                let met = if reader.met { "met" } else { "unmet" };
                let default = reader.default.as_deref().unwrap_or("declares none");
                json!(format!("{} {met}: {default}", reader.service))
            })
            .collect(),
        SettingDefault::None => json!("none"),
        SettingDefault::Unrecognised(found) => json!({ "unrecognised": found }),
    }
}

#[test]
fn a_moved_settings_default_is_typed_by_every_tag_the_contract_gives_it_and_required() -> Outcome {
    let tags: Vec<String> = definition("SettingDefault")?
        .pointer("/oneOf")
        .and_then(Value::as_array)
        .map(|branches| {
            branches
                .iter()
                .filter_map(|branch| {
                    branch
                        .pointer("/properties/of/const")
                        .and_then(Value::as_str)
                })
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(tags, SettingDefault::KNOWN);
    assert_eq!(SettingDefault::TAG, Some("of"));
    assert!(required(&definition("SettingMove")?).contains(&"default"));
    assert!(!required(&definition("ReaderDefault")?).contains(&"default"));
    Ok(())
}

#[test]
fn a_moved_settings_default_reads_by_what_it_is_of_keeping_each_readers_own_and_its_absence(
) -> Outcome {
    let moves = [
        json!({ "setting": "LEMONFIBER_BACKUP_LIMIT", "value": "4", "default": { "of": "core", "value": "off" } }),
        json!({ "setting": "sonarr:port", "was": "9000", "default": { "of": "declared", "value": "8989" } }),
        json!({ "setting": "gluetun:countries", "value": "Belgium", "default": { "of": "per-reader", "readers": [
            { "service": "gluetun", "variable": "SERVER_COUNTRIES", "default": "Netherlands", "met": true },
            { "service": "qbittorrent", "variable": "VPN_COUNTRIES", "met": false }
        ] } }),
        json!({ "setting": "jellyfin:published", "default": { "of": "none" } }),
        json!({ "setting": "sonarr:url", "default": { "of": "inherited", "from": "sonarr" } }),
    ];
    let mut read = Vec::new();
    for move_said in &moves {
        let (taken, written) = kept::<SettingMove>(move_said)?;
        assert_eq!(&written, move_said);
        read.push(said(&taken.default));
    }
    assert_eq!(
        read,
        [
            json!("core: off"),
            json!("declared: 8989"),
            json!([
                "gluetun met: Netherlands",
                "qbittorrent unmet: declares none"
            ]),
            json!("none"),
            json!({ "unrecognised": { "of": "inherited", "from": "sonarr" } }),
        ]
    );
    assert!(serde_json::from_value::<SettingMove>(json!({ "setting": "sonarr:port" })).is_err());
    Ok(())
}

#[test]
fn what_an_invitation_holds_their_asking_to_is_typed_as_the_households_policy_and_optional(
) -> Outcome {
    let policies: Vec<Policy> = definition("Policy")?
        .pointer("/oneOf")
        .and_then(Value::as_array)
        .map(|branches| {
            branches
                .iter()
                .filter_map(|branch| {
                    branch
                        .get("const")
                        .and_then(|tag| serde_json::from_value(tag.clone()).ok())
                })
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(
        policies,
        [
            Policy::Trusted,
            Policy::WithinALimit,
            Policy::EverythingWaits
        ]
    );
    let applied = definition("InvitationApplied")?;
    assert!(applied
        .pointer("/properties/asking")
        .is_some_and(Value::is_object));
    assert!(!required(&applied).contains(&"asking"));
    Ok(())
}

#[test]
fn a_rehearsal_reads_the_policy_it_would_hold_their_asking_to_and_its_absence_as_no_policy(
) -> Outcome {
    let applied = json!({
        "filtering": "A limit hides what is rated above it; it is not a filter on what is unrated.",
        "libraries": ["Films"],
        "limit": "PG",
        "requesting": "made",
        "unrated": "held-back"
    });
    let mut held = applied.clone();
    if let Some(fields) = held.as_object_mut() {
        fields.insert("asking".to_owned(), json!("everything-waits"));
    }
    for (said, asking) in [(held, Some(Policy::EverythingWaits)), (applied, None)] {
        let (read, written) = kept::<InvitationApplied>(&said)?;
        assert_eq!(read.asking, asking);
        assert_eq!(written, said);
    }
    Ok(())
}
