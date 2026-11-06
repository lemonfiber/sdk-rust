//! Writes the `contract` module of `lemonfiber-sdk` from the vendored web API artefact,
//! holds what is committed to what it writes, holds the reads the client reaches to the
//! contract page, and vendors an artefact taken from a lemonfiber revision.

mod artefact;
mod emit;
mod files;
mod names;
mod reading;
mod refused;
mod schema;
mod vendor;

#[cfg(test)]
mod tests;

use std::path::Path;

pub use refused::Refused;

/// Where the vendored artefact sits, beneath the repository root.
pub const CONTRACT: &str = "contract/web-api";

/// The file naming the lemonfiber revision the artefact was taken from.
pub const STAMP: &str = "contract/VERSION";

/// Where the generated module is written, beneath the repository root.
pub const GENERATED: &str = "crates/lemonfiber-sdk/src/contract";

const USAGE: &str = "lemonfiber-sdk-generate takes one of: `generate`, `check`, \
`reads --spec <spec checkout>`, or `take <extracted artefact> <revision>`.";

/// Runs one command against the repository at `root`, answering what it did or why it
/// stopped. A command that stops writes nothing.
///
/// # Errors
///
/// Every reason the command stopped, in a sentence naming what to change.
pub fn run(arguments: &[String], root: &Path) -> Result<String, Refused> {
    let words: Vec<&str> = arguments.iter().map(String::as_str).collect();
    match words.as_slice() {
        ["generate"] => {
            let written = generated(root)?;
            files::write(&root.join(GENERATED), &written)?;
            Ok(format!(
                "wrote {} files into {GENERATED} from the contract at {}",
                written.len(),
                artefact::revision(root)?
            ))
        }
        ["check"] => {
            let written = generated(root)?;
            files::compare(&root.join(GENERATED), &written)?;
            Ok(format!(
                "{GENERATED} is what the contract generates: {} files",
                written.len()
            ))
        }
        ["reads", "--spec", spec] => reading::check(root, Path::new(spec)),
        ["take", from, revision] => vendor::take(root, Path::new(from), revision),
        _ => Err(Refused::new(USAGE)),
    }
}

fn generated(root: &Path) -> Result<Vec<(String, String)>, Refused> {
    let read = artefact::Artefact::read(root)?;
    let shaped = schema::shaped(&read)?;
    emit::files(&read, &shaped)
}
