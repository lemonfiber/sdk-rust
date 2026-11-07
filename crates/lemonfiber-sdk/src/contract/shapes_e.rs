/// A setting changed outside lemonfiber since it last wrote one.
///
/// Both sides, so the operator chooses between them rather than being told one of
/// them lost. Values a listing withholds are withheld here too — a report a script
/// can log must not be the one place a password is printed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Edited {
    /// What the file holds now.
    pub found: String,
    /// Whether either value was withheld rather than shown.
    pub secret: bool,
    /// What lemonfiber last wrote there.
    pub wrote: String,
}

/// A request one of the stack's services makes, which is not lemonfiber's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Elsewhere {
    /// Where its requests go, in the terms an operator would recognise.
    ///
    /// Empty means it reaches nothing, which is an answer. It never means *and we
    /// do not know*: that is [`Self::recorded`], and the two must not be read as one
    /// — an unknown service rendered as an empty destination would be this product
    /// claiming nothing leaves the machine on the strength of having no idea.
    pub destination: String,
    /// Whose request it is: the stack's own, or an installed plugin's, named.
    ///
    /// A column in this account rather than an account of its own, because what leaves
    /// this machine is one question however many parties are asking it.
    pub origin: super::ValueOrigin,
    /// What it asks for.
    pub purpose: String,
    /// Whether lemonfiber ships a record of what this service reaches.
    ///
    /// False for a service that arrived in the stack after this build was made, or
    /// from an operator's own fork. It is listed anyway, because the alternative —
    /// leaving it out — is a privacy inventory that is complete-looking and short,
    /// and a reader counting the services on their machine against the ones on this
    /// list is the reader this surface exists for.
    pub recorded: bool,
    /// The service, by the id the stack declares it under.
    pub service: String,
}

/// How one service's update ended.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Ending {
    /// Updated, and answering again afterwards.
    #[serde(rename = "updated")]
    Updated,
    /// The newer image was never fetched, so nothing about this service changed.
    #[serde(rename = "not-fetched")]
    NotFetched,
    /// It was started on the newer image and did not come back.
    #[serde(rename = "not-started")]
    NotStarted,
    /// The run halted before reaching it, so it is standing where it was.
    #[serde(rename = "not-reached")]
    NotReached,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One change, as a reader meets it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    /// Where it was reviewed, where it was reviewed anywhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The requirements it served, which are the link rather than the headline.
    pub requirements: Vec<String>,
    /// What changed, in the words it was written in.
    pub summary: String,
}

/// One episode, with where it is served.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Episode {
    /// Where its backdrop is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<String>,
    /// The certificate the door presents, which a client pins, beside any location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door: Option<super::Pinned>,
    /// The identifier the server tells it apart by, which is what asking to play one
    /// of them names.
    pub id: String,
    /// Which of the kinds this product deals in it is.
    pub medium: super::Medium,
    /// How long it runs, in whole minutes, where the server knows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<u32>,
    /// Its number in the season, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    /// What happens in it, where the server holds a description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    /// Where its poster is served, where it has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    /// Where it streams from, where it plays.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_from: Option<String>,
    /// What it is called, in the words the server holds it under.
    pub title: String,
    /// Why no location is stated, where none is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlocated: Option<String>,
    /// The year it came out, where the server knows one. Absent rather than guessed:
    /// two films share a title far more often than they share a title and a year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
}

/// About how much room one request will want.
///
/// Carried as a number and a word rather than as a rendered string, so a surface can
/// put it in a column and still say what it is. The word is not decoration: a figure
/// with nothing hedging it is a promise, and this cannot keep one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Estimate {
    /// About how many bytes it will take.
    pub bytes: u64,
    /// Whether this was measured. Always false — nothing here has measured anything,
    /// and the field is present so that a surface rendering it cannot forget to say so.
    pub measured: bool,
}

/// One event kind the operator set apart from the preset.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExceptionReport {
    /// The kind of event, by the name a finding gives it.
    pub kind: String,
    /// Whether it is heard about, whatever the preset would say.
    pub wanted: bool,
}

/// What the answer has to be.
///
/// A status is a claim about the network path rather than about the service: Docker
/// publishes a port by putting a proxy in front of it, and that proxy accepts a
/// connection before knowing whether anything inside is listening. So a status alone
/// is not evidence — except for a refusal or a redirect to HTTPS, answers no port proxy
/// produces.
///
/// **A key of the four key-wise constraints is a place rather than a name.** A plain
/// name is a top-level member, and one beginning with `/` is a JSON Pointer, extended
/// with a step that picks an entry of a list by a field it holds. A flat name was enough while every service answered a flat object, and the
/// only thing it could say about a service that nests its payload was that the envelope
/// was there — which is a probe that passes by observing that something replied.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Expect {
    /// What the body must begin with, where it is not JSON.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_starts_with: Option<String>,
    /// A substring of the content type the answer was served as.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The exit status a run must end with. Not sufficient alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<i64>,
    /// Places the answer must carry, each with the exact value it must hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<std::collections::BTreeMap<String, super::Expected>>,
    /// The answer read as an array, with at least this many entries.
    ///
    /// A catalogue is very often a list rather than an object, and none of the
    /// object-shaped constraints can say anything about one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_array_min: Option<u64>,
    /// Places the answer must carry, each with a number it must not be below.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_at_least: Option<std::collections::BTreeMap<String, i64>>,
    /// Places the answer must carry, whatever they hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_has_keys: Option<Vec<String>>,
    /// The answer did not parse as JSON at all.
    ///
    /// Which is what a service that serves its application shell for every path it does
    /// not implement answers — and the reason a status alone proves nothing against
    /// one: the shell comes back `200` whether the API behind it exists or not, so what
    /// has to be said is that the body was *not* a document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_is_absent: Option<bool>,
    /// Places the answer must carry, each with the kind of value it must be.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_types: Option<std::collections::BTreeMap<String, super::ExpectedKind>>,
    /// The status the answer must carry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// What a run's standard output must begin with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout_starts_with: Option<String>,
}

/// Exactly what a place must hold.
///
/// Three kinds and no nesting: a flag that must be set, a number that must match, or a
/// word. A value deeper than this is asking about a document rather than about a claim
/// — and where the thing worth asserting is deeper *in* the answer, the key reaches it
/// rather than the value growing to match.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Expected {
    /// A true or a false.
    Boolean(bool),
    /// A whole number.
    Integer(i64),
    /// A word.
    Text(String),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// The kind of value a key must hold.
///
/// Five, and closed. A name outside them is one no runner could evaluate, and an
/// assertion nothing evaluates is a proof that silently checks less than it says —
/// which is worse than one that fails.
///
/// Named for what it is the kind *of*, rather than `Kind`, because this is published
/// and whoever generates from it flattens every definition into one scope. `Kind` is
/// the one name there a generator is certain to want for itself: every envelope this
/// contract describes is keyed by its `kind`, so the union of them is a `Kind` too, and
/// two of them in one module is a definition nothing can be compiled against.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ExpectedKind {
    /// A true or a false.
    #[serde(rename = "bool")]
    Bool,
    /// A whole number.
    #[serde(rename = "int")]
    Int,
    /// A string.
    #[serde(rename = "str")]
    Str,
    /// An array.
    #[serde(rename = "list")]
    List,
    /// An object.
    #[serde(rename = "dict")]
    Dict,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
