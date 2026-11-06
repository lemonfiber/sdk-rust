use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::artefact::is_revision;
use crate::{files, Refused, CONTRACT, STAMP};

pub fn take(root: &Path, from: &Path, revision: &str) -> Result<String, Refused> {
    if !is_revision(revision) {
        return Err(Refused::new(format!(
            "{revision:?} is not a release tag or a full commit hash; an abbreviated one may name another revision later"
        )));
    }
    let index = files::json(&from.join("index.json"), "the index of what was extracted")?;
    let mut named: Vec<String> = Vec::new();
    for list in ["kinds", "actions", "bodies", "refusals"] {
        named.extend(
            index
                .get(list)
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                .filter_map(|(_, file)| file.as_str().map(str::to_owned)),
        );
    }
    for list in ["reads", "key_callable"] {
        named.extend(index.get(list).and_then(Value::as_str).map(str::to_owned));
    }
    let held = files::beneath(from);
    let missing: Vec<&str> = named
        .iter()
        .map(String::as_str)
        .filter(|file| file.contains("..") || !held.contains(*file))
        .collect();
    if !missing.is_empty() {
        return Err(Refused::listing(
            &format!("what was extracted at {revision} is missing files its index names, and nothing was written"),
            &missing.iter().map(|file| (*file).to_owned()).collect::<Vec<_>>(),
        ));
    }
    let into = root.join(CONTRACT);
    let mut taken = Vec::new();
    for file in &held {
        taken.push((file.clone(), files::text(&from.join(file), file)?));
    }
    files::write(&into, &taken)?;
    let stamp = root.join(STAMP);
    files::written(fs::write(&stamp, format!("{revision}\n")), &stamp)?;
    Ok(format!(
        "took the contract at {revision}: {} files. Now run `just generate`, and commit contract/ and the generated module.",
        held.len()
    ))
}
