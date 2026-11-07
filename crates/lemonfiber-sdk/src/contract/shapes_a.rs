/// What an undo does.
///
/// Tagged by what it does rather than by the field it sits in, so a reader parsing
/// one branches on a word rather than on which keys are present.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "does")]
#[non_exhaustive]
pub enum Action {
    /// Remove the resource that was created.
    #[serde(rename = "remove")]
    Remove {
        /// The identifier to remove.
        id: String,
        /// The kind of resource.
        resource: String,
    },
    /// Restore a value, or remove it where there was none before (`None`).
    #[serde(rename = "restore")]
    Restore {
        /// The setting to restore.
        key: String,
        /// What to restore it to, or `None` to remove it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
        /// What lemonfiber put there, which has to still be there for putting the
        /// old value back to be putting anything back.
        ///
        /// Carried so that a reversal can ask whether it is undoing its own work.
        /// Without it a reversal knows only what it would like the setting to say,
        /// and a setting the operator has since chosen for themselves reads exactly
        /// like one nobody has touched.
        wrote: String,
    },
    /// Remove a path that was created.
    #[serde(rename = "delete")]
    Delete {
        /// The path to remove.
        path: String,
    },
    /// Take a region lemonfiber wrote back out of the file it was written into.
    ///
    /// Only where the region is still what was written. One that was edited since, or
    /// whose markers were, is somebody else's work now, and is left exactly as it is.
    #[serde(rename = "withdraw")]
    Withdraw {
        /// The same file beneath the stack directory, as the record of what lemonfiber
        /// materialised names it.
        key: String,
        /// Whose region it is, as its markers name it.
        owner: String,
        /// The file the region is in.
        path: String,
        /// The checksum of what was written between the markers, which has to still be
        /// what is there for taking it out to be taking out lemonfiber's own work.
        written: u32,
    },
    /// Write a file back to what it held before lemonfiber wrote over it.
    ///
    /// Only where it still holds what was written. One written since is somebody
    /// else's work now, and is left exactly as it is.
    #[serde(rename = "rewind")]
    Rewind {
        /// The file to write back.
        path: String,
        /// What to write back into it.
        previous: String,
        /// The checksum of what lemonfiber wrote, which has to still be what is there
        /// for writing the old text back to be undoing lemonfiber's own work.
        written: u32,
    },
    /// Write the value the secret store holds of a plugin's minted secret back into a
    /// starting file, where the shipped file puts the placeholder.
    ///
    /// Only where it still holds what was written. One written since is somebody else's
    /// work now, and is left exactly as it is.
    #[serde(rename = "reseed")]
    Reseed {
        /// The file to write back.
        path: String,
        /// The plugin the secret is of.
        plugin: String,
        /// The secret, by its id.
        secret: String,
        /// The checksum of what lemonfiber wrote, which has to still be what is there.
        written: u32,
    },
    /// Move a service's state back to where it was.
    ///
    /// Only where nothing has taken its old place, and only from where it was moved to.
    #[serde(rename = "move-back")]
    MoveBack {
        /// Where it goes back to.
        from: String,
        /// Where it is now.
        to: String,
    },
    /// Move a value in the environment file back to the key it was under.
    ///
    /// Only where the key it is under now holds a value and the one it goes back to holds
    /// none.
    #[serde(rename = "rename-back")]
    RenameBack {
        /// The key it goes back to.
        from: String,
        /// The key it is under now.
        to: String,
    },
    /// Keep the copy of a volume where it is: the volume was never changed, so nothing
    /// is put back.
    #[serde(rename = "keep-copy")]
    KeepCopy {
        /// Where the copy is.
        to: String,
        /// The volume it was copied from.
        volume: String,
    },
    /// Pin a service back to the version it was standing on.
    ///
    /// The one reversal nothing in this product carries out. Which version runs is
    /// decided by the materialised stack and by what Compose was told to start, and
    /// a reversal of settings and files reaches neither — so this is worked out,
    /// reported, and left for the operator rather than attempted.
    #[serde(rename = "repin")]
    Repin {
        /// The version this run moved it to, which has to still be the one running
        /// for putting the old one back to be putting anything back.
        current: String,
        /// The version to put back.
        previous: String,
    },
    /// Put one field of a service's own resource back to what it held.
    ///
    /// The only reversal that needs the service itself: the value lives inside it,
    /// and nothing on the host can write it. A reversal that cannot reach the
    /// service says so rather than reporting the field restored.
    #[serde(rename = "reconfigure")]
    Reconfigure {
        /// The field to put back.
        field: String,
        /// The identifier to change.
        id: String,
        /// The kind of resource.
        resource: String,
        /// What to put back, or `None` where it held nothing.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
    /// Revoke the key a mint made.
    #[serde(rename = "revoke")]
    Revoke {
        /// The key's name.
        name: String,
    },
    /// Make a revoked key good again.
    ///
    /// Worked out so the record of a revoke has a reversal to name, and never carried
    /// out: a key is revoked because something should stop holding it, and reinstating
    /// it would hand back what the revoke took away. Whatever still needs a key is
    /// minted a new one.
    #[serde(rename = "reinstate")]
    Reinstate {
        /// The key's name.
        name: String,
    },
    /// Revoke the device key a registration made.
    #[serde(rename = "revoke-device")]
    RevokeDevice {
        /// The device key's name.
        name: String,
    },
    /// Make a revoked device key good again.
    ///
    /// Worked out so the record of a revoke has a reversal to name, and never carried
    /// out, for the reason [`Action::Reinstate`] never is.
    #[serde(rename = "reinstate-device")]
    ReinstateDevice {
        /// The device key's name.
        name: String,
    },
    /// Put a setting in the settings store back to what it held.
    #[serde(rename = "resettle")]
    Resettle {
        /// The environment file's setting it goes back to, where it was moved out of one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filed: Option<String>,
        /// The setting's name.
        name: String,
        /// What lemonfiber recorded, which has to still be there.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        recorded: Option<String>,
        /// What to put back, or `None` to clear it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
    /// Give a grant back to a plugin, or end one it was given.
    #[serde(rename = "regrant")]
    Regrant {
        /// Whether to give it, rather than end it.
        give: bool,
        /// The grant, by the setting the credential is held under.
        grant: super::PluginGrant,
        /// The plugin the grant is of.
        plugin: String,
    },
    /// Put a secret in the secret store back to the value it held.
    #[serde(rename = "rehold")]
    Rehold {
        /// The fingerprint of the value to put back, or `None` to forget it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        earlier: Option<String>,
        /// The environment file's setting it goes back to, where it was moved out of one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filed: Option<String>,
        /// The fingerprint of what lemonfiber kept, which has to still be there.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kept: Option<String>,
        /// The secret's name.
        name: String,
    },
    /// Leave the media server's internal certificate as it is.
    ///
    /// Worked out so the record of a replacement has a reversal to name, and never carried
    /// out: the certificate is put back only by replacing it again.
    #[serde(rename = "unreplaced")]
    Unreplaced {
        /// The media server's service.
        service: String,
    },
    /// Leave one of lemonfiber's own guards on its plugin.
    ///
    /// Worked out so the record of the move has a reversal to name, and never carried out:
    /// the release that moved the guard moves it for good.
    #[serde(rename = "unmoved")]
    Unmoved {
        /// The stack's own service the guard's plugin took the place of.
        service: String,
    },
    /// Start a service again, or stop it again.
    #[serde(rename = "run")]
    Run {
        /// Whether it is left running.
        running: bool,
        /// The service.
        service: String,
    },
    /// Leave the operator's overlay as it is.
    ///
    /// Worked out so the record of a change to it has a reversal to name, and never
    /// carried out: lemonfiber does not write the operator's own file.
    #[serde(rename = "leave")]
    Leave {
        /// The overlay.
        path: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// One download still coming down when a reduction was asked for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Active {
    /// What it is, as the client names it.
    pub name: String,
    /// How far along, from zero to a hundred.
    pub progress: u8,
    /// Which client has it.
    pub protocol: String,
}

/// Where the front door is reached, and what is worth knowing about the address.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Address {
    /// What is worth knowing about the address itself, where anything is. Absent
    /// for one that keeps working on its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caution: Option<String>,
    /// The whole address, as it would be typed or followed.
    pub url: String,
}

/// A session opened, and the moment it stops being one.
///
/// The ending is carried rather than left for a client to work out, because a client
/// that guessed would be a second opinion about who is admitted — and the two would
/// disagree on the day somebody's clock is wrong.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Admitted {
    /// The household member this session is for, where it is a member's.
    ///
    /// **Absent is the operator**, which is the whole of the discriminator. A second
    /// field naming which kind of person this is could disagree with this one, and
    /// the day they disagreed a client would have to choose which to believe.
    ///
    /// The id and nothing else. What that member is called, what they may watch and
    /// what they have left are read from the household report, which already carries
    /// all of it per member — so there is one fact here and no second copy of
    /// anything that could go stale against the read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// The secret this session is carried by, sent in the header the per-run token is.
    pub token: String,
    /// When it stops being one, written as every other instant this product writes.
    pub until: String,
}

/// What adopting a setup already here came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdoptReport {
    /// The host paths those services keep their data in, so a backup can be taken of
    /// exactly the right thing.
    pub back_up: Vec<String>,
    /// Where the capture of those paths was written, once one has been taken.
    ///
    /// Absent on a rehearsal, which captures nothing, and absent where the setup
    /// mounted nothing worth capturing. Present on an adoption that went through,
    /// because an operator told a backup was taken is owed the path to it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backed_up: Option<String>,
    /// The project lemonfiber would manage, where exactly one could be adopted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Why nothing was done, where nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// Where the act stands.
    pub stance: super::Stance,
    /// The services whose databases a newer version would upgrade, and whose data
    /// therefore has to be backed up before anything opens it.
    pub upgrades: Vec<super::CarryingReport>,
}

/// One thing that is wrong, as the expanded summary lists it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Affected {
    /// Whether the operator has acknowledged it by declining its fix. It still stands
    /// and is still counted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    /// The check that raised it.
    pub check: String,
    /// What is also wrong because of this, counted with it rather than again.
    pub downstream: Vec<String>,
    /// How the service it is about exited, where it has and the engine said.
    ///
    /// The technical half of what happened, kept out of the summary so the plain
    /// words lead, and here for whoever wants the code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<i32>,
    /// What it costs the operator. The line expands to items an operator can act
    /// on, and an item that states only the event leaves the judgement it was
    /// supposed to save them.
    pub meaning: String,
    /// When the stack first saw it wrong since it last saw it right, in whole
    /// seconds since the epoch.
    ///
    /// The condition's own stamp, kept between runs, so every surface that reports
    /// the check names the same moment and a restart does not make an old fault new.
    pub onset: String,
    /// What to do about it, most likely first.
    pub remedies: Vec<String>,
    /// How bad it is.
    pub severity: super::ProblemSeverity,
    /// What is wrong, in one line.
    pub summary: String,
}

/// One interruption: what happened, which way, and how much it matters.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Alert {
    /// Every check this alert speaks for, the first being [`Self::check`]. More
    /// than one where the same event was grouped across several services.
    pub affected: Vec<String>,
    /// The check this came from, so an alert and its condition cannot drift apart.
    /// Where several were grouped, the first of them.
    pub check: String,
    /// How the service it is about exited, where it has and the engine said;
    /// where several were grouped, how the first of them did.
    ///
    /// The technical half of what happened, kept out of the summary so the plain
    /// words lead, and here for whoever wants the code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<i32>,
    /// What names this alert rather than this event: the same on an onset and on the
    /// resolution that ends it, and different the next time the same thing goes wrong —
    /// the check and which recurrence of it this is.
    ///
    /// So a client can close what it opened without comparing sentences. Absent on an
    /// alert recorded before alerts carried one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// What kind of event it is, shared by every instance of it.
    pub kind: String,
    /// What it costs the operator, which is the half between the event and the
    /// fix. "The tunnel dropped" and "restart the gateway" leave whoever reads
    /// them to work out for themselves whether anything leaked.
    pub meaning: String,
    /// Which way it went.
    pub moment: super::Moment,
    /// What to do about it, most likely first. An alert that says what happened
    /// and not what to do is a notification, which is a different and worse thing.
    pub remedies: Vec<String>,
    /// How much it matters. A resolution takes the severity of what resolved,
    /// because "the critical thing is over" is itself worth the attention the
    /// critical thing had.
    pub severity: super::ProblemSeverity,
    /// What happened, in the words the condition was raised with.
    pub summary: String,
}

/// What the operator will be told about, and what changing it came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlertReport {
    /// Whether this call changed the answer.
    pub changed: bool,
    /// Events set apart from the preset, quietest name first.
    pub exceptions: Vec<super::ExceptionReport>,
    /// What that preset means, in the operator's terms.
    pub means: String,
    /// The preset in force for events with no exception of their own.
    pub preset: String,
    /// Whether it only reported what it would have written.
    pub rehearsed: bool,
}

/// How one download client answered about the limits on it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "answered")]
#[non_exhaustive]
pub enum Answer {
    /// It answered, in both directions.
    #[serde(rename = "held")]
    Held {
        /// What became of the download limit.
        down: super::BandwidthHeld,
        /// Which side of the household's day it says it is on, where it keeps the
        /// hours itself.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        period: Option<super::Period>,
        /// And of the upload one.
        up: super::BandwidthHeld,
    },
    /// It did not answer, and this is what it said.
    ///
    /// Its own line rather than an absence, because a client nobody could reach is
    /// a client whose limits are unknown, and an unknown limit rendered as no
    /// limit is the report reading better than the stack is.
    #[serde(rename = "silent")]
    Silent {
        /// What went wrong, in the words of whatever refused.
        said: String,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(serde_json::Value),
}

/// The `Apis` shape.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Apis {
    /// The `Text` variant.
    Text(String),
    /// The `List` variant.
    List(Vec<String>),
    /// A value this client was not generated with, as it arrived.
    Unrecognised(serde_json::Value),
}

/// How much an operator wants to be told.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Appetite {
    /// Failures and risks. Silence means healthy. The one chosen when the operator
    /// expresses no preference.
    #[serde(rename = "problems-only")]
    ProblemsOnly,
    /// The above, plus downloads and imports that succeeded.
    #[serde(rename = "with-completions")]
    WithCompletions,
    /// The above, plus advisories such as an available update.
    #[serde(rename = "everything")]
    Everything,
}

/// One title a member's search found.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Askable {
    /// The id it is asked for by.
    pub id: String,
    /// Whether it is a series or a film.
    pub kind: super::TitleKind,
    /// Where it stands in the house.
    pub stands: super::Stands,
    /// What it is called.
    pub title: String,
    /// The year it came out, where that is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
}

/// One page of what a member's search found, under their limits.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AskableReport {
    /// Whether what they ask for waits for whoever runs the house, as one sentence.
    pub approval: String,
    /// What the member may ask for: what their period allows and has left of each kind.
    pub asking: super::MemberAsking,
    /// The kinds of title the house can fetch now, which are the kinds searched.
    pub kinds: Vec<super::TitleKind>,
    /// The member the search was made for, by the name they are known by.
    pub member: String,
    /// Where the next page begins, absent on the last.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// The words searched for, trimmed.
    pub term: String,
    /// The titles on this page, in the order they were found.
    pub titles: Vec<super::Askable>,
}

/// What asking for one title came to, or would come to under a rehearsal.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AskedReport {
    /// Whether it is a series or a film.
    pub kind: super::TitleKind,
    /// What is left of the member's allowance once it is counted.
    pub leaves: super::LeftAfter,
    /// The member it was asked for, by the name they are known by.
    pub member: String,
    /// What it came to.
    pub outcome: super::Became,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    pub rehearsed: bool,
    /// Whether the title has come out yet. One that has not is asked for, and arrives
    /// once it is out.
    pub released: bool,
    /// All of it, in one household sentence.
    pub said: String,
    /// The seasons asked for, none for a film.
    pub seasons: Vec<u32>,
    /// The id of the title asked for.
    pub title: String,
}

/// Where one member stands against what a period allows them.
///
/// Four, and the one worth having is the middle: somebody told only when they have run
/// out has been told too late to do anything but wait, which is the answer this whole
/// feature exists to avoid handing anybody.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum AskingStanding {
    /// Nothing is counted against them.
    #[serde(rename = "unlimited")]
    Unlimited,
    /// There is room, and comfortably.
    #[serde(rename = "within-quota")]
    WithinQuota,
    /// A fifth of the period's limit or less is left, and at least one of it.
    #[serde(rename = "near-quota")]
    NearQuota,
    /// Nothing is left, so the next thing they ask for is refused as they ask.
    #[serde(rename = "quota-exhausted")]
    QuotaExhausted,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Whether a pass could assess drift — whether it had the record of what lemonfiber
/// last wrote to compare against.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Assessment {
    /// The expected-state record was read, or genuinely absent as on a first seed,
    /// so each connection was judged against it.
    #[serde(rename = "assessed")]
    Assessed,
    /// The expected-state record was there but could not be read — lost. Drift could
    /// not be judged this pass, and re-baselining from the current state is offered.
    #[serde(rename = "unassessable")]
    Unassessable,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// What an operation with no bound on it is waiting for.
///
/// A case rather than a sentence, so what an operation is waiting for is a value
/// something can be asked about. The words are built from it in one place below;
/// a phrase carried here instead would be a phrase every later surface had to
/// parse back into the fact it came from.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Awaiting {
    /// Everything still coming down has finished arriving.
    #[serde(rename = "downloads")]
    Downloads,
    /// A form's images have come down the line.
    #[serde(rename = "images")]
    Images,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
