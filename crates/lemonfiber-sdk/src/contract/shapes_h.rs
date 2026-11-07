/// One app a device can be pointed at the stack with, and the code that points it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HandoffClient {
    /// What to use on it.
    pub client: String,
    /// What the code for this app carries: a link that opens the app at this server where
    /// the app takes one, and the server's address otherwise.
    pub code: String,
    /// Whether [`code`](Self::code) is such a link rather than the address alone.
    pub deep_link: bool,
    /// What somebody would call the device they are holding.
    pub device: String,
    /// Whether that app is open source. A closed one is never the recommended path.
    pub open_source: bool,
}

/// What there is to do next about a hand-off, which each surface says in its own words.
///
/// Carried as a name rather than as a sentence because the act is the same everywhere
/// and the way to take it is not: a terminal names a command, and an app offers a
/// control. A sentence written here would have to pick one of them, and every other
/// surface would then be showing somebody an instruction it cannot carry out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum HandoffRemedy {
    /// Invite them, which makes the account and is where what they may watch is chosen.
    #[serde(rename = "invite")]
    Invite,
    /// Ask again: once they have signed in on the device, or once the media server
    /// answers the question it would not.
    #[serde(rename = "ask-again")]
    AskAgain,
    /// See whether the media server is running, and start it where it is not.
    #[serde(rename = "start-server")]
    StartServer,
    /// Record the address the household reaches this machine at.
    #[serde(rename = "record-address")]
    RecordAddress,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where one person's hand-off stands, and what to hand them.
///
/// **The code is an address and nothing more.** Whoever holds a copy of it can find the
/// server and still has to sign in as somebody, so a code sent to the wrong phone, or
/// photographed over a shoulder, gives away where the server is and nothing else.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HandoffReport {
    /// The address the code carries. Absent where there is no address to carry, which is
    /// one of the ways a hand-off fails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// What is worth knowing about that address, where anything is: most often that it
    /// answers only on the home network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// The apps to point a device at the stack with, each with its code.
    pub clients: Vec<super::HandoffClient>,
    /// When a code was first issued for them, as an instant. Absent until one is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    /// Who it is for, as the media server spells their account where it holds one, and as
    /// it was asked for otherwise.
    pub name: String,
    /// Whether the media server offers the sign-in by short code, where one account already
    /// signed in approves another device.
    pub quick_connect: bool,
    /// Why it stands there, where that is not the state itself: why an account that is not
    /// there stops it, and what stopped one that failed. In words any surface can show, so
    /// it names no command; what to do about it is [`remedy`](Self::remedy).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Whether this was a rehearsal, in which no issue was written down.
    pub rehearsed: bool,
    /// What there is to do next, where there is anything: named rather than said, so
    /// that each surface offers it in its own way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remedy: Option<super::HandoffRemedy>,
    /// The devices signed in to the account now, as the media server lists them.
    pub sessions: Vec<super::HandoffSession>,
    /// Where it stands.
    pub state: super::HandoffState,
    /// How the person signs in on the new device, one step at a time.
    ///
    /// Every step is something the person does on their device, in words any surface can
    /// show. Asking again afterwards is not one of them: that is [`remedy`](Self::remedy).
    ///
    /// **Guidance and never an approval.** Where the sign-in asks for a short code to be
    /// approved from a device they are already signed in on, that approval is theirs: it
    /// is the step that proves the person holding the new device is the person the
    /// account is for, and a program that took it for them would have proved nothing.
    pub steps: Vec<String>,
}

/// One device the media server lists as signed in to the account.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HandoffSession {
    /// The app it signed in with.
    pub client: String,
    /// What the device calls itself.
    pub device: String,
    /// When the media server last heard from it, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

/// Where one person's hand-off stands.
///
/// Read from the media server each time rather than remembered, apart from when a code
/// was first issued and which devices were signed in then: whether a device is signed in
/// is the server's to say, and a copy kept here would go on saying it after the person
/// signed out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum HandoffState {
    /// Nobody by that name holds an account yet, so there is nothing to sign in to.
    ///
    /// No account is made here. Making one is an invitation, which is where what the
    /// person may watch is chosen; an account made on the way to handing over a phone
    /// would be one nobody chose anything for.
    #[serde(rename = "unprovisioned")]
    Unprovisioned,
    /// The account is there and the code was issued by this run.
    #[serde(rename = "ready")]
    Ready,
    /// The code went out on an earlier run and no device of theirs has signed in since.
    ///
    /// Told apart from [`Failed`](Self::Failed) because nothing has gone wrong: the next
    /// step is on the person's device, and until they take it there is nothing to prove.
    #[serde(rename = "pending")]
    Pending,
    /// The media server lists a device signed in to their account now that was not
    /// signed in when the code was first given.
    ///
    /// A phone they already had proves nothing about the one just handed over, so the
    /// devices signed in at that moment are not counted.
    #[serde(rename = "connected")]
    Connected,
    /// The hand-off could not go ahead, for the reason given beside it.
    #[serde(rename = "failed")]
    Failed,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Where a finished walkthrough leaves the operator.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Handover {
    /// What to do next, in order.
    pub next: Vec<super::Next>,
}

/// Whether imports are hardlinking or copying — the difference between an import
/// that is free and one that doubles the disk it uses.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Hardlink {
    /// Imports hardlink, as they should.
    #[serde(rename = "linking")]
    Linking,
    /// Imports copy, doubling space and slowing every import.
    #[serde(rename = "copying")]
    Copying,
    /// It could not be established which.
    #[serde(rename = "unknown")]
    Unknown,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What the stack amounts to.
///
/// Ordered from best to worst, so the worst of several is a `max` and there is no
/// second place to encode the ranking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum HealthStanding {
    /// Nothing is wrong.
    #[serde(rename = "healthy")]
    Healthy,
    /// Nothing is running, and that was on purpose.
    #[serde(rename = "stopped")]
    Stopped,
    /// Nothing is set up.
    #[serde(rename = "unconfigured")]
    Unconfigured,
    /// Worth knowing, nothing to do.
    #[serde(rename = "advisory")]
    Advisory,
    /// Working, with something wrong.
    #[serde(rename = "degraded")]
    Degraded,
    /// Something is broken.
    #[serde(rename = "broken")]
    Broken,
    /// Something is wrong outside this machine, or data is at risk.
    #[serde(rename = "critical")]
    Critical,
    /// It could not be established. Never reported as healthy.
    #[serde(rename = "unknown")]
    Unknown,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// The one-line summary, and what it expands to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HealthSummary {
    /// Everything that is wrong, worst first, so the line expands to the affected
    /// items and their remedies rather than to a number nobody can act on.
    pub affected: Vec<super::Affected>,
    /// The one word.
    pub standing: super::HealthStanding,
    /// How many things are wrong — root causes, counted once each, so a disk that
    /// filled and the nine imports that then failed is one thing and not ten.
    pub wanting_attention: u64,
    /// The worst thing, named, so the line says something rather than only
    /// grading. Absent where nothing is wrong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<String>,
}

/// One thing the household holds, as a member is shown it.
///
/// What a person recognises and nothing else. There is no file path, no container,
/// no bitrate and no library id: a member deciding what to watch is not choosing a
/// transcode, and a surface handed those would have to decide not to draw them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Held {
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

/// What one member can watch, and who they are.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HeldReport {
    /// Whether the shelf could be read at all.
    ///
    /// An empty shelf and an unread one are different answers, and collapsing them
    /// would tell a household they own nothing on the day the media server rebooted.
    /// Anything that could not be read is said in `findings` and this goes false.
    pub available: bool,
    /// What is worth saying about this shelf, in the words its reader would use.
    ///
    /// Always written, empty or not. A field the schema requires and the document
    /// sometimes omits is one a reader has to guess about, and an empty list already
    /// says the thing it would say: there is nothing to report about this shelf.
    pub findings: Vec<String>,
    /// What they hold, newest first.
    pub holdings: Vec<super::Held>,
    /// The identifier the media server files them under.
    pub id: String,
    /// The member this was asked for, by the name they are known by.
    pub member: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// Everything lemonfiber changed, most recent first.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HistoryReport {
    /// The changes, newest first.
    pub changes: Vec<super::ChangeReport>,
    /// How far back the record goes, in the operator's terms.
    ///
    /// Stated rather than left to be inferred from the oldest entry: a record that has
    /// been trimmed and one that has always been short look identical from the entries
    /// alone, and only one of them means something is missing.
    pub horizon: String,
}

/// One download client, and what became of the limits it was given.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Holding {
    /// What it said.
    pub answer: super::Answer,
    /// The client, by the name the stack knows it under.
    pub client: String,
    /// Whether it is fetching at all, where a declared cap made that a question.
    ///
    /// Absent on a stack with no cap rather than assumed to be fetching: asking
    /// every client whether it has stopped, on a stack where nothing would ever
    /// stop it, is traffic spent on a figure nothing would act on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulling: Option<super::Pulling>,
}

/// One long-running command, and what stands between it and this machine.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HostedCommand {
    /// How it is typed in a terminal, which is what hosting installs.
    pub command: String,
    /// The service definition installed for it, where there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// What it does for as long as it runs, in one sentence.
    pub guarantees: String,
    /// The program the definition names, where nothing is there any more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<String>,
    /// lemonfiber's own name for it.
    pub name: String,
    /// Where a hosted run writes the words it would have said on a terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The whole command line that definition runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<String>,
    /// What stands between it and the machine.
    pub standing: super::Hosting,
}

/// What stands between one long-running command and the machine.
///
/// Written with hyphens because these are the words the operator reads and the
/// requirement names, and a reading that spelled them differently would be a
/// second vocabulary for one set of facts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Hosting {
    /// Nothing is installed; it runs only while a terminal holds it.
    #[serde(rename = "not-hosted")]
    NotHosted,
    /// Installed, and the manager confirms it is running.
    #[serde(rename = "hosted")]
    Hosted,
    /// Installed, and the manager would not say whether it is running.
    #[serde(rename = "installed-unverified")]
    InstalledUnverified,
    /// Installed, and the manager says it is not running.
    #[serde(rename = "stopped")]
    Stopped,
    /// Installed against a program that is no longer there.
    #[serde(rename = "orphaned")]
    Orphaned,
    /// This platform has no service manager lemonfiber configures.
    #[serde(rename = "unsupported")]
    Unsupported,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What this machine keeps running on lemonfiber's behalf.
///
/// Defaultable so a test can read one out of a `Result` without a branch it can
/// never take: a closure standing in for the impossible arm is a region no
/// passing run enters, and the coverage gate counts those.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HostingReport {
    /// What is true of this manager and worth knowing before it is relied on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caveat: Option<String>,
    /// What this run changed, where it was asked to change something.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed: Option<super::Changed>,
    /// Every long-running command there is, hosted or not.
    pub commands: Vec<super::HostedCommand>,
    /// What to do instead, where lemonfiber cannot configure this platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    /// The service manager this platform has, or the absence of one.
    pub manager: super::Manager,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// One household member: who they are, what they may watch, when they were last
/// seen, and everything they have asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HouseholdMember {
    /// What they may watch.
    pub access: super::MemberAccess,
    /// What they may ask for, and what their period has left of it.
    ///
    /// Absent where the request service holds no account for them, and where it could
    /// not be asked — an unread answer is not an unlimited member, and reporting one as
    /// the other would tell an operator their quota was never applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asking: Option<super::MemberAsking>,
    /// Whether somebody has set a password on the account. False is an invitation
    /// nobody has taken up rather than a member who is not here.
    pub claimed: bool,
    /// When the media server last saw them, as it timestamps it. Absent where nobody
    /// has ever signed in, which is exactly the unclaimed invitations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// The member, by the name their account is held under.
    pub name: String,
    /// What they asked for, newest first.
    pub requests: Vec<super::MemberRequest>,
    /// Where the account stands: an invitation still out or run out, a member who can
    /// sign in, or one switched off.
    pub standing: super::MemberStanding,
    /// What this member would be told, in the words they would read it in.
    ///
    /// Everything a household member is owed at the moment of asking and cannot be
    /// shown where they ask: what happens to what they ask for, what their period has
    /// left and when it makes room, what is still waiting on an answer, and what was
    /// refused and why. Written to them rather than about them, in the household's own
    /// words, so it can be handed over as it stands.
    ///
    /// Empty where there is nothing to tell them — a member the request service holds
    /// no account for has no standing to report and nothing waiting.
    pub to_hand_over: Vec<String>,
}

/// What removing somebody costs, and what it did.
///
/// Read before anything is written: the whole point of the unconfirmed run is that every
/// figure here is knowable without removing anybody.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HouseholdRemoval {
    /// Whether the request service holds an account for them at all.
    ///
    /// False where they never signed in there, which is nothing to revoke rather than a
    /// revocation that failed.
    #[serde(rename = "asks-through-the-request-service")]
    pub asks_through_the_request_service: bool,
    /// Whether it was carried out, or only described pending confirmation.
    pub confirmed: bool,
    /// What could not be done, and anything else worth the operator's attention.
    pub findings: Vec<String>,
    /// The name their account is held under, as the media server spells it rather than
    /// as the operator typed it.
    pub name: String,
    /// The name of this reading, which the removal takes as its yes.
    pub offer: String,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// How many of their requests go with them.
    ///
    /// **They are destroyed, not reassigned.** The request service removes them by hand
    /// so that a title still waiting goes back to being unrequested rather than pointing
    /// at nobody — so this is a count of things that will stop existing.
    pub requests: u64,
    /// How far it got.
    pub revoked: super::Revoked,
    /// What removing them does to their watch history, in the stack's words.
    #[serde(rename = "watch-history")]
    pub watch_history: String,
}

/// Who is in the household, what each may watch, and what each has asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HouseholdReport {
    /// What that policy allows in a period, in the words a household says it in.
    ///
    /// Absent where nothing limits the household, which is not the same as a policy
    /// that could not be read: that one leaves [`Self::policy`] absent too.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allows: Option<String>,
    /// Whether the household could be read at all. A false here is why the list is
    /// empty, and keeps an unread record from being mistaken for an empty house — the
    /// same honesty a trace keeps about a silence it did not hear.
    pub available: bool,
    /// What the limits on this household are and are not, where anybody carries one.
    ///
    /// Absent on a household nobody has limited, because there is no claim to be modest
    /// about. Present the moment there is one, because a parent who has set a limit is
    /// exactly the reader who might take it for a lock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filtering: Option<String>,
    /// What could not be read, and anything else worth the operator's attention.
    pub findings: Vec<String>,
    /// Everybody the media server holds an account for, in name order — including
    /// those who have never asked for anything, and the invitations nobody has taken
    /// up yet.
    pub members: Vec<super::HouseholdMember>,
    /// What happens to what the household asks for where nobody chose otherwise for
    /// one person. Absent where the request service could not be asked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<super::Policy>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}
