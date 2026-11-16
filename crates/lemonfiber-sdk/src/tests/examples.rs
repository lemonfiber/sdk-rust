use std::path::Path;

use serde_json::Value;

use super::Outcome;
use crate::envelope::{raw, Raw};

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
