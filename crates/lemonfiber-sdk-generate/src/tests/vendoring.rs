use std::fs;

use serde_json::json;

use super::{edited, laid_out, refused_with, tree, words, Outcome, REVISION};
use crate::{run, CONTRACT, STAMP};

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
