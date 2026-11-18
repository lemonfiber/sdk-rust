use std::fs;

use serde_json::json;

use crate::tests::{edited, laid_out, refused_with, refuses, remove, set, tree, words};
use crate::{run, CONTRACT, GENERATED, STAMP};

type Outcome = Result<(), Box<dyn std::error::Error>>;

#[test]
fn the_household_words_are_written_as_the_contract_publishes_them() -> Outcome {
    let root = laid_out(&tree())?;
    run(&words(&["generate"]), root.path())?;
    let generated = root.path().join(GENERATED);
    let written = fs::read_to_string(generated.join("household_words.rs"))?;
    for list in [
        "pub const NAMES: &[&str] = &[\"sonarr\"];",
        "pub const TERMS: &[&str] = &[\"container\"];",
        "pub const SIZES: &[&str] = &[\"gb\"];",
        "pub const MARKS: &[&str] = &[\"://\"];",
        "crate::DeclaredName {\n    name: \"stack\",\n    names: \"names the machine\",\n}",
        "crate::HouseholdExample {\n    text: \"Sonarr is down\",\n    names: \"sonarr\",\n}",
    ] {
        assert!(written.contains(list), "{list:?} in {written}");
    }
    assert!(fs::read_to_string(generated.join("mod.rs"))?.contains("pub mod household_words;"));
    Ok(())
}

#[test]
fn a_contract_publishing_no_household_words_writes_empty_lists() -> Outcome {
    let root = edited(remove("index.json", "", "household_words"))?;
    run(&words(&["generate"]), root.path())?;
    let written = fs::read_to_string(root.path().join(GENERATED).join("household_words.rs"))?;
    for list in ["NAMES", "TERMS", "SIZES", "MARKS"] {
        assert!(
            written.contains(&format!("pub const {list}: &[&str] = &[];")),
            "{written}"
        );
    }
    assert!(written.contains("pub const EXAMPLES: &[crate::HouseholdExample] = &[];"));
    Ok(())
}

#[test]
fn household_words_this_cannot_write_are_refused_naming_what() -> Outcome {
    refuses(
        set("index.json", "/household_words", json!(1)),
        "the index names no file for the household words",
    )?;
    refuses(
        |files| {
            files.insert("household-words.json".to_owned(), json!([]));
        },
        "the file of household words is not an object",
    )?;
    refuses(
        set("household-words.json", "/tones", json!([])),
        "the file of household words carries tones",
    )?;
    refuses(
        remove("household-words.json", "", "terms"),
        "the household words hold no list of terms",
    )?;
    refuses(
        set("household-words.json", "/sizes/0", json!("")),
        "entry 0 of the sizes in the household words is not a word",
    )?;
    refuses(
        set("household-words.json", "/marks", json!(["://", "://"])),
        "the household words list \"://\" twice in marks",
    )?;
    refuses(
        remove("household-words.json", "", "declared"),
        "the household words hold no names lemonfiber declares",
    )?;
    refuses(
        set("household-words.json", "/declared/form", json!(1)),
        "the declared name \"form\" in the household words is not said with what it names",
    )?;
    refuses(
        remove("household-words.json", "", "examples"),
        "the household words hold no list of examples",
    )?;
    refuses(
        set(
            "household-words.json",
            "/examples/0",
            json!("Sonarr is down"),
        ),
        "example 0 of the household words is not an object",
    )?;
    refuses(
        set("household-words.json", "/examples/0/why", json!("named")),
        "example 0 of the household words carries why",
    )?;
    refuses(
        set("household-words.json", "/examples/0/names", json!("")),
        "example 0 of the household words is not a text and what in it is named",
    )
}

#[test]
fn a_contract_taken_without_the_household_words_its_index_names_is_refused() -> Outcome {
    let from = laid_out(&tree())?;
    let extracted = from.path().join(CONTRACT);
    fs::remove_file(extracted.join("household-words.json"))?;
    let into = laid_out(&tree())?;
    let said = refused_with(
        into.path(),
        &[
            "take",
            &extracted.to_string_lossy(),
            fs::read_to_string(into.path().join(STAMP))?.trim(),
        ],
    );
    assert!(
        said.contains("missing files its index names") && said.contains("household-words.json"),
        "{said}"
    );
    Ok(())
}
