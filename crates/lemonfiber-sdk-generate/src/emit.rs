use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Output, Stdio};

use crate::artefact::{Artefact, Read};
use crate::names::{field, literal, pascal, screaming};
use crate::schema::{Body, Field, Holds, Shape, Shaped, Ty, Variant, UNRECOGNISED};
use crate::Refused;

const JSON: &str = "serde_json::Value";

const DERIVES: &str = "#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]";

const OPEN: &str = "#[non_exhaustive]\n";

const MOST_LINES: usize = 900;

const CUT: &str = "// cut\n";

pub type Groups = BTreeMap<String, Vec<String>>;

struct Modules {
    shapes: Vec<(String, String)>,
    reads: Vec<(String, String)>,
    actions: Vec<(String, String)>,
    refusals: Vec<(String, String)>,
}

pub fn files(artefact: &Artefact, shaped: &Shaped) -> Result<Vec<(String, String)>, Refused> {
    let mut groups = Groups::new();
    for shape in shaped.shapes.values() {
        groups
            .entry(letter(&shape.name))
            .or_default()
            .push(shape_of(shape, !shaped.asked.contains(&shape.name)));
    }
    let modules = Modules {
        shapes: paged("shapes", &groups)?,
        reads: paged("reads", &reads(artefact, shaped)?)?,
        actions: paged("actions", &actions(shaped))?,
        refusals: paged("refusals", &refusals(artefact))?,
    };
    let lists = [
        ("mod", root(artefact, shaped, &modules)),
        ("action_names", action_names(shaped)),
        ("refusal_list", refusal_list(artefact)),
        ("key_callable", key_callable(artefact)),
    ];
    let mut written: Vec<(String, String)> = lists
        .into_iter()
        .map(|(name, text)| formatted(&text).map(|text| (name.to_owned(), text)))
        .collect::<Result<_, _>>()?;
    written.extend(modules.shapes);
    written.extend(modules.reads);
    written.extend(modules.actions);
    written.extend(modules.refusals);
    Ok(written
        .into_iter()
        .map(|(name, text)| (format!("{name}.rs"), text))
        .collect())
}

fn letter(name: &str) -> String {
    name.chars()
        .next()
        .unwrap_or('x')
        .to_ascii_lowercase()
        .to_string()
}

pub fn paged(stem: &str, groups: &Groups) -> Result<Vec<(String, String)>, Refused> {
    let mut modules = Vec::new();
    for (key, items) in groups {
        let text = formatted(&items.join(CUT))?;
        let pages = pages_of(&text);
        let several = pages.len() > 1;
        for (at, page) in pages.iter().enumerate() {
            let name = if several {
                format!("{stem}_{key}_{}", at + 1)
            } else {
                format!("{stem}_{key}")
            };
            modules.push((name, format!("{}\n", page.join("\n\n"))));
        }
    }
    Ok(modules)
}

fn pages_of(text: &str) -> Vec<Vec<&str>> {
    let mut pages = Vec::new();
    let mut page = Vec::new();
    let mut lines = 0;
    for item in text.split(CUT).map(str::trim) {
        let counted = item.lines().count() + 1;
        if lines + counted > MOST_LINES && !page.is_empty() {
            pages.push(std::mem::take(&mut page));
            lines = 0;
        }
        page.push(item);
        lines += counted;
    }
    pages.push(page);
    pages
}

pub fn formatted(text: &str) -> Result<String, Refused> {
    match rustfmt(text) {
        Some(done) if done.status.success() => Ok(String::from_utf8_lossy(&done.stdout).into_owned()),
        _ => Err(Refused::new(
            "rustfmt could not format the generated module: it is not installed, or what was generated is not Rust",
        )),
    }
}

fn rustfmt(text: &str) -> Option<Output> {
    let mut running = Command::new("rustfmt")
        .args(["--edition", "2021", "--emit", "stdout", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    running.stdin.take()?.write_all(text.as_bytes()).ok()?;
    running.wait_with_output().ok()
}

fn doc(said: Option<&str>, otherwise: &str) -> String {
    let text = said.unwrap_or(otherwise);
    text.lines()
        .map(|line| {
            let line = line.trim_end();
            if line.is_empty() {
                "///\n".to_owned()
            } else {
                format!("/// {line}\n")
            }
        })
        .collect()
}

fn rust(ty: &Ty) -> String {
    match ty {
        Ty::Str => "String".to_owned(),
        Ty::Bool => "bool".to_owned(),
        Ty::Int(held) => (*held).to_owned(),
        Ty::Float => "f64".to_owned(),
        Ty::Unit => "()".to_owned(),
        Ty::Json => JSON.to_owned(),
        Ty::List(item) => format!("Vec<{}>", rust(item)),
        Ty::Map(value) => format!("std::collections::BTreeMap<String, {}>", rust(value)),
        Ty::Opt(inner) => format!("Option<{}>", rust(inner)),
        Ty::Boxed(inner) => format!("Box<{}>", rust(inner)),
        Ty::Named(name) => format!("super::{name}"),
        Ty::Tuple(places) => format!(
            "({})",
            places.iter().map(rust).collect::<Vec<_>>().join(", ")
        ),
    }
}

fn field_of(field: &Field, public: bool) -> String {
    let mut text = doc(
        field.doc.as_deref(),
        &format!("The `{}` field.", field.json),
    );
    if field.ident.trim_start_matches("r#") != field.json {
        let _ = writeln!(text, "#[serde(rename = {})]", literal(&field.json));
    }
    if matches!(field.ty, Ty::Opt(_)) {
        text.push_str("#[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
    }
    let _ = writeln!(
        text,
        "{}{}: {},",
        if public { "pub " } else { "" },
        field.ident,
        rust(&field.ty)
    );
    text
}

fn variant_of(variant: &Variant) -> String {
    let mut text = doc(
        variant.doc.as_deref(),
        &format!("The `{}` variant.", variant.name),
    );
    if let Some(wire) = &variant.wire {
        let _ = writeln!(text, "#[serde(rename = {})]", literal(wire));
    }
    match &variant.holds {
        Holds::Nothing => {
            let _ = writeln!(text, "{},", variant.name);
        }
        Holds::One(ty) => {
            let _ = writeln!(text, "{}({}),", variant.name, rust(ty));
        }
        Holds::Fields(fields) => {
            let _ = writeln!(text, "{} {{", variant.name);
            for field in fields {
                text.push_str(&field_of(field, false));
            }
            text.push_str("},\n");
        }
    }
    text
}

fn shape_of(shape: &Shape, open: bool) -> String {
    let mut text = doc(
        shape.doc.as_deref(),
        &format!("The `{}` shape.", shape.name),
    );
    match &shape.body {
        Body::Struct(fields) => {
            let _ = writeln!(text, "{DERIVES}\npub struct {} {{", shape.name);
            for field in fields {
                text.push_str(&field_of(field, true));
            }
            text.push_str("}\n\n");
        }
        Body::External(variants) => {
            let plain = variants
                .iter()
                .all(|variant| variant.holds == Holds::Nothing);
            let held = if plain { "String" } else { JSON };
            let last = open.then(|| unrecognised("#[serde(untagged)]\n", held));
            enum_of(&mut text, &shape.name, "", last, variants);
        }
        Body::Tagged { tag, variants } => {
            let attribute = format!("#[serde(tag = {})]\n", literal(tag));
            let last = open.then(|| unrecognised("#[serde(untagged)]\n", JSON));
            enum_of(&mut text, &shape.name, &attribute, last, variants);
        }
        Body::Untagged(variants) => {
            let last = open.then(|| unrecognised("", JSON));
            enum_of(
                &mut text,
                &shape.name,
                "#[serde(untagged)]\n",
                last,
                variants,
            );
        }
        Body::Alias(ty) => {
            let _ = writeln!(text, "pub type {} = {};\n", shape.name, rust(ty));
        }
    }
    text
}

fn unrecognised(attribute: &str, held: &str) -> String {
    format!("/// A value this client was not generated with, as it arrived.\n{attribute}{UNRECOGNISED}({held}),\n")
}

fn enum_of(
    text: &mut String,
    name: &str,
    attribute: &str,
    last: Option<String>,
    variants: &[Variant],
) {
    let open = if last.is_some() { OPEN } else { "" };
    let _ = write!(text, "{DERIVES}\n{attribute}{open}pub enum {name} {{\n");
    for variant in variants {
        text.push_str(&variant_of(variant));
    }
    text.push_str(&last.unwrap_or_default());
    text.push_str("}\n\n");
}

fn each(modules: &[(String, String)], before: &str, after: &str) -> String {
    let mut text = String::new();
    for (name, _) in modules {
        let _ = writeln!(text, "{before}{name}{after}");
    }
    text
}

fn module(said: &str, name: &str, list: &str, pages: &[(String, String)]) -> String {
    let exported = each(pages, "pub use super::", "::*;");
    format!("/// {said}\npub mod {name} {{\n{list}{exported}}}\n\n")
}

fn root(artefact: &Artefact, shaped: &Shaped, modules: &Modules) -> String {
    let mut text = "//! The lemonfiber web API's shapes, reads, actions and refusals, generated from the\n\
                    //! contract artefact taken from the lemonfiber revision [`REVISION`] names.\n\
                    //! `just generate` writes this module, and CI refuses it where it differs from\n\
                    //! what that writes.\n\n\
                    mod action_names;\npub mod key_callable;\nmod refusal_list;\n"
        .to_owned();
    text.push_str(&each(&modules.reads, "mod ", ";"));
    text.push_str(&each(&modules.actions, "mod ", ";"));
    text.push_str(&each(&modules.refusals, "mod ", ";"));
    for (name, _) in &modules.shapes {
        let _ = writeln!(text, "mod {name};\npub use {name}::*;");
    }
    text.push('\n');
    text.push_str(&module(
        "Every read the web API serves: a struct for each, holding what it takes.",
        "reads",
        "",
        &modules.reads,
    ));
    text.push_str(&module(
        "Every action the web API takes: a struct for each, holding its arguments.",
        "actions",
        "pub use super::action_names::NAMES;\n",
        &modules.actions,
    ));
    text.push_str(&module(
        "Every code a refusal may carry, with the status it is answered with.",
        "refusals",
        "pub use super::refusal_list::ALL;\n",
        &modules.refusals,
    ));
    let _ = write!(
        text,
        "/// The wire version this contract describes.\npub const API_VERSION: u32 = {};\n\n\
         /// The lemonfiber revision this contract was taken from.\npub const REVISION: &str = {};\n\n",
        artefact.api_version,
        literal(&artefact.revision)
    );
    text.push_str("/// Every kind the contract describes.\npub const KINDS: &[&str] = &[\n");
    for kind in shaped.kinds.keys() {
        let _ = writeln!(text, "{},", literal(kind));
    }
    text.push_str("];\n\n");
    text.push_str(
        "/// The data of an envelope, typed by its kind.\n#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n\
         #[serde(tag = \"kind\", content = \"data\")]\n#[non_exhaustive]\npub enum Payload {\n",
    );
    for (kind, ty) in &shaped.kinds {
        let _ = writeln!(
            text,
            "/// The `{kind}` kind.\n#[serde(rename = {})]\n{}(Box<{}>),",
            literal(kind),
            pascal(kind),
            rust_here(ty)
        );
    }
    text.push_str("}\n");
    text
}

fn rust_here(ty: &Ty) -> String {
    rust(ty).replace("super::", "")
}

fn read_name(read: &Read) -> String {
    pascal(read.path.trim_start_matches("/api/"))
}

fn segments_of(read: &Read) -> Vec<&str> {
    read.path
        .split('/')
        .filter_map(|segment| {
            segment
                .strip_prefix('{')
                .and_then(|inner| inner.strip_suffix('}'))
        })
        .collect()
}

fn listed(words: &[&str]) -> String {
    words
        .iter()
        .map(|word| literal(word))
        .collect::<Vec<_>>()
        .join(", ")
}

fn read_struct(read: &Read, name: &str, segments: &[&str]) -> String {
    let what = if read.file {
        "answered with a file".to_owned()
    } else {
        let kinds: Vec<String> = read.kinds.iter().map(|kind| format!("`{kind}`")).collect();
        format!("answered with {}", kinds.join(" or "))
    };
    let mut text = format!(
        "/// `GET {}`, {what}.\n#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]\npub struct {name} {{\n",
        read.path
    );
    for segment in segments {
        let _ = writeln!(
            text,
            "/// The `{segment}` segment of the path.\npub {}: String,",
            field(segment)
        );
    }
    for (parameter, repeatable) in &read.parameters {
        let ident = field(parameter);
        let rename = if ident.trim_start_matches("r#") == parameter {
            String::new()
        } else {
            format!("#[serde(rename = {})]\n", literal(parameter))
        };
        let (said, skipped, held) = if *repeatable {
            (", sent once for each value", "Vec::is_empty", "Vec<String>")
        } else {
            ("", "Option::is_none", "Option<String>")
        };
        let _ = writeln!(
            text,
            "/// The `{parameter}` parameter{said}.\n{rename}#[serde(skip_serializing_if = \"{skipped}\")]\npub {ident}: {held},"
        );
    }
    text.push_str("}\n\n");
    text
}

fn read_answer(
    read: &Read,
    name: &str,
    shaped: &Shaped,
    answered: &mut BTreeSet<String>,
) -> (String, String) {
    if let [kind] = read.kinds.as_slice() {
        if let Some(Ty::Named(data)) = shaped.kinds.get(kind) {
            let text = if answered.insert(data.clone()) {
                format!(
                    "impl crate::Answer for super::{data} {{\nconst TAGGED: bool = false;\n}}\n\n"
                )
            } else {
                String::new()
            };
            return (format!("super::{data}"), text);
        }
    }
    let answer = format!("{name}Answer");
    let mut text = format!(
        "/// What `GET {}` answers with, by its kind.\n#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n#[serde(tag = \"kind\", content = \"data\")]\n#[non_exhaustive]\npub enum {answer} {{\n",
        read.path
    );
    for kind in &read.kinds {
        let held = shaped.kinds.get(kind).map(rust).unwrap_or_default();
        let _ = writeln!(
            text,
            "/// The `{kind}` kind.\n#[serde(rename = {})]\n{}(Box<{held}>),",
            literal(kind),
            pascal(kind)
        );
    }
    let _ = writeln!(
        text,
        "}}\n\nimpl crate::Answer for {answer} {{\nconst TAGGED: bool = true;\n}}\n"
    );
    (answer, text)
}

fn reads(artefact: &Artefact, shaped: &Shaped) -> Result<Groups, Refused> {
    let mut groups = Groups::new();
    let mut names = BTreeSet::new();
    let mut answered = BTreeSet::new();
    let mut problems = Vec::new();
    for read in &artefact.reads {
        let name = read_name(read);
        if !names.insert(name.clone()) || !names.insert(format!("{name}Answer")) {
            problems.push(format!(
                "{}: written as {name}, which another read takes",
                read.path
            ));
        }
        let segments = segments_of(read);
        let mut text = read_struct(read, &name, &segments);
        let path = literal(&read.path);
        let segments = listed(&segments);
        if read.file {
            let _ = writeln!(
                text,
                "impl crate::FileRead for {name} {{\nconst PATH: &str = {path};\nconst SEGMENTS: &[&str] = &[{segments}];\n}}\n"
            );
            groups.entry(letter(&name)).or_default().push(text);
            continue;
        }
        let (answer, written) = read_answer(read, &name, shaped, &mut answered);
        text.push_str(&written);
        let kinds: Vec<&str> = read.kinds.iter().map(String::as_str).collect();
        let _ = writeln!(
            text,
            "impl crate::Read for {name} {{\ntype Answer = {answer};\nconst PATH: &str = {path};\nconst SEGMENTS: &[&str] = &[{segments}];\nconst KINDS: &[&str] = &[{}];\n}}\n",
            listed(&kinds)
        );
        groups.entry(letter(&name)).or_default().push(text);
    }
    if problems.is_empty() {
        Ok(groups)
    } else {
        Err(Refused::listing(
            "the vendored contract lists reads this generator cannot tell apart",
            &problems,
        ))
    }
}

fn actions(shaped: &Shaped) -> Groups {
    let mut groups = Groups::new();
    for action in &shaped.actions {
        let derives = if action.defaulted {
            "Debug, Clone, Default, PartialEq, serde::Serialize"
        } else {
            "Debug, Clone, PartialEq, serde::Serialize"
        };
        let mut text = format!(
            "/// `POST /api/actions/{}`.\n#[derive({derives})]\npub struct {} {{\n",
            action.name, action.ident
        );
        for field in &action.fields {
            text.push_str(&field_of(field, true));
        }
        let consent: Vec<&str> = action.consent.iter().map(String::as_str).collect();
        let _ = writeln!(
            text,
            "}}\n\nimpl crate::Action for {} {{\nconst NAME: &str = {};\nconst REHEARSAL: bool = {};\nconst CONSENT: &[&str] = &[{}];\n}}\n",
            action.ident,
            literal(&action.name),
            action.rehearsal,
            listed(&consent)
        );
        groups.entry(letter(&action.ident)).or_default().push(text);
    }
    groups
}

fn action_names(shaped: &Shaped) -> String {
    let mut text =
        "/// The name of every action the web API takes.\npub const NAMES: &[&str] = &[\n"
            .to_owned();
    for action in &shaped.actions {
        let _ = writeln!(text, "{},", literal(&action.name));
    }
    text.push_str("];\n");
    text
}

fn refusals(artefact: &Artefact) -> Groups {
    let mut groups = Groups::new();
    for refusal in &artefact.refusals {
        groups.entry(refusal.family.clone()).or_default().push(format!(
            "/// `{}`: {}\npub const {}: crate::RefusalCode = crate::RefusalCode {{\ncode: {},\nname: {},\nstatus: {},\ndescription: {},\n}};\n\n",
            refusal.code,
            refusal.description,
            screaming(&refusal.name),
            literal(&refusal.code),
            literal(&refusal.name),
            refusal.status,
            literal(&refusal.description)
        ));
    }
    groups
}

fn refusal_list(artefact: &Artefact) -> String {
    let mut text =
        "/// Every code a refusal may carry.\npub const ALL: &[crate::RefusalCode] = &[\n"
            .to_owned();
    for refusal in &artefact.refusals {
        let _ = writeln!(text, "super::refusals::{},", screaming(&refusal.name));
    }
    text.push_str("];\n");
    text
}

fn key_callable(artefact: &Artefact) -> String {
    let mut text = "//! The actions a key may call, and what the contract says of each.\n\n\
                    /// Every action a key may call.\npub const ALL: &[crate::KeyCallable] = &[\n"
        .to_owned();
    for callable in &artefact.key_callable {
        let moved = callable
            .moved
            .as_ref()
            .and_then(|code| {
                artefact
                    .refusals
                    .iter()
                    .find(|refusal| &refusal.code == code)
            })
            .map_or_else(
                || "None".to_owned(),
                |refusal| format!("Some(&super::refusals::{})", screaming(&refusal.name)),
            );
        let _ = writeln!(
            text,
            "crate::KeyCallable {{\naction: {},\ndisturbs: {},\nrehearsal: {},\nidempotent: {},\nmoved: {moved},\n}},",
            literal(&callable.action),
            callable.disturbs,
            callable.rehearsal,
            callable.idempotent
        );
    }
    text.push_str("];\n");
    text
}
