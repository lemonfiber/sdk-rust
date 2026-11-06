/// The service manager a machine has, or the absence of one lemonfiber configures.
///
/// The absence is the default, because a machine nobody has told is a machine
/// nothing is known about, and guessing at a manager is how a report comes to
/// claim a platform it never asked.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Manager {
    /// macOS, through a launch agent in the operator's own login session.
    #[serde(rename = "launchd")]
    Launchd,
    /// Linux, through a user service in the operator's own session.
    #[serde(rename = "systemd")]
    Systemd,
    /// A platform lemonfiber does not configure.
    #[serde(rename = "unsupported")]
    Unsupported,
}

/// The kinds of thing a household holds.
///
/// Named rather than passed through as the server's own word, because a surface
/// drawing "Series" against one server and "tvshow" against another would be
/// rendering a detail of which server this household runs.
///
/// `Medium` rather than `Kind`, `Holding` or `Sort`: this product already calls the two
/// request services a [`crate::media::Kind`], a request's suspension a
/// [`crate::service::asking::Holding`], and what one line of a manifest is a
/// `uninstall::Sort` — and one word meaning two things in one vocabulary is how a
/// reader comes to trust the wrong one. The contract flattens every type name into one
/// namespace, so a clash there is a clash for anything reading it by name.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Medium {
    /// One film.
    #[serde(rename = "film")]
    Film,
    /// A television series, rather than one episode of one.
    #[serde(rename = "series")]
    Series,
    /// One episode of a series.
    #[serde(rename = "episode")]
    Episode,
    /// Something the server holds that is neither, and is not hidden for that.
    #[serde(rename = "other")]
    Other,
}

/// One entry in an archive's contents listing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Member {
    /// Where it sits inside the archive.
    pub archive_path: String,
    /// What it is, in the operator's terms.
    pub label: String,
}

/// What one member may watch, in the household's own words.
///
/// Read off the media server's account rather than kept here: that is where access is
/// decided, and a second copy is a copy able to disagree.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberAccess {
    /// Whether the account administers the media server.
    pub administrator: bool,
    /// The highest rating they may watch, where the operator set a limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_limit: Option<u32>,
    /// Whether the account is switched off — held, but unable to sign in.
    pub disabled: bool,
    /// Every library, rather than a chosen few. The ordinary case.
    pub every_library: bool,
    /// The libraries they may watch where it is not every one, by the names the
    /// operator gave them — or by the server's identifiers where the library list
    /// could not be read, which a finding says.
    pub libraries: Vec<String>,
    /// What that limit comes to in the certificates this media server names, in the
    /// operator's own country. Absent where no limit is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rated: Option<super::Rated>,
    /// What they are held to, in one word — including where what they may watch and
    /// what they may ask for disagree.
    pub restriction: super::Restriction,
    /// What becomes of content the media server has no rating for.
    ///
    /// Said either way, because an unexplained absence is the thing this answers: a
    /// restricted member missing half the library is either this setting or a defect,
    /// and an operator cannot tell which from silence.
    pub unrated: super::Unrated,
}

/// What one member may ask for, where the request service could be asked.
///
/// Both counts, because the service keeps them apart and folding them would report a
/// household as within its limit while the half that matters is spent: television is
/// counted a season at a time, so one ask for a six-season series spends six.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberAsking {
    /// Films, counted one to a request.
    pub films: super::Counted,
    /// When the count next lets go of something, so one more becomes possible.
    ///
    /// Absent where nothing limits them, and where the request service's own dates
    /// could not be read — an invented one would be a promise about a day on which
    /// nothing happens. The period is a window that rolls rather than a month that
    /// ends, so this is the moment their earliest counted request ages out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frees_up: Option<String>,
    /// What happens to what they ask for.
    pub policy: super::Policy,
    /// Where they stand against what their period allows, taken over both counts.
    pub standing: super::AskingStanding,
    /// Television, counted one to a season.
    pub television: super::Counted,
}

/// One thing a household member asked for, and where it stands in their words.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberRequest {
    /// When the title arrived on the media server, as the request service timestamps it.
    /// Absent, not null, until it is there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrived: Option<String>,
    /// About how much room it will want, at the quality in force.
    ///
    /// A guess and labelled as one — see [`crate::asking::Estimate`]. Absent where the
    /// request service names a kind this build does not know, since there is nothing to
    /// guess the length of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimate: Option<super::Estimate>,
    /// The number the request service files it under, which is how one is named again
    /// when somebody rules on it.
    pub id: i64,
    /// What kind of thing it is — a series, a film — in the household's own words.
    /// Absent where the request service names a kind this build does not know.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    /// Why it was turned down, where it was turned down from here.
    ///
    /// **The request service keeps none**, so this is lemonfiber's own record and is
    /// said to be — a reason presented as delivered would end the operator's job at
    /// exactly the point it begins. Absent on a request nobody has refused, and on one
    /// refused in the request service itself, where there are no words to report and
    /// inventing some would put them in somebody's mouth.
    ///
    /// Whether the words were carried to the person who asked is the record's own
    /// `told`, which is why the two travel together: what an operator does next turns
    /// on it, and a reason read without it is a reason of unknown standing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refused: Option<super::Refused>,
    /// The identifier the media server holds the title under, which is the one the held
    /// read names it by, so a request that has arrived can be found on the shelf. Absent,
    /// not null, until it is there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shelf_id: Option<String>,
    /// Where the request stands, or absent where the request service reports a status
    /// this build does not know rather than guessing it into the nearest word.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<super::RequestState>,
    /// What it is called: from the library of the service filing it, where it has been
    /// told about it and that library could be read, and otherwise from the request
    /// service's own record of the title. Absent where neither names it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// How many whole days it has been waiting on somebody, where it is waiting at all
    /// and the service's own date could be read.
    ///
    /// Only on the ones nobody has ruled on. A request already answered has not been
    /// waiting since it was made, and a figure beside one would be counting the wrong
    /// thing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting_days: Option<u64>,
    /// The year that title came out, where the service that named it knows one. Absent,
    /// not null, otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
}

/// Where one member's account stands.
///
/// **Switched off is its own answer rather than a flag beside another.** An account the
/// media server switched off after too many wrong passwords reads, on every other field,
/// exactly like one that works — and the person holding it has only been told their
/// password is wrong. What unlocks it is a reissue, which is the operator's to do, so the
/// operator is the one who has to be able to see it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MemberStanding {
    /// Offered, and the offer still stands: nobody has set a password yet.
    #[serde(rename = "invited")]
    Invited,
    /// Offered, and the offer ran out before anybody claimed it. The stack takes it back
    /// within a minute: an account nobody was seen in is removed and still listed as
    /// this until the next invitation, and a reset is switched off and kept.
    #[serde(rename = "expired")]
    Expired,
    /// Offered, and the person refused it at its decline address. The account is kept,
    /// switched off, until the operator removes it or reissues it; no sweep takes it
    /// back as run out.
    #[serde(rename = "declined")]
    Declined,
    /// Claimed, and able to sign in.
    #[serde(rename = "active")]
    Active,
    /// Switched off — by the operator, or by the media server after too many wrong
    /// passwords. The account and what they watched are kept, and a reissue switches
    /// it back on.
    #[serde(rename = "suspended")]
    Suspended,
}

/// One repair, and what became of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mended {
    /// How it turned out, once the check was asked again.
    pub outcome: super::RepairOutcome,
    /// What was proposed.
    pub repair: super::Repair,
}

/// What the stack itself moved in a calendar month, and what that leaves out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metered {
    /// Bytes pulled down, as far as the clients count them.
    pub down: u64,
    /// What this count does not include, always said.
    pub excludes: String,
    /// What is known to be missing from the count itself, where anything is.
    pub incomplete: Vec<String>,
    /// The month, as the client that dated the figures dates them.
    pub month: String,
    /// Bytes given back.
    pub up: u64,
}

/// What is already on this machine, before anything is proposed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MigrationReport {
    /// Where each service would listen to run beside the existing setup.
    pub beside: Vec<super::MovedReport>,
    /// What adopting each recognised service would come to, by service name.
    pub carrying: Vec<super::CarryingReport>,
    /// Ports wanted by lemonfiber that an existing service already holds.
    pub conflicts: Vec<super::ConflictReport>,
    /// What the existing layout costs where it cannot hold a hardlink, absent where
    /// it can.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linking: Option<super::LinkingReport>,
    /// What may be done about what was found, least destructive first.
    pub modes: Vec<super::ModeReport>,
    /// What no migration carries across, whatever mode it runs in.
    pub not_carried: Vec<super::UnsupportedReport>,
    /// Whether the engine answered at all.
    ///
    /// False means the survey found nothing because it could not look, which is a
    /// different answer from finding nothing, and the only one that must never be
    /// read as an empty machine.
    pub read: bool,
    /// Existing projects, by project name.
    pub standing: Vec<super::StandingReport>,
    /// What was found and cannot be adopted.
    pub unsupported: Vec<super::UnsupportedReport>,
}

/// A key minted, with everything a client on another machine needs to use it.
///
/// The only document that ever carries the secret. Every surface that renders it does so
/// once, and nothing here writes it down.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MintedKey {
    /// Where a client on another machine reaches the stack: the `https` address it was
    /// last served at on the network. Absent where it has never been served that way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// What is worth knowing before handing the key over, where anything is: how to
    /// serve the stack so another machine can reach it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// The name it was minted under.
    pub name: String,
    /// The certificate that address presents: SHA-256 over its DER encoding, in
    /// lower-case hex. A client off this machine pins it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    /// What the minter said it is for.
    pub purpose: super::KeyPurpose,
    /// What it admits: `read`, `act` or `member:<name>`.
    pub scope: String,
    /// The secret, sent in `X-Lemonfiber-Token`. It is not shown again.
    pub secret: super::Secret,
}

/// One thing an operator may do about a setup already here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModeReport {
    /// Whether carrying it out stops or alters what is already running.
    pub disturbs: bool,
    /// The word an operator types for it.
    pub mode: String,
    /// Whether it is offered already chosen. Only adopting is.
    pub preselected: bool,
    /// What choosing it would come to, in the operator's terms.
    pub what: String,
}

/// Which way a condition went.
///
/// Both directions are worth saying and neither is worth saying twice. An operator
/// told a disk filled up and never told it was resolved goes on believing it — so
/// resolution is an alert in its own right rather than the absence of one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Moment {
    /// It started.
    #[serde(rename = "onset")]
    Onset,
    /// It stopped, having previously started.
    #[serde(rename = "resolved")]
    Resolved,
}

/// A move in place the bundle offers: the plugin and the stack's own service of its id it
/// would take the place of.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MoveOffered {
    /// The part's adapter that writes the configuration the service reads, where one does:
    /// it runs only once the move is taken, so until then the service starts without what
    /// that configuration holds, the credentials it reaches other services with included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configured_by: Option<String>,
    /// The plugin the bundle installs.
    pub plugin: String,
    /// The stack's own service it would take the place of, under the same container name,
    /// configuration directory, data and names.
    pub service: String,
}

/// Where one service would listen to run beside what is already here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MovedReport {
    /// The port it would ordinarily take.
    pub from: u16,
    /// The lemonfiber service being moved.
    pub service: String,
    /// The port it would take instead.
    pub to: u16,
}

/// One audio-format choice in force, for media that has no resolution — the same
/// question as a [`PresetChoice`], answered in format terms rather than resolution.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MusicChoice {
    /// The format's plain-language name.
    pub format: String,
    /// What it means, in the operator's terms.
    pub means: String,
    /// The practical caveat worth knowing — playing it, or finding it.
    pub note: String,
    /// What this applies to — `music`.
    pub scope: String,
    /// Roughly how much disk an hour of it takes.
    pub size_per_hour: String,
    /// The audio format it targets, in plain terms.
    pub targets: String,
}

/// What choosing an audio format for music did: the choice, whether it was recorded
/// or only rehearsed, and — once recorded — what became of applying it to the music
/// service.
///
/// Music has no resolution and no community profile to lean on, so unlike a resolution
/// preset the choice is carried straight to the service through its API. The choice is
/// still recorded first, so it is remembered even when the service cannot be reached.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MusicReport {
    /// What the choice changes, in the stack's words.
    pub changes: String,
    /// The format chosen, what it means, and what it costs.
    pub choice: super::MusicChoice,
    /// Whether the choice was recorded, or only rehearsed.
    pub disposition: super::Disposition,
    /// What became of applying it to the music service, or `None` for a rehearsal
    /// that applied nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<super::Triggered>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}
