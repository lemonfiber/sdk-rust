use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use crate::{files, Refused, CONTRACT};

const PAGE: &str = "20-architecture/contracts/web-api.md";

const HEADING: &str = "## Reading";

const VERB: &str = "GET ";

const SERVED: &str = "/api/";

const FEWEST: usize = 25;

fn named(page: &str) -> Result<BTreeSet<String>, Refused> {
    let Some((_, after)) = page.split_once(&format!("\n{HEADING}\n")) else {
        return Err(Refused::new(format!("{PAGE} has no `{HEADING}` heading")));
    };
    let section = after.split("\n## ").next().unwrap_or_default();
    let mut fenced = String::new();
    for (at, block) in section.split("```").enumerate() {
        if at % 2 == 1 {
            fenced.push_str(block);
            fenced.push('\n');
        }
    }
    let mut paths = BTreeSet::new();
    let mut rest = fenced.as_str();
    let entry = format!("{VERB}{SERVED}");
    while let Some(at) = rest.find(&entry) {
        let from = rest.get(at + VERB.len()..).unwrap_or_default();
        let path: String = from
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "{}/-_".contains(*c))
            .collect();
        paths.insert(path);
        rest = from;
    }
    Ok(paths)
}

fn held(root: &Path) -> Result<BTreeSet<String>, Refused> {
    let index = files::json(&root.join(CONTRACT).join("index.json"), "the index")?;
    let Some(listed) = index.get("reads").and_then(Value::as_str) else {
        return Err(Refused::new("the index names no file for the reads"));
    };
    let reads = files::json(&root.join(CONTRACT).join(listed), listed)?;
    Ok(reads
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|read| read.get("path").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

pub fn check(root: &Path, spec: &Path) -> Result<String, Refused> {
    let page = files::text(&spec.join(PAGE), &format!("{PAGE} in {}", spec.display()))?;
    let named = named(&page)?;
    let held = held(root)?;
    let mut problems = Vec::new();
    for (count, what) in [
        (named.len(), format!("reads under `{HEADING}` in {PAGE}")),
        (held.len(), "reads in the vendored artefact".to_owned()),
    ] {
        if count < FEWEST {
            problems.push(format!(
                "read {count} {what}, fewer than the {FEWEST} there have never been fewer than; the text has changed shape and this is no longer reading it"
            ));
        }
    }
    let unreached: Vec<&str> = named.difference(&held).map(String::as_str).collect();
    if !unreached.is_empty() {
        problems.push(format!(
            "`{HEADING}` names these and this client holds no read for them: {}",
            unreached.join(", ")
        ));
    }
    let unnamed: Vec<&str> = held.difference(&named).map(String::as_str).collect();
    if !unnamed.is_empty() {
        problems.push(format!(
            "this client holds these reads and `{HEADING}` does not name them: {}",
            unnamed.join(", ")
        ));
    }
    if problems.is_empty() {
        Ok(format!(
            "this client holds a read for each of the {} reads the contract page names, and no other",
            named.len()
        ))
    } else {
        Err(Refused::listing(
            "the reads this client reaches are not the ones the contract page lists",
            &problems,
        ))
    }
}
