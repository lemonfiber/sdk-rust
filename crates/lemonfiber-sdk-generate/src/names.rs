const RESERVED: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let",
    "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return",
    "static", "struct", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use",
    "virtual", "where", "while", "yield",
];

const UNRAWABLE: &[&str] = &["crate", "self", "super", "Self"];

fn words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut previous_lower = false;
    for character in text.chars() {
        if !character.is_ascii_alphanumeric() {
            words.push(std::mem::take(&mut word));
            previous_lower = false;
            continue;
        }
        if character.is_ascii_uppercase() && previous_lower {
            words.push(std::mem::take(&mut word));
        }
        previous_lower = character.is_ascii_lowercase() || character.is_ascii_digit();
        word.push(character);
    }
    words.push(word);
    words.retain(|word| !word.is_empty());
    words
}

#[must_use]
pub fn pascal(text: &str) -> String {
    let joined: String = words(text)
        .iter()
        .map(|word| {
            let mut letters = word.chars();
            letters.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + &letters.as_str().to_ascii_lowercase()
            })
        })
        .collect();
    if joined.starts_with(|first: char| first.is_ascii_digit()) {
        format!("N{joined}")
    } else {
        joined
    }
}

#[must_use]
pub fn screaming(text: &str) -> String {
    words(text)
        .iter()
        .map(|word| word.to_ascii_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

#[must_use]
pub fn field(text: &str) -> String {
    let joined = words(text)
        .iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("_");
    if RESERVED.contains(&joined.as_str()) {
        format!("r#{joined}")
    } else if UNRAWABLE.contains(&joined.as_str())
        || joined.starts_with(|first: char| first.is_ascii_digit())
    {
        format!("{joined}_")
    } else {
        joined
    }
}

#[must_use]
pub fn is_type_name(name: &str) -> bool {
    name.starts_with(|first: char| first.is_ascii_uppercase())
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
}

#[must_use]
pub fn type_name(name: &str) -> String {
    name.split('_')
        .map(|part| {
            let mut letters = part.chars();
            letters.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + letters.as_str()
            })
        })
        .collect()
}

fn read_base(path: &str) -> String {
    pascal(path.trim_start_matches("/api/"))
}

#[must_use]
pub fn read_type(path: &str) -> String {
    format!("{}Read", read_base(path))
}

#[must_use]
pub fn answer_type(path: &str) -> String {
    format!("{}Answer", read_base(path))
}

#[must_use]
pub fn action_type(name: &str) -> String {
    format!("{}Action", pascal(name))
}

#[must_use]
pub fn literal(text: &str) -> String {
    format!("{text:?}")
}
