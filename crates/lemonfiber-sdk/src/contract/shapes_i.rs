/// What copying an operator's own records across came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImportReport {
    /// What was carried across.
    pub carried: Vec<super::RecordReport>,
    /// What could not be carried, and why.
    pub not_carried: Vec<super::UnsupportedReport>,
    /// The project the records were read from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Why nothing was carried, where nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Where the act stands.
    pub stance: super::Stance,
    /// What would be, where nothing has been yet.
    pub would_carry: Vec<super::RecordReport>,
}

/// An indexer credential the operator supplied, and whether it was proven.
///
/// `validated` records whether the live test passed before this was kept — an
/// operator may proceed with one that could not be proven, and a later diagnosis
/// is owed the knowledge that it went in unverified rather than treating it as
/// good.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Indexer {
    /// The API key it authenticates with.
    pub key: String,
    /// The indexer's API base URL.
    pub url: String,
    /// Whether a live test proved the key before it was kept.
    ///
    /// Defaulted when it is absent, so a surface submitting an answer states only
    /// what was entered; what a test decided is not a caller's to assert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validated: Option<bool>,
}

/// How this copy of lemonfiber got onto the machine.
///
/// Nothing known is the default. Every other answer is a claim about somebody's
/// machine, and a value that arrived by nobody filling it in has established none of
/// them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Installed {
    /// Homebrew put it here, on macOS or on Linux.
    #[serde(rename = "homebrew")]
    Homebrew,
    /// Scoop put it here.
    #[serde(rename = "scoop")]
    Scoop,
    /// winget put it here.
    #[serde(rename = "winget")]
    Winget,
    /// `cargo install` built and placed it.
    #[serde(rename = "cargo")]
    Cargo,
    /// A distribution's own package manager placed it.
    #[serde(rename = "distribution")]
    Distribution,
    /// The shell installer this project publishes wrote it.
    #[serde(rename = "installer")]
    Installer,
    /// Downloaded, unpacked or built by hand, and owned by nobody but the operator.
    #[serde(rename = "elsewhere")]
    Elsewhere,
    /// Running in a container, from an image. The binary is part of the image, so
    /// moving to another version is pulling that version's image and recreating the
    /// container on it; a binary replaced inside goes when the container does.
    #[serde(rename = "image")]
    Image,
    /// This machine would not say where the running binary is, so none of the above
    /// can be told apart. Not a failure, and not a licence to guess.
    #[serde(rename = "untellable")]
    Untellable,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// An import that stopped part-way, in the words of whatever stopped it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Interrupted {
    /// What the service calls it.
    pub name: String,
    /// What is on disk for it already, where the walk could find it.
    pub partial: u64,
    /// What the service said, verbatim.
    pub said: String,
}

/// The whole answer to a question about credentials.
///
/// Every ask answers with the inventory as it now stands, and adds what became of
/// whatever else was asked for. An operator who has just rotated something wants to
/// see the inventory that rotation produced, not a receipt they have to go and check
/// against one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inventory {
    /// Every credential granted to a third-party plugin.
    pub grants: Vec<super::CredentialGranted>,
    /// Every credential, whether or not it is present.
    pub held: Vec<super::CredentialHeld>,
    /// What keeping them in files does and does not protect against.
    pub protection: super::Protection,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// One value, where one was asked for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revealed: Option<super::Revealed>,
    /// What became of revoking a grant, where one was asked for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<super::CredentialRevoked>,
    /// What became of a rotation, where one was asked for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotated: Option<super::Rotation>,
}

/// One invitation, as it was just made.
///
/// Carries what the operator has to pass on and nothing else: a name to sign in with,
/// the address to sign in at, the link the app opens, the address that declines it, and
/// how long it stands. The address is the media server's, where a first password is set
/// in a browser; the link sets it through the core instead.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Invitation {
    /// The one address to send them.
    ///
    /// Where a *person* reaches the media server: built from what this machine is
    /// called on the network, not from either of the hosts the stack wires itself
    /// with — those resolve only on this machine or inside the stack, and an
    /// invitation carrying one sends somebody an address that cannot open.
    pub address: String,
    /// What this offer wrote on the account, where it wrote anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied: Option<super::InvitationApplied>,
    /// What is worth knowing about the address itself, where anything is.
    ///
    /// An address that is a number is one a router can hand elsewhere, so a bookmark
    /// made from it stops working with nothing here having changed. Carried on the
    /// invitation because that is the copy somebody keeps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// Where they can decline it instead, where the stack runs the decline service.
    ///
    /// A page on the household's own network that refuses this invitation and nothing
    /// else, and switches the account made for it off. Its token is in this address
    /// alone: offering the same person again mints a new one, and the old address stops
    /// declining anything. Absent on a rehearsal, for somebody already in the household,
    /// and where the stack runs no decline service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decline: Option<String>,
    /// How many hours it stands before it is withdrawn.
    ///
    /// Counted from when it was *offered*, which for an account whose password was
    /// taken off is the moment of the reset rather than when the account was made.
    /// What happens at the end depends on the account: one nobody has been in is
    /// removed, and one somebody has is switched off and kept.
    pub hours: i64,
    /// The link the companion app opens this invitation at, `lemonfiber://join` with the
    /// address, certificate fingerprint and identifier pairing names, the name to sign in
    /// as, when it lapses in seconds since the Unix epoch, and a claim token where there is
    /// an account to claim. Absent on a rehearsal, and where `unjoinable` says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    /// Whether the request service knows about the household yet.
    ///
    /// Separate from `standing`, which is about the media-server account alone. The
    /// two can disagree — an account made while the request service was unreachable
    /// is `Made` and `NotYet` — and that disagreement is the state this reports.
    pub linked: super::Linked,
    /// The name they sign in as.
    pub name: String,
    /// Whether the account was made, or only described.
    ///
    /// A rehearsal can say the whole answer without writing any of it — the name is
    /// the one asked for, the address is the stack's, and what has run out has just
    /// been read — so the only thing separating it from the real run is this.
    pub rehearsed: bool,
    /// What was found where this was going.
    pub standing: super::InvitationStanding,
    /// Accounts somebody had been in, reset and not claimed again in time, switched off
    /// on the way past rather than removed.
    ///
    /// Kept because removing one takes what they watched with it. Switched off because
    /// an account with no password that anybody may still claim is the thing a window
    /// exists to close. Offering it again, or reissuing it, switches it back on. On a
    /// rehearsal these are the ones that *would* be switched off.
    pub suspended: Vec<String>,
    /// Why there is no join link, in words every surface can show. Absent where there is
    /// one, and on a rehearsal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unjoinable: Option<String>,
    /// Invitations nobody claimed in time, removed on the way past.
    ///
    /// Reported rather than done quietly: an operator who invited somebody last
    /// week and hears nothing would otherwise have no way to learn the account is
    /// gone. On a rehearsal these are the ones that *would* be removed.
    pub withdrawn: Vec<String>,
}

/// What an invitation wrote on the account, in the household's own words.
///
/// **Said back so an absence later is explicable.** A member held to a rating who
/// cannot find half the library is either this working or a defect, and an operator
/// with nothing on record cannot tell which. So the limit, the libraries, what happened
/// to content the media server has no rating for, and whether the request service was
/// held to the same decision all travel back on the answer that applied them.
///
/// Absent where the offer set nothing at all, which is not the same as an offer that
/// set no restrictions: naming neither a library nor a limit is saying nothing about
/// access, and nothing is what gets written.
///
/// Serialised in the field names the household read uses for the same facts, rather
/// than in the invitation's own spelling: one setting named two ways across two shapes
/// is two shapes a client has to be told are about the same thing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvitationApplied {
    /// What a limit here is, and what it is not.
    ///
    /// Carried on the answer rather than left to a document, because the reader who
    /// most needs it is the parent who has just set one.
    pub filtering: String,
    /// The libraries they may open, as the operator named them. Empty is every one.
    pub libraries: Vec<String>,
    /// How far up the ratings they may watch, in the words and the certificates this
    /// media server names in the operator's own country. Absent where no limit was set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Whether the request service was held to the same decision.
    ///
    /// The same three answers a link carries, and for the same reason: what somebody
    /// may *ask for* is a second service's business, and that service can be down while
    /// the media server is not. `NotTried` here is a service with no account for them
    /// yet — nothing to hold rather than a failure to hold something.
    pub requesting: super::Linked,
    /// What becomes of content the media server has no rating for.
    ///
    /// Held back by default on somebody being narrowed, because a rating limit cannot
    /// decide about a thing that carries no rating. The cost is real and is why this is
    /// reported rather than assumed: some legitimate content becomes invisible to them.
    pub unrated: super::Unrated,
}

/// What was found where the invitation was going.
///
/// Offering somebody an account twice is a thing operators do — they forget, or the
/// first message went unanswered — and it is not a mistake to be refused. Each of
/// these is an answer, and which one it is decides what there is to say rather than
/// whether anything worked.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum InvitationStanding {
    /// The account did not exist, and was made.
    #[serde(rename = "made")]
    Made,
    /// An invitation was already out for them, and it still stands.
    ///
    /// The same account, offered again: nothing is made, and the message to send is
    /// the one that was already true.
    #[serde(rename = "waiting")]
    Waiting,
    /// They have set a password, so they are already in the house.
    ///
    /// Nothing to claim and nothing to send. An account is not made, because the one
    /// they have is theirs and a second under a nearly identical name is how a
    /// household ends up with two of somebody.
    #[serde(rename = "joined")]
    Joined,
    /// The account was theirs already, and its password has been taken off.
    ///
    /// **Told apart from [`Made`](Self::Made) because what the person needs to hear is
    /// different.** Nobody is being invited: they are already in the household, and the
    /// news is that the password they had has stopped working. It runs out on the same
    /// window an offer does, counted from the reset — and what is at stake if it does is
    /// larger, because the account being withdrawn is one they have watched on.
    #[serde(rename = "reset")]
    Reset,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// One thing a removal reaches, said to be going or said to be kept.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    /// What it occupies, where that is knowable. Absent for a container or a network,
    /// whose room is the image's rather than their own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Why it is being kept rather than removed, where it is being kept.
    ///
    /// `None` is the ordinary case: this line is going. A reason here is the whole of
    /// how an image shared with another project, or a path this run could not
    /// confirm, stays on the list without being taken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept: Option<String>,
    /// What it is called — a container name, an image reference, or a full path.
    pub name: String,
    /// Whether it holds a credential, so a report can say what destroying it destroys.
    pub secret: bool,
    /// Which of the four sorts of thing it is.
    pub sort: super::Sort,
    /// What it is, in the operator's words.
    pub what: String,
}
