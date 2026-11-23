use std::path::Path;

use serde_json::{json, Map, Value};

use super::Outcome;
use crate::contract::API_VERSION;
use crate::envelope::{raw, Raw};
use crate::Envelope;

/// The vendored contract, as the generator reads it.
fn contract() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../contract/web-api")
}

#[test]
fn every_example_of_a_kind_the_contract_carries_reads_as_its_typed_payload() -> Outcome {
    let index: Value = serde_json::from_slice(&std::fs::read(contract().join("index.json"))?)?;
    let examples = index
        .pointer("/examples/kinds")
        .and_then(Value::as_object)
        .ok_or("the vendored index carries no examples of its kinds")?;
    assert!(!examples.is_empty());
    for (kind, file) in examples {
        let file = file.as_str().ok_or("an example names no file")?;
        let body = std::fs::read(contract().join(file))?;
        let read = raw(&body)
            .and_then(Raw::payload)
            .map_err(|why| format!("{kind}: {why}"))?;
        assert_eq!(read.kind, *kind);
    }
    Ok(())
}

/// Every field `envelope` carries, as the wire wrote it.
fn carried(envelope: &Envelope<Value>) -> Value {
    json!({
        "api_version": envelope.api_version,
        "kind": envelope.kind,
        "host": envelope.host,
        "job": envelope.job,
        "data": envelope.data,
    })
}

#[test]
fn every_top_level_property_a_kind_declares_is_one_the_envelope_carries() -> Outcome {
    let kinds: Vec<std::path::PathBuf> = std::fs::read_dir(contract().join("kinds"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()?;
    assert!(!kinds.is_empty(), "the vendored contract carries no kinds");
    for path in kinds {
        let kind = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("a kind's file has no name")?;
        let schema: Value = serde_json::from_slice(&std::fs::read(&path)?)?;
        let declared = schema
            .get("properties")
            .and_then(Value::as_object)
            .ok_or_else(|| format!("{kind} declares no properties"))?;
        let said: Map<String, Value> = declared
            .keys()
            .map(|field| {
                let value = match field.as_str() {
                    "api_version" => json!(API_VERSION),
                    "kind" => json!(kind),
                    "data" => json!({ "of": kind }),
                    _ => json!(format!("{field} of {kind}")),
                };
                (field.clone(), value)
            })
            .collect();
        let envelope = raw(&serde_json::to_vec(&said)?)?.expect::<Value>(&[kind], false)?;
        let carried = carried(&envelope);
        for (field, value) in &said {
            assert_eq!(
                carried.get(field),
                Some(value),
                "{kind} declares `{field}`, and the envelope does not carry it"
            );
        }
    }
    Ok(())
}
