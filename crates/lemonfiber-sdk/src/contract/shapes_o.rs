/// One container of somebody else's stack, as the engine reports it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OccupantReport {
    /// Whether lemonfiber knows this service and could take it over as it stands.
    pub adoptable: bool,
    /// Where it reads its configuration from on this machine.
    pub config: super::Location,
    /// Where its library is on this machine.
    pub library: super::Location,
    /// Every host port it publishes, lowest first.
    pub ports: Vec<u16>,
    /// Whether it is running now, as against present but stopped.
    pub running: bool,
    /// The Compose service name it answers to.
    pub service: String,
}

/// A request lemonfiber makes on an installed plugin's behalf, attributed to it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OnBehalf {
    /// Where it goes, as the plugin declared it.
    pub destination: String,
    /// The plugin that declared it, by its id.
    pub plugin: String,
    /// Why it may be made.
    pub purpose: String,
}

/// One thing a way of downloading newly asks the operator for.
///
/// What is opened and nothing beside it: an operator adding Usenet is shown the
/// Usenet provider and the settings its login is kept in, and never the tunnel,
/// which they neither need nor asked about.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Opening {
    /// Why this protocol needs it.
    pub because: String,
    /// The setting its answer is kept in, where it is kept in one. Absent for an
    /// account the operator has to go and obtain, which no setting holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting: Option<String>,
    /// What it is, in the operator's terms.
    pub what: String,
}

/// Who produced a credential, which decides what can be done about it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Origin {
    /// The operator supplied it, from an account they hold somewhere else.
    #[serde(rename = "operator")]
    Operator,
    /// The service minted it for itself and wrote it into its own configuration.
    #[serde(rename = "service")]
    Service,
    /// lemonfiber minted it, because the service offers nothing durable to read.
    #[serde(rename = "lemonfiber")]
    Lemonfiber,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One request lemonfiber makes, where it goes, and what refusing it costs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Outbound {
    /// Whether this machine's settings allow it.
    pub allowed: bool,
    /// What stops working once it is off.
    pub cost: String,
    /// Where it goes as this machine is configured. Empty where nothing is
    /// configured to reach, which is not the same as switched off.
    pub destination: Vec<String>,
    /// Why lemonfiber asks.
    pub purpose: String,
    /// Which request this is.
    pub reach: super::OutboundReach,
    /// Exactly what travels in the request.
    pub sends: String,
    /// The setting that switches it off.
    pub switch: String,
}

/// One of the requests lemonfiber makes on its own account.
///
/// Fifteen, and the closed set is the claim. A sixteenth is a decision somebody makes by
/// adding a variant here and answering four questions about it, rather than one
/// that happens by somebody building a request.
///
/// The sixth is the one that carries somebody's words rather than a credential or a
/// name, and it was added deliberately and late: five of these prove or fetch
/// something and could say what travels in a phrase, and this one travels to a
/// person. What that costs is the same four answers as the rest, and one more thing
/// the others do not owe — it is the only entry whose destination is somebody else's
/// choice, so the list names the two services it can reach and the sender is handed
/// them rather than holding addresses of its own.
///
/// The seventh is the only one this program makes about *itself*, and the one most easily
/// left off a list of what this program reaches. What it costs to allow is the shortest
/// answer on the list: it carries nothing at all, so the only thing switching it off
/// keeps from anybody is the knowledge that a version came out.
///
/// The eighth goes where the operator points it and nowhere else: a plugin's git
/// source, named at install. It is the only entry whose destination nobody chose in
/// advance, which is why it is made only when somebody names one.
///
/// The ninth goes to one place this build names, and only when an operator installs a
/// plugin by name: the catalogue's newest release, for its index and the signature over
/// it.
///
/// The tenth is made for a member rather than for the operator: the poster of a title
/// their search found, from the one catalogue address this build names, so their client
/// draws it without ever being sent there.
///
/// The last four deliver alerts to the back-ends the operator added, one for each kind,
/// so each kind is switched off on its own.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum OutboundReach {
    /// Fetching the service images the stack runs.
    #[serde(rename = "registry")]
    Registry,
    /// Probing the source the community quality guides are synced from.
    #[serde(rename = "guides")]
    Guides,
    /// Asking what public address this machine's traffic comes out of.
    #[serde(rename = "echo")]
    Echo,
    /// Proving an indexer key against the indexer.
    #[serde(rename = "indexer")]
    Indexer,
    /// Proving a Usenet login against the provider.
    #[serde(rename = "usenet")]
    Usenet,
    /// Telling a household member the one thing the request service cannot carry.
    #[serde(rename = "household")]
    Household,
    /// Asking which version of lemonfiber itself has been released.
    #[serde(rename = "updates")]
    Updates,
    /// Fetching a plugin from a git source the operator names.
    #[serde(rename = "plugin-source")]
    PluginSource,
    /// Reading the catalogue's index to resolve a plugin installed by name.
    #[serde(rename = "catalogue")]
    Catalogue,
    /// Fetching the poster of a title a member's search found.
    #[serde(rename = "posters")]
    Posters,
    /// Waking a phone that asked to be woken when this stack has something to say.
    #[serde(rename = "waking")]
    Waking,
    /// Delivering an alert to an ntfy topic.
    #[serde(rename = "ntfy")]
    Ntfy,
    /// Delivering an alert to a gotify server.
    #[serde(rename = "gotify")]
    Gotify,
    /// Posting an alert to a webhook.
    #[serde(rename = "webhook")]
    Webhook,
    /// Delivering an alert to an Apprise API.
    #[serde(rename = "apprise")]
    Apprise,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Something an uninstall leaves behind, and how to remove it by hand.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Outside {
    /// How to remove it on this platform, as the operator would type or do it.
    pub by_hand: String,
    /// Whether this machine was found to have it.
    ///
    /// A survey that could not look says nothing was found rather than that nothing
    /// is there, which is why the entry is listed either way and this field carries
    /// the difference.
    pub found: bool,
    /// What it is.
    pub what: String,
    /// Why it is not lemonfiber's to take away.
    pub why: String,
}

/// One file far larger than the rest.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Outsized {
    /// What it occupies.
    pub bytes: u64,
    /// Where it is.
    pub path: String,
    /// How many times the middle file of this walk it is.
    pub times_typical: u64,
}

/// What a run's findings amount to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Overall {
    /// Everything that ran passed.
    #[serde(rename = "healthy")]
    Healthy,
    /// Warnings, but nothing broken.
    #[serde(rename = "degraded")]
    Degraded,
    /// Something is broken.
    #[serde(rename = "broken")]
    Broken,
    /// Something could not be determined, so health is not a settled fact.
    #[serde(rename = "unknown")]
    Unknown,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Whose a setting or a secret is.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "of")]
#[non_exhaustive]
pub enum Owner {
    /// The core's.
    #[serde(rename = "core")]
    Core,
    /// A plugin's, or a stack service's, by its namespace.
    #[serde(rename = "namespace")]
    Namespace {
        /// The namespace.
        namespace: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Owner, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Owner {
    const TAG: Option<&str> = Some("of");
    const KNOWN: &[&str] = &["core", "namespace"];
}
