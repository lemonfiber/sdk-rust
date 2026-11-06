//! Every read the web API serves: a struct for each, holding what it takes.

/// `GET /api/version`, answered with `version`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Version {}

impl crate::Answer for super::VersionReport {
    const TAGGED: bool = false;
}

impl crate::Read for Version {
    type Answer = super::VersionReport;
    const PATH: &'static str = "/api/version";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["version"];
}

/// `GET /api/forms`, answered with `forms` or `preview`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Forms {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
}

/// What `GET /api/forms` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum FormsAnswer {
    /// The `forms` kind.
    #[serde(rename = "forms")]
    Forms(Box<super::FormsReport>),
    /// The `preview` kind.
    #[serde(rename = "preview")]
    Preview(Box<super::Plan>),
}

impl crate::Answer for FormsAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Forms {
    type Answer = FormsAnswer;
    const PATH: &'static str = "/api/forms";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["forms", "preview"];
}

/// `GET /api/status`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Status {}

impl crate::Answer for super::StatusReport {
    const TAGGED: bool = false;
}

impl crate::Read for Status {
    type Answer = super::StatusReport;
    const PATH: &'static str = "/api/status";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["status"];
}

/// `GET /api/services`, answered with `status`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Services {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
}

impl crate::Read for Services {
    type Answer = super::StatusReport;
    const PATH: &'static str = "/api/services";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["status"];
}

/// `GET /api/checks`, answered with `doctor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Checks {
    /// The `only` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<String>,
}

impl crate::Answer for super::DoctorReport {
    const TAGGED: bool = false;
}

impl crate::Read for Checks {
    type Answer = super::DoctorReport;
    const PATH: &'static str = "/api/checks";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["doctor"];
}

/// `GET /api/storage`, answered with `doctor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Storage {}

impl crate::Read for Storage {
    type Answer = super::DoctorReport;
    const PATH: &'static str = "/api/storage";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["doctor"];
}

/// `GET /api/requests`, answered with `household`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Requests {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::Answer for super::HouseholdReport {
    const TAGGED: bool = false;
}

impl crate::Read for Requests {
    type Answer = super::HouseholdReport;
    const PATH: &'static str = "/api/requests";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["household"];
}

/// `GET /api/held`, answered with `held`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Held {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
    /// The `most` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most: Option<String>,
}

impl crate::Answer for super::HeldReport {
    const TAGGED: bool = false;
}

impl crate::Read for Held {
    type Answer = super::HeldReport;
    const PATH: &'static str = "/api/held";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["held"];
}

/// `GET /api/held/{id}`, answered with `title`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldId {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::Answer for super::TitleReport {
    const TAGGED: bool = false;
}

impl crate::Read for HeldId {
    type Answer = super::TitleReport;
    const PATH: &'static str = "/api/held/{id}";
    const SEGMENTS: &'static [&'static str] = &["id"];
    const KINDS: &'static [&'static str] = &["title"];
}

/// `GET /api/watching`, answered with `part-way`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Watching {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `most` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most: Option<String>,
}

impl crate::Answer for super::PartWayReport {
    const TAGGED: bool = false;
}

impl crate::Read for Watching {
    type Answer = super::PartWayReport;
    const PATH: &'static str = "/api/watching";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["part-way"];
}

/// `GET /api/askable`, answered with `askable`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Askable {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `term` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The `after` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl crate::Answer for super::AskableReport {
    const TAGGED: bool = false;
}

impl crate::Read for Askable {
    type Answer = super::AskableReport;
    const PATH: &'static str = "/api/askable";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["askable"];
}

/// `GET /api/playing`, answered with `playing`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Playing {
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
}

impl crate::Answer for super::PlayingReport {
    const TAGGED: bool = false;
}

impl crate::Read for Playing {
    type Answer = super::PlayingReport;
    const PATH: &'static str = "/api/playing";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["playing"];
}

/// `GET /api/hosting`, answered with `hosting`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Hosting {}

impl crate::Answer for super::HostingReport {
    const TAGGED: bool = false;
}

impl crate::Read for Hosting {
    type Answer = super::HostingReport;
    const PATH: &'static str = "/api/hosting";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["hosting"];
}

/// `GET /api/front-door`, answered with `front-door`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct FrontDoor {}

impl crate::Answer for super::FrontDoorReport {
    const TAGGED: bool = false;
}

impl crate::Read for FrontDoor {
    type Answer = super::FrontDoorReport;
    const PATH: &'static str = "/api/front-door";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["front-door"];
}

/// `GET /api/news`, answered with `news-items`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct News {}

impl crate::Answer for super::News {
    const TAGGED: bool = false;
}

impl crate::Read for News {
    type Answer = super::News;
    const PATH: &'static str = "/api/news";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["news-items"];
}

/// `GET /api/trace`, answered with `trace`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Trace {
    /// The `term` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// The `season` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
}

impl crate::Answer for super::TraceReport {
    const TAGGED: bool = false;
}

impl crate::Read for Trace {
    type Answer = super::TraceReport;
    const PATH: &'static str = "/api/trace";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["trace"];
}

/// `GET /api/stuck`, answered with `stuck`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Stuck {}

impl crate::Answer for super::StuckReport {
    const TAGGED: bool = false;
}

impl crate::Read for Stuck {
    type Answer = super::StuckReport;
    const PATH: &'static str = "/api/stuck";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["stuck"];
}

/// `GET /api/config`, answered with `config`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Config {
    /// The `key` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl crate::Answer for super::ConfigReport {
    const TAGGED: bool = false;
}

impl crate::Read for Config {
    type Answer = super::ConfigReport;
    const PATH: &'static str = "/api/config";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["config"];
}

/// `GET /api/quality`, answered with `quality`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Quality {}

impl crate::Answer for super::QualityReport {
    const TAGGED: bool = false;
}

impl crate::Read for Quality {
    type Answer = super::QualityReport;
    const PATH: &'static str = "/api/quality";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["quality"];
}

/// `GET /api/explain`, answered with `glossary` or `word`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Explain {
    /// The `word` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
}

/// What `GET /api/explain` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ExplainAnswer {
    /// The `glossary` kind.
    #[serde(rename = "glossary")]
    Glossary(Box<super::Vocabulary>),
    /// The `word` kind.
    #[serde(rename = "word")]
    Word(Box<super::Term>),
}

impl crate::Answer for ExplainAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Explain {
    type Answer = ExplainAnswer;
    const PATH: &'static str = "/api/explain";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["glossary", "word"];
}

/// `GET /api/backups`, answered with `archives`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Backups {}

impl crate::Answer for super::Listing {
    const TAGGED: bool = false;
}

impl crate::Read for Backups {
    type Answer = super::Listing;
    const PATH: &'static str = "/api/backups";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["archives"];
}

/// `GET /api/outbound`, answered with `outbound`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Outbound {}

impl crate::Answer for super::Leaving {
    const TAGGED: bool = false;
}

impl crate::Read for Outbound {
    type Answer = super::Leaving;
    const PATH: &'static str = "/api/outbound";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["outbound"];
}

/// `GET /api/provenance`, answered with `provenance`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Provenance {}

impl crate::Answer for super::ProvenanceReport {
    const TAGGED: bool = false;
}

impl crate::Read for Provenance {
    type Answer = super::ProvenanceReport;
    const PATH: &'static str = "/api/provenance";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["provenance"];
}

/// `GET /api/catalogue`, answered with `catalogue`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Catalogue {}

impl crate::Answer for super::CatalogueReport {
    const TAGGED: bool = false;
}

impl crate::Read for Catalogue {
    type Answer = super::CatalogueReport;
    const PATH: &'static str = "/api/catalogue";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["catalogue"];
}

/// `GET /api/stored`, answered with `stored`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Stored {}

impl crate::Answer for super::Stored {
    const TAGGED: bool = false;
}

impl crate::Read for Stored {
    type Answer = super::Stored;
    const PATH: &'static str = "/api/stored";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["stored"];
}

/// `GET /api/uninstall`, answered with `uninstall`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Uninstall {
    /// The `tier` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl crate::Answer for super::Uninstall {
    const TAGGED: bool = false;
}

impl crate::Read for Uninstall {
    type Answer = super::Uninstall;
    const PATH: &'static str = "/api/uninstall";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["uninstall"];
}

/// `GET /api/space`, answered with `space`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Space {}

impl crate::Answer for super::Reckoning {
    const TAGGED: bool = false;
}

impl crate::Read for Space {
    type Answer = super::Reckoning;
    const PATH: &'static str = "/api/space";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["space"];
}

/// `GET /api/bandwidth`, answered with `bandwidth`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Bandwidth {}

impl crate::Answer for super::Sharing {
    const TAGGED: bool = false;
}

impl crate::Read for Bandwidth {
    type Answer = super::Sharing;
    const PATH: &'static str = "/api/bandwidth";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["bandwidth"];
}

/// `GET /api/clients`, answered with `clients`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Clients {}

impl crate::Answer for super::Guidance {
    const TAGGED: bool = false;
}

impl crate::Read for Clients {
    type Answer = super::Guidance;
    const PATH: &'static str = "/api/clients";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["clients"];
}

/// `GET /api/alerts`, answered with `alerts`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Alerts {}

impl crate::Answer for super::AlertReport {
    const TAGGED: bool = false;
}

impl crate::Read for Alerts {
    type Answer = super::AlertReport;
    const PATH: &'static str = "/api/alerts";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["alerts"];
}

/// `GET /api/credentials`, answered with `credentials`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Credentials {}

impl crate::Answer for super::Inventory {
    const TAGGED: bool = false;
}

impl crate::Read for Credentials {
    type Answer = super::Inventory;
    const PATH: &'static str = "/api/credentials";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["credentials"];
}

/// `GET /api/migration`, answered with `migration`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Migration {}

impl crate::Answer for super::MigrationReport {
    const TAGGED: bool = false;
}

impl crate::Read for Migration {
    type Answer = super::MigrationReport;
    const PATH: &'static str = "/api/migration";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["migration"];
}

/// `GET /api/history`, answered with `history`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct History {}

impl crate::Answer for super::HistoryReport {
    const TAGGED: bool = false;
}

impl crate::Read for History {
    type Answer = super::HistoryReport;
    const PATH: &'static str = "/api/history";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["history"];
}

/// `GET /api/update`, answered with `self-update` or `update`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Update {
    /// The `what` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what: Option<String>,
    /// The `to` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// What `GET /api/update` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum UpdateAnswer {
    /// The `self-update` kind.
    #[serde(rename = "self-update")]
    SelfUpdate(Box<super::UpdateReport>),
    /// The `update` kind.
    #[serde(rename = "update")]
    Update(Box<super::StackUpdateReport>),
}

impl crate::Answer for UpdateAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Update {
    type Answer = UpdateAnswer;
    const PATH: &'static str = "/api/update";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["self-update", "update"];
}

/// `GET /api/plugins`, answered with `plugins`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Plugins {}

impl crate::Answer for super::PluginInstalls {
    const TAGGED: bool = false;
}

impl crate::Read for Plugins {
    type Answer = super::PluginInstalls;
    const PATH: &'static str = "/api/plugins";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["plugins"];
}

/// `GET /api/wiring`, answered with `wiring`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Wiring {}

impl crate::Answer for super::WiringReport {
    const TAGGED: bool = false;
}

impl crate::Read for Wiring {
    type Answer = super::WiringReport;
    const PATH: &'static str = "/api/wiring";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["wiring"];
}

/// `GET /api/settings`, answered with `settings`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Settings {}

impl crate::Answer for super::SettingsReport {
    const TAGGED: bool = false;
}

impl crate::Read for Settings {
    type Answer = super::SettingsReport;
    const PATH: &'static str = "/api/settings";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["settings"];
}

/// `GET /api/logs`, answered with `job` or `log`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Logs {
    /// The `form` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<String>,
    /// The `service` parameter, sent once for each value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<String>,
    /// The `tail` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    /// The `follow` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<String>,
}

/// What `GET /api/logs` answers with, by its kind.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum LogsAnswer {
    /// The `job` kind.
    #[serde(rename = "job")]
    Job(Box<super::Started>),
    /// The `log` kind.
    #[serde(rename = "log")]
    Log(Box<super::LogLine>),
}

impl crate::Answer for LogsAnswer {
    const TAGGED: bool = true;
}

impl crate::Read for Logs {
    type Answer = LogsAnswer;
    const PATH: &'static str = "/api/logs";
    const SEGMENTS: &'static [&'static str] = &[];
    const KINDS: &'static [&'static str] = &["job", "log"];
}

/// `GET /api/bundle/{name}`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct BundleName {
    /// The `name` segment of the path.
    pub name: String,
}

impl crate::FileRead for BundleName {
    const PATH: &'static str = "/api/bundle/{name}";
    const SEGMENTS: &'static [&'static str] = &["name"];
}

/// `GET /api/held/{id}/poster`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldIdPoster {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::FileRead for HeldIdPoster {
    const PATH: &'static str = "/api/held/{id}/poster";
    const SEGMENTS: &'static [&'static str] = &["id"];
}

/// `GET /api/held/{id}/backdrop`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct HeldIdBackdrop {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `defaults` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<String>,
}

impl crate::FileRead for HeldIdBackdrop {
    const PATH: &'static str = "/api/held/{id}/backdrop";
    const SEGMENTS: &'static [&'static str] = &["id"];
}

/// `GET /api/askable/{id}/poster`, answered with a file.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct AskableIdPoster {
    /// The `id` segment of the path.
    pub id: String,
    /// The `member` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The `kind` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl crate::FileRead for AskableIdPoster {
    const PATH: &'static str = "/api/askable/{id}/poster";
    const SEGMENTS: &'static [&'static str] = &["id"];
}
