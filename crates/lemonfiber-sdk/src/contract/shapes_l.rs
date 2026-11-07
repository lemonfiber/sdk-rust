/// Everything that leaves this machine: lemonfiber's own requests, those it makes on an
/// installed plugin's behalf, and the stack's.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Leaving {
    /// Every request lemonfiber makes on an installed plugin's behalf.
    pub on_behalf: Vec<super::OnBehalf>,
    /// Every request lemonfiber makes on its own account, in a fixed order.
    pub ours: Vec<super::Outbound>,
    /// The requests made by services this stack runs, attributed to them.
    pub theirs: Vec<super::Elsewhere>,
}

/// What is left of a member's allowance once an ask is counted, for each kind a limit
/// holds them to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LeftAfter {
    /// Films still to ask for, absent where nothing limits them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub films: Option<u32>,
    /// Seasons of television still to ask for, absent where nothing limits them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub television: Option<u32>,
}

/// One kind of thing the part a substitution stops holds, and the replacement does not
/// receive.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LeftBehind {
    /// The capability whose contract declares it held.
    pub capability: String,
    /// The kind, by its stable id.
    pub kind: String,
    /// The words the rehearsal names it in.
    pub said: String,
}

/// One completed download, what letting it go would cost, and what became of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Letting {
    /// What this offer names itself, so an answer to it can say which offer it
    /// answered.
    pub agreement: String,
    /// The download, in the same words the account names it in: where it stands, what
    /// it occupies, and what removing it costs.
    pub download: super::Candidate,
    /// What goes with it, carried rather than left for a surface to remember.
    pub goes: String,
    /// What became of an answered offer, and nothing where the offer is all this is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gone: Option<super::Gone>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
}

/// Where a volume stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Level {
    /// Usage cannot be determined.
    ///
    /// Ordered first so that it never wins a comparison against a level that was
    /// actually measured: a volume nobody could read must not make a stack that is
    /// visibly full read as merely unknown.
    #[serde(rename = "unknown")]
    Unknown,
    /// Comfortable headroom.
    #[serde(rename = "ample")]
    Ample,
    /// Below comfortable, not urgent.
    #[serde(rename = "advisory")]
    Advisory,
    /// Projected to exhaust within the horizon.
    #[serde(rename = "warning")]
    Warning,
    /// Nearly exhausted.
    #[serde(rename = "critical")]
    Critical,
    /// Full; acquisitions halted.
    #[serde(rename = "exhausted")]
    Exhausted,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// How the operator wants their existing and downloaded media served, where they
/// want it served at all.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Library {
    /// No media server; an existing player, or downloads only.
    #[serde(rename = "none")]
    None,
    /// The media server in a container — the portable default that works everywhere.
    #[serde(rename = "container")]
    Container,
    /// The media server on the host, for a hardware encoder the container cannot reach.
    ///
    /// Only a valid answer where the platform offers it; the wizard rejects it
    /// elsewhere rather than letting a surface record a choice that buys nothing.
    #[serde(rename = "host")]
    Host,
}

/// One library path a service files into, and what moving the data location does
/// to it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LibraryPath {
    /// Why it does or does not, in the operator's terms.
    pub because: String,
    /// Whether the library at this path survives the move.
    pub carried: bool,
    /// The host directory it would resolve to after the move, where the move can
    /// resolve it at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The path as that service holds it, which is a path inside its container.
    pub path: String,
    /// The service holding it.
    pub service: String,
}

/// What a lifecycle command did, or would have done.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LifecycleReport {
    /// The Compose subcommand that was run.
    pub action: String,
    /// The exact command, so what happened is never a matter of trust.
    pub command: Vec<String>,
    /// What those services amount to, as one word.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<super::Condition>,
    /// Why the addresses of the engine's host were not asked or not kept for a fence, where
    /// a service is fenced and they were not. Absent for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fencing: Option<String>,
    /// What starting the stack did about the VPN's forwarded port, where it did
    /// anything. Absent in the ordinary case — the client was already on it, or
    /// there is no tunnel to forward through — and a sentence where the client was
    /// moved, or could not be.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forwarding: Option<String>,
    /// Each of lemonfiber's own guards this act moved onto its plugin, or on a rehearsal
    /// would move, for good and with no yes of its own; one it could not move stays on
    /// offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// Why nothing was run, where a start declined to run anything.
    ///
    /// Absent for every ordinary command, which is what makes it readable: a
    /// lifecycle report with an empty plan and a status of nothing is a report of
    /// something that did not happen, and without this there is nowhere to say
    /// whether that was a fault or the correct answer. A start at a login declines
    /// for three reasons the operator would each act on differently — the stack was
    /// stopped on purpose, autostart was never asked for, or this machine is on its
    /// battery and nobody said to start anyway — and a run nobody is watching has to
    /// leave the reason somewhere a reader finds later.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub held: Option<String>,
    /// Each part this build carries that the forms hold and this machine has never run,
    /// offered as an install from the build, offline, before anything of the forms is
    /// written or started; answered by [`Self::offer`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installing: Option<Vec<super::UpInstallOffered>>,
    /// Each part this release installs as a plugin that the stack still runs as its own
    /// service: a move in place on offer, named by every start and taken by none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moves: Option<Vec<super::MoveOffered>>,
    /// The offer a restart answers: the services it would restart, named so that a
    /// restart carrying it back is carried out against those services or refused.
    /// Absent for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    /// Why the pass service is left out of the project a start writes, where the stream
    /// guard is filled and it is. Absent for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passing: Option<String>,
    /// What the named forms came to: the profiles, the services they hold, and
    /// what the configuration left out.
    ///
    /// The resolved plan itself rather than a copy of its parts, because it is
    /// stated to the operator before the command runs and read out of the
    /// report afterwards — two accounts of one run, and a second shape for it
    /// would be a way for them to differ.
    pub plan: super::Plan,
    /// Host ports this start wants that another Compose project on this machine
    /// already answers on, each named on both sides.
    ///
    /// Empty for every action that starts nothing, and empty on a machine running one
    /// stack. Reported rather than refused: a port somebody deliberately shares is
    /// their business, and the start goes ahead — what this changes is whether an
    /// operator meeting a bind failure knows who is holding the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_conflicts: Option<Vec<super::ConflictReport>>,
    /// Whether this was a rehearsal.
    pub rehearsed: bool,
    /// Why the group owning the render devices of the engine's host was not asked or not
    /// kept, where a service takes `render` and it was not. Absent for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rendering: Option<String>,
    /// What asking the egress guard to hold its control server to the credential its
    /// adapter was handed came to, where it was more than held at once: the guard
    /// restarted to take it up, or why it did not. Absent where it held it at once or no
    /// guard speaks its contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secured: Option<String>,
    /// What each service ended up doing, where the action waited to find out, or
    /// where a start did not complete, read once when it ended.
    ///
    /// Empty for actions that do not wait. Stopping is finished when Compose
    /// says it is, and surveying afterwards would only report the absence it
    /// was asked to produce. A start that failed is not waited on, and names every
    /// service it addressed and those they depend on as the engine had them then.
    pub services: Vec<super::Service>,
    /// Stack files the operator has edited, left as they set them rather than
    /// overwritten with lemonfiber's own. Empty in the ordinary case; a named entry
    /// warns that an upgrade would change a file they changed, and shows the diff.
    pub stack_edits: Vec<super::StackEdit>,
    /// The exit status, absent for a rehearsal or a signalled process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// Every network the engine holds for the project that the project no longer
    /// declares, by the engine's name: disconnected and taken away before a start, or
    /// named and left by a rehearsal. Empty for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swept: Option<Vec<String>>,
    /// What narrowing moved, where the command was a switch. Absent for every
    /// other action, which is what tells a reader that nothing was left running
    /// on purpose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switched: Option<super::Switched>,
    /// Each fenced service a start holds back because no fence can be written for it, in
    /// words naming it and why. Empty for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unfenced: Option<Vec<String>>,
    /// Each service taking `render` a start writes without the host's render devices, in
    /// words naming it. Empty for every other action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unrendered: Option<Vec<String>>,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this run starts, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
}

/// How much of the line something may take.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "as")]
#[non_exhaustive]
pub enum Limit {
    /// Nothing holds it back.
    #[serde(rename = "unlimited")]
    Unlimited,
    /// A proportion of what the line was measured to carry, in whole per cent.
    #[serde(rename = "share")]
    Share {
        /// The `at` field.
        at: u8,
    },
    /// A figure in bytes a second, as it was given.
    #[serde(rename = "absolute")]
    Absolute {
        /// The `at` field.
        at: u64,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged, deserialize_with = "crate::open::unrecognised::<Limit, _>")]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Limit {
    const TAG: Option<&str> = Some("as");
    const KNOWN: &[&str] = &["unlimited", "share", "absolute"];
}

/// One narrated line: a step, and what was specifically true of it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Line {
    /// What was specifically true — the evidence that makes the line worth reading
    /// rather than a spinner. Empty where there is nothing particular to say.
    pub detail: String,
    /// What it is doing, in plain language.
    pub said: String,
    /// The step being narrated.
    pub step: super::WalkthroughStep,
}

/// What the import did with the finished download — the difference between one copy of a
/// file and two.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Link {
    /// The library entry and the download are the same file under two names, which is
    /// what a correctly-shared volume allows and what costs no extra disk.
    #[serde(rename = "hardlinked")]
    Hardlinked,
    /// The file was copied, so it now exists twice. Works, but every import costs its own
    /// size again and seeding a torrent holds the second copy for as long as it seeds.
    #[serde(rename = "copied")]
    Copied,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Whether the request service has been given an account for the household yet.
///
/// The account somebody watches with is made on the media server, and it stands on its
/// own from that moment — nothing has to be running for them to claim it. Being able
/// to *ask* for something is a second account, on a second service, and that service
/// can be down while the first is not.
///
/// So this is reported rather than made a condition of the invitation: an operator who
/// invites somebody during an outage has still invited them, and what they cannot do
/// yet is worth one line rather than a refusal.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Linked {
    /// It holds an account for everybody the media server does.
    #[serde(rename = "made")]
    Made,
    /// It could not be told. The media-server account stands, and the next run tells
    /// it — the service skips anybody it already knows, so nothing has to be
    /// remembered in between.
    #[serde(rename = "not-yet")]
    NotYet,
    /// Nothing was tried: a rehearsal, or a stack with no request service at all.
    ///
    /// Not a failure either way, and nothing a later run would put right — which is
    /// what separates it from `NotYet`.
    #[serde(rename = "not-tried")]
    NotTried,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What an existing layout costs, where it cannot hold a hardlink.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkingReport {
    /// Why they cannot, naming the filesystems it is about.
    pub because: String,
    /// What that costs, in room rather than in adjectives.
    pub cost: String,
    /// The filesystems the existing setup keeps its data on.
    pub filesystems: Vec<String>,
    /// Whether lemonfiber will do it. Always false: the layout and the library in it
    /// are the operator's, and correctness does not outrank their data.
    pub forced: bool,
    /// Whether imports can be hardlinks across this layout. False whenever this is
    /// reported at all, since a layout that links is not reported.
    pub links: bool,
    /// What would fix it, offered.
    pub remedy: String,
}

/// One device key as the listing shows it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListedDeviceKey {
    /// The app that made it.
    pub app: String,
    /// Where it lives, as its attestation said.
    pub backing: super::DeviceKeyBacking,
    /// The SHA-256 of its public half's `SubjectPublicKeyInfo`, in lower-case hex.
    pub fingerprint: String,
    /// The name it was registered under.
    pub name: String,
    /// When it was registered.
    pub registered: String,
    /// When it was revoked, where it has been.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<String>,
    /// When it last proved a mint, where it has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<String>,
}

/// One key as the listing shows it, without its secret.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListedKey {
    /// Whether a household member minted it for themselves.
    pub member_minted: bool,
    /// When it was minted.
    pub minted: String,
    /// The name it was minted under.
    pub name: String,
    /// The device key whose signature proved its mint, absent where none did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proved_by: Option<String>,
    /// What the minter said it is for. A declaration, not something the core verified.
    pub purpose: super::KeyPurpose,
    /// When it was revoked, where it has been.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<String>,
    /// What it admits: `read`, `act` or `member:<name>`.
    pub scope: String,
    /// Where it stands.
    pub state: super::KeyState,
    /// When it was last admitted, where it has been.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<String>,
}

/// The archives this machine has kept.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Listing {
    /// Each one by the name it was written under, newest first.
    ///
    /// The name is the whole of what another surface needs: it is what a restore
    /// asks for, and it carries the moment the archive was taken and what it
    /// covers, because that is how a capture names one.
    pub archives: Vec<String>,
}

/// How bad a line says it is.
///
/// Ordered, so a filter can ask for "warnings and worse" without a table of which
/// level outranks which. Deliberately coarse: these six are what services agree
/// on, and a seventh that only one of them writes would be a level nobody could filter
/// by across the stack.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum LogLevel {
    /// Detail a developer asked for.
    #[serde(rename = "trace")]
    Trace,
    /// Detail beyond the ordinary account of what happened.
    #[serde(rename = "debug")]
    Debug,
    /// The ordinary account of what happened.
    #[serde(rename = "info")]
    Info,
    /// Something the operator may want to know about, that did not stop the service.
    #[serde(rename = "warn")]
    Warn,
    /// Something that failed.
    #[serde(rename = "error")]
    Error,
    /// Something that failed and stopped the service.
    #[serde(rename = "fatal")]
    Fatal,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One line of output from one service, and how bad it says it is.
///
/// What a machine-readable surface hands on, rather than the engine's line alone, so
/// a consumer can mark the lines that say they failed, or find the first of them,
/// without reading the text for itself — a second reading of severity would be a
/// second answer, and the two would disagree about some service's spelling.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LogLine {
    /// When the container itself says it wrote the line, where it said so.
    ///
    /// Kept verbatim and unparsed. Containers disagree with the host clock and
    /// with each other, and the only defensible ordering is each container's own
    /// account of itself — which a reader can only apply if it is carried
    /// rather than replaced by an arrival time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// How bad the line says it is, in one lowercase word.
    ///
    /// Absent where the line says nothing about itself. It is never guessed from
    /// the stream the line arrived on or from the words in it: most of this stack
    /// writes ordinary progress to standard error, and a line saying it could not
    /// find something is often a routine miss.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<super::LogLevel>,
    /// The line, without its trailing newline.
    pub line: String,
    /// The Compose service it came from.
    pub service: String,
    /// Which stream it arrived on.
    pub stream: super::Stream,
}
