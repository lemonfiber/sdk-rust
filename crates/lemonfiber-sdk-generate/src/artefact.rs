use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

use serde_json::{Map, Value};

use crate::files;
use crate::names::is_type_name;
use crate::{Refused, CONTRACT, STAMP};

pub const SPOKEN: u64 = 1;

const INDEX_FIELDS: &[&str] = &[
    "api_version",
    "kinds",
    "actions",
    "bodies",
    "key_callable",
    "reads",
    "refusals",
];

const ANNOTATIONS: &[&str] = &[
    "$ref",
    "description",
    "title",
    "default",
    "examples",
    "$comment",
];

const ACTION_FIELDS: &[&str] = &["action", "arguments", "consent", "rehearsal"];

const READ_FIELDS: &[&str] = &["path", "parameters", "kinds", "file"];

const CALLABLE_FIELDS: &[&str] = &["action", "disturbs", "rehearsal", "idempotent", "moved"];

const REFUSAL_FIELDS: &[&str] = &["name", "status", "description"];

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String,
    pub arguments: Vec<(String, Value)>,
    pub consent: Vec<String>,
    pub rehearsal: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub route: String,
    pub schema: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read {
    pub path: String,
    pub parameters: Vec<(String, bool)>,
    pub kinds: Vec<String>,
    pub file: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCallable {
    pub action: String,
    pub disturbs: bool,
    pub rehearsal: bool,
    pub idempotent: bool,
    pub moved: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub code: String,
    pub name: String,
    pub status: u16,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Artefact {
    pub api_version: u64,
    pub revision: String,
    pub kinds: BTreeMap<String, Value>,
    pub defs: BTreeMap<String, Value>,
    pub actions: Vec<Action>,
    pub bodies: Vec<Body>,
    pub reads: Vec<Read>,
    pub key_callable: Vec<KeyCallable>,
    pub refusals: Vec<Refusal>,
}

pub fn revision(root: &Path) -> Result<String, Refused> {
    let said = files::text(&root.join(STAMP), STAMP)?;
    let said = said.trim();
    if is_revision(said) {
        Ok(said.to_owned())
    } else {
        Err(Refused::new(format!(
            "{STAMP} names {said:?}, which is not a release tag or a full commit hash"
        )))
    }
}

#[must_use]
pub fn is_revision(said: &str) -> bool {
    let hash = said.len() == 40
        && said
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c));
    let tag = said.strip_prefix('v').is_some_and(|version| {
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() == 3
            && parts
                .iter()
                .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
    });
    hash || tag
}

struct Reader<'a> {
    directory: &'a Path,
    problems: Vec<String>,
    wanted: VecDeque<String>,
}

impl Reader<'_> {
    fn json(&self, file: &str) -> Result<Value, Refused> {
        files::json(&self.directory.join(file), &format!("{CONTRACT}/{file}"))
    }

    fn schema(&mut self, file: &str) -> Result<Value, Refused> {
        let mut schema = self.json(file)?;
        self.resolve(&mut schema, file);
        Ok(schema)
    }

    fn resolve(&mut self, node: &mut Value, file: &str) {
        match node {
            Value::Array(items) => {
                for item in items {
                    self.resolve(item, file);
                }
            }
            Value::Object(fields) => {
                if let Some(Value::String(reference)) = fields.get("$ref") {
                    let beside: Vec<&str> = fields
                        .keys()
                        .map(String::as_str)
                        .filter(|key| !ANNOTATIONS.contains(key))
                        .collect();
                    if !beside.is_empty() {
                        self.problems.push(format!(
                            "{file}: `{reference}` has {} beside it, which one reader applies and another discards",
                            beside.join(", ")
                        ));
                    }
                    match self.reached(file, reference) {
                        Some(name) => {
                            fields.insert("$ref".to_owned(), Value::String(name.clone()));
                            self.wanted.push_back(name);
                        }
                        None => self.problems.push(format!(
                            "{file}: `{reference}` reaches no definition in the vendored copy"
                        )),
                    }
                }
                for (key, value) in fields.iter_mut() {
                    match key.as_str() {
                        "$ref" | "default" | "const" | "enum" | "examples" => {}
                        "properties" | "$defs" | "patternProperties" => {
                            for property in value
                                .as_object_mut()
                                .into_iter()
                                .flat_map(|named| named.values_mut())
                            {
                                self.resolve(property, file);
                            }
                        }
                        _ => self.resolve(value, file),
                    }
                }
            }
            _ => {}
        }
    }

    fn reached(&self, file: &str, reference: &str) -> Option<String> {
        let mut path: Vec<&str> = file.split('/').collect();
        path.pop();
        for segment in reference.split('/') {
            match segment {
                ".." => {
                    path.pop()?;
                }
                "." => {}
                other => path.push(other),
            }
        }
        let [folder, named] = path.as_slice() else {
            return None;
        };
        let name = named.strip_suffix(".json")?;
        (*folder == "defs" && is_type_name(name) && self.directory.join(path.join("/")).is_file())
            .then(|| name.to_owned())
    }
}

fn object<'a>(
    node: &'a Value,
    what: &str,
    problems: &mut Vec<String>,
) -> Option<&'a Map<String, Value>> {
    let fields = node.as_object();
    if fields.is_none() {
        problems.push(format!("{what} is not an object"));
    }
    fields
}

fn unread(fields: &Map<String, Value>, known: &[&str], what: &str, problems: &mut Vec<String>) {
    let extra: Vec<&str> = fields
        .keys()
        .map(String::as_str)
        .filter(|key| !known.contains(key))
        .collect();
    if !extra.is_empty() {
        problems.push(format!(
            "{what} carries {}, which this generator does not read",
            extra.join(", ")
        ));
    }
}

fn file_of<'a>(node: Option<&'a Value>, what: &str, problems: &mut Vec<String>) -> Option<&'a str> {
    let named = node.and_then(Value::as_str);
    if named.is_none() {
        problems.push(format!("the index names no file for {what}"));
    }
    named
}

fn is_word(name: &str) -> bool {
    !name.is_empty()
        && name.split('-').all(|part| {
            part.starts_with(|c: char| c.is_ascii_lowercase())
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

fn is_code(code: &str, family: &str) -> bool {
    code.strip_prefix(family)
        .and_then(|rest| rest.strip_prefix('-'))
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()))
}

fn is_read_path(path: &str) -> bool {
    path.strip_prefix("/api/").is_some_and(|rest| {
        rest.split('/').all(|segment| {
            is_word(segment)
                || segment
                    .strip_prefix('{')
                    .and_then(|inner| inner.strip_suffix('}'))
                    .is_some_and(|inner| {
                        !inner.is_empty()
                            && inner.chars().all(|c| c.is_ascii_lowercase() || c == '_')
                    })
        })
    })
}

impl Artefact {
    pub fn read(root: &Path) -> Result<Self, Refused> {
        let revision = revision(root)?;
        let directory = root.join(CONTRACT);
        let mut reader = Reader {
            directory: &directory,
            problems: Vec::new(),
            wanted: VecDeque::new(),
        };
        let index = reader.json("index.json")?;
        let mut problems = Vec::new();
        let Some(fields) = object(&index, "the index", &mut problems) else {
            return Err(Refused::listing(
                "the vendored contract cannot be read",
                &problems,
            ));
        };
        let api_version = fields.get("api_version").and_then(Value::as_u64);
        if api_version != Some(SPOKEN) {
            let said = fields
                .get("api_version")
                .map_or_else(|| "none".to_owned(), Value::to_string);
            return Err(Refused::new(format!(
                "the vendored contract is api_version {said}, and this package implements {SPOKEN}. \
                 Take a revision that speaks {SPOKEN}, or implement the newer version first."
            )));
        }
        unread(fields, INDEX_FIELDS, "the index", &mut problems);

        let kinds = read_kinds(&mut reader, fields, &mut problems)?;
        let actions = read_actions(&mut reader, fields, &mut problems)?;
        let bodies = read_bodies(&mut reader, fields, &mut problems)?;
        let refusals = read_refusals(&reader, fields, &mut problems)?;
        let reads = match file_of(fields.get("reads"), "the reads", &mut problems) {
            Some(file) => read_reads(&reader.json(file)?, &kinds, &mut problems),
            None => Vec::new(),
        };
        let key_callable = match file_of(
            fields.get("key_callable"),
            "the actions a key may call",
            &mut problems,
        ) {
            Some(file) => {
                read_key_callable(&reader.json(file)?, &actions, &refusals, &mut problems)
            }
            None => Vec::new(),
        };

        let mut defs = BTreeMap::new();
        while let Some(name) = reader.wanted.pop_front() {
            if let std::collections::btree_map::Entry::Vacant(vacant) = defs.entry(name) {
                let schema = reader.schema(&format!("defs/{}.json", vacant.key()))?;
                vacant.insert(schema);
            }
        }
        problems.append(&mut reader.problems);
        if problems.is_empty() {
            Ok(Self {
                api_version: SPOKEN,
                revision,
                kinds,
                defs,
                actions,
                bodies,
                reads,
                key_callable,
                refusals,
            })
        } else {
            Err(Refused::listing(
                "the vendored contract holds what this generator cannot write, and nothing was written",
                &problems,
            ))
        }
    }
}

fn read_kinds(
    reader: &mut Reader<'_>,
    index: &Map<String, Value>,
    problems: &mut Vec<String>,
) -> Result<BTreeMap<String, Value>, Refused> {
    let mut kinds = BTreeMap::new();
    let Some(listed) = index
        .get("kinds")
        .and_then(Value::as_object)
        .filter(|listed| !listed.is_empty())
    else {
        problems.push("the index lists no kinds".to_owned());
        return Ok(kinds);
    };
    for (kind, file) in listed {
        let Some(file) = file.as_str() else {
            problems.push(format!("the kind {kind} names no file"));
            continue;
        };
        if !is_word(kind) {
            problems.push(format!(
                "the kind {kind:?} is not lower-case words joined by hyphens"
            ));
        }
        let envelope = reader.schema(file)?;
        match envelope.pointer("/properties/data") {
            Some(data) => {
                kinds.insert(kind.clone(), data.clone());
            }
            None => problems.push(format!("{file} is not an envelope: it describes no data")),
        }
    }
    Ok(kinds)
}

fn read_actions(
    reader: &mut Reader<'_>,
    index: &Map<String, Value>,
    problems: &mut Vec<String>,
) -> Result<Vec<Action>, Refused> {
    let mut actions = Vec::new();
    for (name, file) in index
        .get("actions")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        let Some(file) = file.as_str() else {
            problems.push(format!("the action {name} names no file"));
            continue;
        };
        let read = reader.schema(file)?;
        let Some(fields) = object(&read, file, problems) else {
            continue;
        };
        unread(fields, ACTION_FIELDS, file, problems);
        if !is_word(name) || fields.get("action").and_then(Value::as_str) != Some(name.as_str()) {
            problems.push(format!("{file} does not describe the action {name}"));
        }
        let mut arguments = Vec::new();
        for argument in fields
            .get("arguments")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            match (
                argument.get("name").and_then(Value::as_str),
                argument.get("type"),
            ) {
                (Some(named), Some(schema)) if !named.is_empty() => {
                    arguments.push((named.to_owned(), schema.clone()));
                }
                _ => problems.push(format!("{file}: an argument names no name and type")),
            }
        }
        let consent = strings(fields.get("consent"), &format!("{file}: consent"), problems);
        let Some(rehearsal) = fields.get("rehearsal").and_then(Value::as_bool) else {
            problems.push(format!("{file}: rehearsal is not true or false"));
            continue;
        };
        actions.push(Action {
            name: name.clone(),
            arguments,
            consent,
            rehearsal,
        });
    }
    Ok(actions)
}

fn strings(node: Option<&Value>, what: &str, problems: &mut Vec<String>) -> Vec<String> {
    let listed: Option<Vec<String>> = node.and_then(Value::as_array).and_then(|items| {
        items
            .iter()
            .map(|item| item.as_str().map(str::to_owned))
            .collect()
    });
    listed.unwrap_or_else(|| {
        problems.push(format!("{what} is not a list of words"));
        Vec::new()
    })
}

fn read_bodies(
    reader: &mut Reader<'_>,
    index: &Map<String, Value>,
    problems: &mut Vec<String>,
) -> Result<Vec<Body>, Refused> {
    let mut bodies = Vec::new();
    for (route, file) in index
        .get("bodies")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        let Some(file) = file.as_str() else {
            problems.push(format!("the body of {route} names no file"));
            continue;
        };
        if route
            .strip_prefix("/api/")
            .is_some_and(|rest| rest.split('/').all(is_word))
        {
            bodies.push(Body {
                route: route.clone(),
                schema: reader.schema(file)?,
            });
        } else {
            problems.push(format!(
                "the body of {route:?} is not taken at a route of words"
            ));
        }
    }
    Ok(bodies)
}

fn read_refusals(
    reader: &Reader<'_>,
    index: &Map<String, Value>,
    problems: &mut Vec<String>,
) -> Result<Vec<Refusal>, Refused> {
    let mut refusals = Vec::new();
    let mut names = BTreeSet::new();
    for (family, file) in index
        .get("refusals")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        let Some(file) = file.as_str() else {
            problems.push(format!("the refusals of {family} name no file"));
            continue;
        };
        let listed = reader.json(file)?;
        for (code, entry) in listed.as_object().into_iter().flatten() {
            let what = format!("{file}: {code}");
            let Some(fields) = object(entry, &what, problems) else {
                continue;
            };
            unread(fields, REFUSAL_FIELDS, &what, problems);
            let name = fields
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let status = fields
                .get("status")
                .and_then(Value::as_u64)
                .and_then(|status| u16::try_from(status).ok())
                .filter(|status| (400..600).contains(status));
            let description = fields
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let screaming = !name.is_empty()
                && name.split('_').all(|word| {
                    !word.is_empty()
                        && word
                            .chars()
                            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                });
            if !is_code(code, family) || !screaming || description.trim().is_empty() {
                problems.push(format!(
                    "{what} is not a code of {family} with a name and a description"
                ));
                continue;
            }
            let Some(status) = status else {
                problems.push(format!("{what}: its status is not a refusal's"));
                continue;
            };
            if !names.insert(name.to_owned()) {
                problems.push(format!("{what}: the name {name} is another code's too"));
            }
            refusals.push(Refusal {
                code: code.clone(),
                name: name.to_owned(),
                status,
                description: description.to_owned(),
            });
        }
    }
    refusals.sort_by_key(|refusal| order_of(&refusal.code));
    Ok(refusals)
}

fn order_of(code: &str) -> (String, u64) {
    let (family, number) = code.rsplit_once('-').unwrap_or((code, "0"));
    (family.to_owned(), number.parse().unwrap_or_default())
}

fn read_reads(
    listed: &Value,
    kinds: &BTreeMap<String, Value>,
    problems: &mut Vec<String>,
) -> Vec<Read> {
    let mut reads: Vec<Read> = Vec::new();
    let Some(entries) = listed.as_array() else {
        problems.push("the reads are not a list".to_owned());
        return reads;
    };
    for (at, entry) in entries.iter().enumerate() {
        let what = format!("read {at}");
        let Some(fields) = object(entry, &what, problems) else {
            continue;
        };
        unread(fields, READ_FIELDS, &what, problems);
        let path = fields
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !is_read_path(path) {
            problems.push(format!("{what}: {path:?} is not a read's path"));
            continue;
        }
        let mut parameters = Vec::new();
        for parameter in fields
            .get("parameters")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let name = parameter.get("name").and_then(Value::as_str);
            let repeatable = parameter.get("repeatable").and_then(Value::as_bool);
            match (name, repeatable) {
                (Some(name), Some(repeatable))
                    if !name.is_empty()
                        && name.chars().all(|c| c.is_ascii_lowercase() || c == '_')
                        && !path.contains(&format!("{{{name}}}")) =>
                {
                    parameters.push((name.to_owned(), repeatable));
                }
                _ => problems.push(format!(
                    "{what}: a parameter of {path} is not a name and whether it repeats"
                )),
            }
        }
        let answers = strings(fields.get("kinds"), &format!("{what}: kinds"), problems);
        for kind in &answers {
            if !kinds.contains_key(kind) {
                problems.push(format!(
                    "{what}: {path} answers with {kind}, which the contract describes no kind as"
                ));
            }
        }
        let Some(file) = fields.get("file").and_then(Value::as_bool) else {
            problems.push(format!("{what}: file is not true or false"));
            continue;
        };
        if file == answers.is_empty() {
            if reads.iter().any(|read| read.path == path) {
                problems.push(format!("{what}: {path} is listed twice"));
            }
            reads.push(Read {
                path: path.to_owned(),
                parameters,
                kinds: answers,
                file,
            });
        } else {
            problems.push(format!(
                "{what}: {path} answers with a file and kinds, or with neither"
            ));
        }
    }
    reads
}

fn read_key_callable(
    listed: &Value,
    actions: &[Action],
    refusals: &[Refusal],
    problems: &mut Vec<String>,
) -> Vec<KeyCallable> {
    let mut callable = Vec::new();
    for (at, entry) in listed.as_array().into_iter().flatten().enumerate() {
        let what = format!("key-callable entry {at}");
        let Some(fields) = object(entry, &what, problems) else {
            continue;
        };
        unread(fields, CALLABLE_FIELDS, &what, problems);
        let flag = |name: &str| fields.get(name).and_then(Value::as_bool);
        let action = fields
            .get("action")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let moved = fields
            .get("moved")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let (Some(disturbs), Some(rehearsal), Some(idempotent)) =
            (flag("disturbs"), flag("rehearsal"), flag("idempotent"))
        else {
            problems.push(format!(
                "{what}: disturbs, rehearsal and idempotent are not each true or false"
            ));
            continue;
        };
        if !actions.iter().any(|known| known.name == action) {
            problems.push(format!(
                "{what}: {action:?} is not an action the contract lists"
            ));
        }
        if let Some(code) = &moved {
            if !refusals.iter().any(|refusal| &refusal.code == code) {
                problems.push(format!(
                    "{what}: moved {code} is not a refusal the contract lists"
                ));
            }
        }
        callable.push(KeyCallable {
            action: action.to_owned(),
            disturbs,
            rehearsal,
            idempotent,
            moved,
        });
    }
    callable
}
