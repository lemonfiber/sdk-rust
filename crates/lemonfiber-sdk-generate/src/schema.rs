use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::artefact::Artefact;
use crate::names::{action_type, answer_type, field, pascal, read_type, type_name};
use crate::Refused;

const WRITTEN: &[&str] = &["Payload"];

const PRELUDE: &[&str] = &["Box", "Option", "String", "Vec"];

pub const UNRECOGNISED: &str = "Unrecognised";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Str,
    Bool,
    Int(&'static str),
    Float,
    Unit,
    Json,
    List(Box<Ty>),
    Map(Box<Ty>),
    Opt(Box<Ty>),
    Boxed(Box<Ty>),
    Named(String),
    Tuple(Vec<Ty>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub json: String,
    pub ident: String,
    pub doc: Option<String>,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub wire: Option<String>,
    pub doc: Option<String>,
    pub holds: Holds,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Holds {
    Nothing,
    One(Ty),
    Fields(Vec<Field>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Struct(Vec<Field>),
    External(Vec<Variant>),
    Tagged { tag: String, variants: Vec<Variant> },
    Untagged(Vec<Variant>),
    Alias(Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub name: String,
    pub doc: Option<String>,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapedAction {
    pub name: String,
    pub ident: String,
    pub fields: Vec<Field>,
    pub defaulted: bool,
    pub consent: Vec<String>,
    pub rehearsal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shaped {
    pub shapes: BTreeMap<String, Shape>,
    pub kinds: BTreeMap<String, Ty>,
    pub actions: Vec<ShapedAction>,
    pub bodies: Vec<(String, String)>,
    pub asked: BTreeSet<String>,
}

struct Shaper {
    defs: BTreeSet<String>,
    shapes: BTreeMap<String, Shape>,
    problems: Vec<String>,
}

fn doc_of(schema: &Value) -> Option<String> {
    schema
        .get("description")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|said| !said.is_empty())
        .map(str::to_owned)
}

fn alternatives(schema: &Value) -> Option<&Vec<Value>> {
    schema
        .get("oneOf")
        .or_else(|| schema.get("anyOf"))
        .and_then(Value::as_array)
}

fn is_null(schema: &Value) -> bool {
    schema.get("type").and_then(Value::as_str) == Some("null")
}

fn constant(schema: &Value) -> Option<&str> {
    schema.get("const").and_then(Value::as_str).or_else(|| {
        schema
            .get("enum")
            .and_then(Value::as_array)
            .filter(|values| values.len() == 1)
            .and_then(|values| values.first())
            .and_then(Value::as_str)
    })
}

fn properties(schema: &Value) -> Option<&Map<String, Value>> {
    schema.get("properties").and_then(Value::as_object)
}

fn required(schema: &Value) -> BTreeSet<&str> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

fn integer(format: Option<&str>) -> Option<&'static str> {
    match format {
        None | Some("int64") => Some("i64"),
        Some("int32") => Some("i32"),
        Some("uint8") => Some("u8"),
        Some("uint16") => Some("u16"),
        Some("uint32") => Some("u32"),
        Some("uint64" | "uint") => Some("u64"),
        Some(_) => None,
    }
}

impl Shaper {
    fn problem(&mut self, at: &str, what: &str) -> Ty {
        self.problems.push(format!("{at}: {what}"));
        Ty::Json
    }

    fn ty(&mut self, schema: &Value, hint: &str) -> Ty {
        if let Some(name) = schema.get("$ref").and_then(Value::as_str) {
            return Ty::Named(type_name(name));
        }
        if let Some(options) = alternatives(schema) {
            let present: Vec<&Value> = options.iter().filter(|option| !is_null(option)).collect();
            if present.len() < options.len() {
                if let [one] = present.as_slice() {
                    return Ty::Opt(Box::new(self.ty(one, hint)));
                }
                let rest: Vec<Value> = present.into_iter().cloned().collect();
                let named = self.shape(&serde_json::json!({ "oneOf": rest }), hint, doc_of(schema));
                return Ty::Opt(Box::new(named));
            }
            return self.shape(schema, hint, doc_of(schema));
        }
        match schema.get("type") {
            Some(Value::Array(types)) => {
                let named: Vec<&str> = types.iter().filter_map(Value::as_str).collect();
                match named.as_slice() {
                    [one, "null"] | ["null", one] => {
                        let mut alone = schema.as_object().cloned().unwrap_or_default();
                        alone.insert("type".to_owned(), Value::String((*one).to_owned()));
                        Ty::Opt(Box::new(self.ty(&Value::Object(alone), hint)))
                    }
                    _ => self.problem(hint, "a value of several types other than one and null"),
                }
            }
            Some(Value::String(one)) => match one.as_str() {
                "string" => Ty::Str,
                "boolean" => Ty::Bool,
                "number" => Ty::Float,
                "null" => Ty::Unit,
                "integer" => match integer(schema.get("format").and_then(Value::as_str)) {
                    Some(held) => Ty::Int(held),
                    None => {
                        self.problem(hint, "an integer of a format this generator does not know")
                    }
                },
                "array" => {
                    if let Some(places) = schema.get("prefixItems").and_then(Value::as_array) {
                        let hints: Vec<String> =
                            (1..=places.len()).map(|at| format!("{hint}{at}")).collect();
                        Ty::Tuple(
                            places
                                .iter()
                                .zip(&hints)
                                .map(|(place, hint)| self.ty(place, hint))
                                .collect(),
                        )
                    } else {
                        let items = schema.get("items").cloned().unwrap_or(Value::Null);
                        Ty::List(Box::new(self.ty(&items, &format!("{hint}Item"))))
                    }
                }
                "object" => {
                    if properties(schema).is_some() {
                        self.shape(schema, hint, doc_of(schema))
                    } else {
                        match schema.get("additionalProperties") {
                            Some(Value::Object(values)) => {
                                let values = Value::Object(values.clone());
                                Ty::Map(Box::new(self.ty(&values, &format!("{hint}Value"))))
                            }
                            _ => Ty::Map(Box::new(Ty::Json)),
                        }
                    }
                }
                _ => self.problem(hint, "a type this generator does not know"),
            },
            _ => Ty::Json,
        }
    }

    fn shape(&mut self, schema: &Value, name: &str, doc: Option<String>) -> Ty {
        let body = self.body(schema, name);
        if self.shapes.contains_key(name) {
            self.problems.push(format!(
                "{name}: a definition takes the name the generator writes a shape under, or two shapes are written under it"
            ));
        }
        self.shapes.insert(
            name.to_owned(),
            Shape {
                name: name.to_owned(),
                doc,
                body,
            },
        );
        Ty::Named(name.to_owned())
    }

    fn body(&mut self, schema: &Value, name: &str) -> Body {
        if let Some(options) = alternatives(schema) {
            return self.union(options, name);
        }
        if let Some(fields) = properties(schema) {
            if matches!(schema.get("additionalProperties"), Some(Value::Object(_))) {
                self.problems.push(format!(
                    "{name}: an object with fields and further keys of a shape, which this generator does not write"
                ));
            }
            return Body::Struct(self.fields(fields, &required(schema), name, &[]));
        }
        Body::Alias(self.ty(schema, name))
    }

    fn fields(
        &mut self,
        listed: &Map<String, Value>,
        required: &BTreeSet<&str>,
        owner: &str,
        leaving: &[&str],
    ) -> Vec<Field> {
        let mut written = Vec::new();
        let mut idents = BTreeSet::new();
        for (json, schema) in listed {
            if leaving.contains(&json.as_str()) {
                continue;
            }
            let ident = field(json);
            if ident.is_empty() || !idents.insert(ident.clone()) {
                self.problems.push(format!(
                    "{owner}: the field {json:?} is written as a name another field takes, or as none"
                ));
            }
            let held = self.ty(schema, &format!("{owner}{}", pascal(json)));
            let ty = if required.contains(json.as_str()) || matches!(held, Ty::Opt(_)) {
                held
            } else {
                Ty::Opt(Box::new(held))
            };
            written.push(Field {
                json: json.clone(),
                ident,
                doc: doc_of(schema),
                ty,
            });
        }
        written
    }

    fn union(&mut self, options: &[Value], name: &str) -> Body {
        if let Some(tag) = tag_of(options) {
            let mut variants = Vec::new();
            for option in options {
                let value = option
                    .pointer(&format!("/properties/{tag}"))
                    .and_then(constant)
                    .unwrap_or_default()
                    .to_owned();
                let variant = pascal(&value);
                let fields = properties(option).map_or_else(Vec::new, |listed| {
                    self.fields(
                        listed,
                        &required(option),
                        &format!("{name}{variant}"),
                        &[tag.as_str()],
                    )
                });
                variants.push(Variant {
                    name: variant,
                    wire: Some(value),
                    doc: doc_of(option),
                    holds: if fields.is_empty() {
                        Holds::Nothing
                    } else {
                        Holds::Fields(fields)
                    },
                });
            }
            return Body::Tagged {
                tag,
                variants: self.distinct(variants, name),
            };
        }
        if options.iter().all(is_keyed) {
            let mut variants = Vec::new();
            for option in options {
                if let Some(value) = constant(option) {
                    variants.push(Variant {
                        name: pascal(value),
                        wire: Some(value.to_owned()),
                        doc: doc_of(option),
                        holds: Holds::Nothing,
                    });
                }
                for (key, held) in properties(option).into_iter().flatten() {
                    let variant = pascal(key);
                    let ty = self.ty(held, &format!("{name}{variant}"));
                    variants.push(Variant {
                        name: variant,
                        wire: Some(key.clone()),
                        doc: doc_of(option).or_else(|| doc_of(held)),
                        holds: Holds::One(ty),
                    });
                }
            }
            return Body::External(self.distinct(variants, name));
        }
        let mut variants = Vec::new();
        for (at, option) in options.iter().enumerate() {
            let variant = untagged_name(option, at);
            let ty = self.ty(option, &format!("{name}{variant}"));
            variants.push(Variant {
                name: variant,
                wire: None,
                doc: doc_of(option),
                holds: Holds::One(ty),
            });
        }
        Body::Untagged(self.distinct(variants, name))
    }

    fn distinct(&mut self, variants: Vec<Variant>, name: &str) -> Vec<Variant> {
        let mut seen = BTreeSet::new();
        for variant in &variants {
            if variant.name.is_empty() || !seen.insert(variant.name.clone()) {
                self.problems.push(format!(
                    "{name}: two variants are written as {:?}",
                    variant.name
                ));
            }
        }
        variants
    }
}

fn tag_of(options: &[Value]) -> Option<String> {
    let first = options.first()?;
    properties(first)?.keys().find_map(|key| {
        options
            .iter()
            .all(|option| {
                option
                    .pointer(&format!("/properties/{key}"))
                    .and_then(constant)
                    .is_some()
                    && required(option).contains(key.as_str())
            })
            .then(|| key.clone())
    })
}

fn is_keyed(option: &Value) -> bool {
    constant(option).is_some()
        || properties(option).is_some_and(|listed| {
            listed.len() == 1
                && listed
                    .keys()
                    .all(|key| required(option).contains(key.as_str()))
                && option.get("additionalProperties") == Some(&Value::Bool(false))
        })
}

fn untagged_name(option: &Value, at: usize) -> String {
    if let Some(name) = option.get("$ref").and_then(Value::as_str) {
        return type_name(name);
    }
    match option.get("type").and_then(Value::as_str) {
        Some("string") => "Text".to_owned(),
        Some("integer") => "Integer".to_owned(),
        Some("number") => "Number".to_owned(),
        Some("boolean") => "Boolean".to_owned(),
        Some("array") => "List".to_owned(),
        _ => format!("Shape{}", at + 1),
    }
}

fn direct(ty: &Ty, into: &mut Vec<String>) {
    match ty {
        Ty::Named(name) => into.push(name.clone()),
        Ty::Opt(inner) | Ty::Boxed(inner) => direct(inner, into),
        Ty::Tuple(places) => {
            for place in places {
                direct(place, into);
            }
        }
        _ => {}
    }
}

fn named_in(ty: &Ty, into: &mut Vec<String>) {
    match ty {
        Ty::Named(name) => into.push(name.clone()),
        Ty::Opt(inner) | Ty::Boxed(inner) | Ty::List(inner) | Ty::Map(inner) => {
            named_in(inner, into);
        }
        Ty::Tuple(places) => {
            for place in places {
                named_in(place, into);
            }
        }
        _ => {}
    }
}

fn asked_of(
    shapes: &BTreeMap<String, Shape>,
    bodies: &[(String, String)],
    actions: &[ShapedAction],
) -> BTreeSet<String> {
    let mut waiting: Vec<String> = bodies.iter().map(|(_, name)| name.clone()).collect();
    for field in actions.iter().flat_map(|action| &action.fields) {
        named_in(&field.ty, &mut waiting);
    }
    let mut asked = BTreeSet::new();
    while let Some(name) = waiting.pop() {
        let fresh = asked.insert(name.clone());
        let held = shapes
            .get(&name)
            .filter(|_| fresh)
            .map(|shape| types_of(&shape.body))
            .unwrap_or_default();
        for ty in held {
            named_in(ty, &mut waiting);
        }
    }
    asked
}

fn taken_names(shapes: &BTreeMap<String, Shape>, asked: &BTreeSet<String>) -> Vec<String> {
    shapes
        .values()
        .filter(|shape| {
            !asked.contains(&shape.name)
                && variants_of(&shape.body)
                    .iter()
                    .any(|variant| variant.name == UNRECOGNISED)
        })
        .map(|shape| {
            format!(
                "{} holds a variant named {UNRECOGNISED}, the name this generator writes for a value it was not generated with",
                shape.name
            )
        })
        .collect()
}

fn variants_of(body: &Body) -> &[Variant] {
    match body {
        Body::External(variants) | Body::Untagged(variants) | Body::Tagged { variants, .. } => {
            variants
        }
        Body::Struct(_) | Body::Alias(_) => &[],
    }
}

fn held_types(variants: &[Variant]) -> Vec<&Ty> {
    variants
        .iter()
        .flat_map(|variant| match &variant.holds {
            Holds::Nothing => Vec::new(),
            Holds::One(ty) => vec![ty],
            Holds::Fields(fields) => fields.iter().map(|field| &field.ty).collect(),
        })
        .collect()
}

fn held_types_mut(variants: &mut [Variant]) -> Vec<&mut Ty> {
    variants
        .iter_mut()
        .flat_map(|variant| match &mut variant.holds {
            Holds::Nothing => Vec::new(),
            Holds::One(ty) => vec![ty],
            Holds::Fields(fields) => fields.iter_mut().map(|field| &mut field.ty).collect(),
        })
        .collect()
}

fn types_of(body: &Body) -> Vec<&Ty> {
    match body {
        Body::Struct(fields) => fields.iter().map(|field| &field.ty).collect(),
        Body::External(variants) | Body::Untagged(variants) | Body::Tagged { variants, .. } => {
            held_types(variants)
        }
        Body::Alias(ty) => vec![ty],
    }
}

fn types_of_mut(body: &mut Body) -> Vec<&mut Ty> {
    match body {
        Body::Struct(fields) => fields.iter_mut().map(|field| &mut field.ty).collect(),
        Body::External(variants) | Body::Untagged(variants) | Body::Tagged { variants, .. } => {
            held_types_mut(variants)
        }
        Body::Alias(ty) => vec![ty],
    }
}

fn boxed(shapes: &mut BTreeMap<String, Shape>) {
    let edges: BTreeMap<String, Vec<String>> = shapes
        .iter()
        .map(|(name, shape)| {
            let mut reached = Vec::new();
            for ty in types_of(&shape.body) {
                direct(ty, &mut reached);
            }
            (name.clone(), reached)
        })
        .collect();
    let reaches = |from: &str, to: &str| -> bool {
        let mut seen = BTreeSet::new();
        let mut waiting = vec![from.to_owned()];
        while let Some(at) = waiting.pop() {
            if at == to {
                return true;
            }
            if seen.insert(at.clone()) {
                waiting.extend(edges.get(&at).into_iter().flatten().cloned());
            }
        }
        false
    };
    for (name, shape) in shapes.iter_mut() {
        for ty in types_of_mut(&mut shape.body) {
            point(ty, &|target| reaches(target, name));
        }
    }
}

fn point(ty: &mut Ty, cycles: &dyn Fn(&str) -> bool) {
    match ty {
        Ty::Named(target) if cycles(target) => {
            *ty = Ty::Boxed(Box::new(ty.clone()));
        }
        Ty::Opt(inner) => point(inner, cycles),
        Ty::Tuple(places) => {
            for place in places {
                point(place, cycles);
            }
        }
        _ => {}
    }
}

pub fn shaped(artefact: &Artefact) -> Result<Shaped, Refused> {
    let mut shaper = Shaper {
        defs: artefact.defs.keys().map(|name| type_name(name)).collect(),
        shapes: BTreeMap::new(),
        problems: Vec::new(),
    };
    for written in written_names(artefact) {
        if shaper.defs.contains(&written) {
            let carriers: Vec<&str> = artefact
                .kinds
                .keys()
                .map(String::as_str)
                .filter(|kind| reaches(artefact, kind, &|name| type_name(name) == written))
                .collect();
            shaper.problems.push(format!(
                "the definition {written} takes a name this generator writes itself; it is carried by {}",
                if carriers.is_empty() { "no kind".to_owned() } else { carriers.join(", ") }
            ));
        }
    }
    for (name, schema) in &artefact.defs {
        shaper.shape(schema, &type_name(name), doc_of(schema));
    }
    let mut kinds = BTreeMap::new();
    for (kind, schema) in &artefact.kinds {
        let ty = shaper.ty(schema, &format!("{}Data", pascal(kind)));
        kinds.insert(kind.clone(), ty);
    }
    let mut bodies = Vec::new();
    for body in &artefact.bodies {
        let name = format!("{}Body", pascal(body.route.trim_start_matches("/api/")));
        shaper.shape(&body.schema, &name, doc_of(&body.schema));
        bodies.push((body.route.clone(), name));
    }
    let mut actions = Vec::new();
    for action in &artefact.actions {
        let ident = action_type(&action.name);
        let mut fields = Vec::new();
        let mut defaulted = true;
        for (name, schema) in &action.arguments {
            let held = shaper.ty(
                schema,
                &format!("{}Arguments{}", pascal(&action.name), pascal(name)),
            );
            let optional = schema.get("default").is_some();
            defaulted &= optional || matches!(held, Ty::Opt(_));
            fields.push(Field {
                json: name.clone(),
                ident: field(name),
                doc: doc_of(schema),
                ty: if optional && !matches!(held, Ty::Opt(_)) {
                    Ty::Opt(Box::new(held))
                } else {
                    held
                },
            });
        }
        if action.rehearsal && !action.arguments.iter().any(|(name, _)| name == "dry_run") {
            fields.push(Field {
                json: "dry_run".to_owned(),
                ident: "dry_run".to_owned(),
                doc: Some(
                    "Whether to rehearse: write nothing, and answer with what the action would do."
                        .to_owned(),
                ),
                ty: Ty::Opt(Box::new(Ty::Bool)),
            });
        }
        actions.push(ShapedAction {
            name: action.name.clone(),
            ident,
            fields,
            defaulted,
            consent: action.consent.clone(),
            rehearsal: action.rehearsal,
        });
    }
    let mut written = shaper.shapes;
    boxed(&mut written);
    let asked = asked_of(&written, &bodies, &actions);
    shaper.problems.extend(taken_names(&written, &asked));
    if shaper.problems.is_empty() {
        Ok(Shaped {
            shapes: written,
            kinds,
            actions,
            bodies,
            asked,
        })
    } else {
        Err(Refused::listing(
            "the vendored contract holds a shape this generator cannot write, and nothing was written",
            &shaper.problems,
        ))
    }
}

fn written_names(artefact: &Artefact) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = WRITTEN
        .iter()
        .chain(PRELUDE)
        .map(|name| (*name).to_owned())
        .collect();
    names.extend(
        artefact
            .actions
            .iter()
            .map(|action| action_type(&action.name)),
    );
    for read in &artefact.reads {
        names.insert(read_type(&read.path));
        names.insert(answer_type(&read.path));
    }
    names
}

fn reaches(artefact: &Artefact, kind: &str, target: &dyn Fn(&str) -> bool) -> bool {
    let mut seen = BTreeSet::new();
    let mut waiting: Vec<&Value> = artefact.kinds.get(kind).into_iter().collect();
    while let Some(node) = waiting.pop() {
        match node {
            Value::Object(fields) => {
                if let Some(name) = fields.get("$ref").and_then(Value::as_str) {
                    if target(name) {
                        return true;
                    }
                    if seen.insert(name.to_owned()) {
                        waiting.extend(artefact.defs.get(name));
                    }
                }
                waiting.extend(fields.values());
            }
            Value::Array(items) => waiting.extend(items),
            _ => {}
        }
    }
    false
}
