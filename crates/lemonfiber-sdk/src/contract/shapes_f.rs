/// What a service published to the local network is to the people in the house.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Facing {
    /// Where asking for something begins, and where what was asked for is followed.
    /// It links onward to the library, which is what makes it a place to start
    /// rather than one more address to be sent.
    #[serde(rename = "asking")]
    Asking,
    /// The library, where what arrived is watched. A place to start only where there
    /// is nothing to ask for, because somebody sent there has no way to ask.
    #[serde(rename = "watching")]
    Watching,
    /// One kind of media and no other, reached from the library rather than instead
    /// of it.
    #[serde(rename = "shelf")]
    Shelf,
    /// An index over every service, including the ones nobody in the house should
    /// learn exist.
    #[serde(rename = "operators")]
    Operators,
    /// How the others are reached, rather than one of them.
    #[serde(rename = "carriage")]
    Carriage,
    /// Published to the household, and nothing here says what it is to them.
    #[serde(rename = "unstated")]
    Unstated,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}

/// Whether media filenames are shown as they are.
///
/// Replaced unless asked otherwise. A library's contents are not a credential, but they
/// are the one thing in a bundle that says something about the person rather than about
/// the machine, and replacing them costs a diagnostic a reader can still follow — the
/// marks keep two mentions of one file recognisable as one file.
///
/// Read from the bare flag a surface carries rather than from a name of its own,
/// because that is what both surfaces have: `--filenames` on a command line and a
/// `filenames` in a request body are one word that is there or is not. Which way
/// round it reads is decided here, once — a surface that read it the other way
/// round would put a library's contents in a file people post in public.
pub type Filenames = bool;

/// A service a closure asked for that the configuration leaves out, and why.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Filtered {
    /// The forms that asked for it, in the order the stack declares them.
    pub forms: Vec<String>,
    /// The service's identifier.
    pub id: String,
    /// What it is called in front of an operator.
    pub name: String,
    /// The provider it cannot run without.
    pub needs: super::StackProtocol,
    /// The profile it belongs to, which is what the configuration leaves out.
    pub profile: String,
}

/// One thing a check established, and how it turned out.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Finding {
    /// The family this belongs to.
    pub category: super::DoctorCategory,
    /// The check whose finding explains this one, where another does.
    ///
    /// Set after the run rather than by the check itself: a check is independent
    /// by construction and cannot see what any other found, which is a property
    /// worth keeping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caused_by: Option<String>,
    /// A stable identifier for the thing checked, such as `vpn.egress-match`.
    pub check: String,
    /// What checking it disturbs, where the read left it unverified because checking it
    /// disturbs something, and how a diagnosis is asked to run it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disruptive: Option<super::Disruptive>,
    /// When the stack first saw this check wrong since it last saw it right, in whole
    /// seconds since the epoch.
    ///
    /// Set after the run from the store of conditions, like [`Self::said`], so it is
    /// the moment the health summary names for the same check and a restart does not
    /// move it. Absent where the finding says nothing is wrong, and where the checks
    /// ran for something other than a diagnosis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onset: Option<String>,
    /// Whose check this is: one this build ships, or one a named plugin contributed.
    ///
    /// Carried rather than read off the identifier. A contributed check's id is
    /// namespaced with the plugin's, and a reader could decode that from the colon —
    /// but an origin a reader has to decode is one a reader gets wrong, and the day a
    /// bundled id grew a colon every such reader would misattribute it in silence.
    pub origin: super::ValueOrigin,
    /// What the service said for itself, lately.
    ///
    /// Carried on the finding rather than left for the operator to go and fetch,
    /// because the explanation is almost always in it: a check can say a service is
    /// not answering, and only the service can say why. Absent where the finding is
    /// not about a service, where the service is fine, or where the engine would not
    /// say — an empty section would be a promise of evidence that is not there.
    ///
    /// Set after the run, like [`Self::caused_by`], since reading a service's output
    /// is not the check's own business and a check that did it would be doing two
    /// things.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub said: Option<String>,
    /// The service this is about, where it is about one.
    ///
    /// Absent for the checks that are about the machine rather than about
    /// something running on it — the environment, the filesystem, the operator's
    /// own choices. Carried so that one service's trouble can be attributed to
    /// the service underneath it rather than counted as one more independent
    /// thing wrong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// What the stack calls that service in front of an operator.
    ///
    /// Carried beside the id rather than left for a surface to derive, because the id is
    /// a key and not a name: capitalising `qbittorrent` does not arrive at `qBittorrent`,
    /// and the stack has already written the name down. Absent where the finding is about
    /// no service, and where the stack declares no service by that id — an id standing in
    /// for a name would put the key back in front of the operator.
    ///
    /// Set after the run, like [`Self::caused_by`], from the same manifest that says
    /// which service depends on which.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// The one-line summary of what was checked.
    pub title: String,
    /// How it turned out.
    pub verdict: super::DoctorVerdict,
}

/// What a proposed change comes to on this machine.
///
/// Empty on every change that comes to nothing beyond its value, which is most of
/// them: a report full of empty lists about a timezone would teach the operator to
/// skip the one that matters.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Findings {
    /// What is still coming down, where a reduction would interrupt it.
    pub active: Vec<super::Active>,
    /// The hand-edit found in the configuration file, where one was found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited: Option<super::Edited>,
    /// What this change leaves exactly as it is, said in full — because an operator
    /// dropping a way of downloading is weighing whether they lose what they built
    /// with it.
    pub keeps: Vec<String>,
    /// The library paths the services hold, and what moving the data location does
    /// to each. Empty for every change that does not move it.
    pub library: Vec<super::LibraryPath>,
    /// What this change newly asks the operator for, in the order they meet it.
    pub opens: Vec<super::Opening>,
    /// What this change stops running, by service name.
    pub stops: Vec<String>,
}

/// The memory the stack estimates a set of services needs.
///
/// An estimate by name as well as by description, because a figure read as a
/// measurement is one an operator believes and acts on. It is the sum of what each
/// service declares; nothing here has looked at anything running.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Footprint {
    /// The sum of the estimates the services declare, in MiB.
    pub estimated_mib: u64,
    /// The services that declare no estimate, and so are not in the sum.
    pub unestimated: Vec<String>,
}

/// Something beneath the data location that the stack did not put there.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Foreign {
    /// The directory it is in, relative to the data location — or the file itself,
    /// where it sits directly in the data location.
    pub at: String,
    /// What they occupy.
    pub bytes: u64,
    /// How many files were found under it.
    pub files: u64,
}

/// One form the stack declares, as a listing shows it.
///
/// The manifest's own words rather than lemonfiber's: forms come from the stack, so a
/// stack of somebody's own names and describes them however it likes, and a listing that
/// paraphrased would be describing a different stack from the one being run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormReport {
    /// Whether it can be started alongside another form.
    ///
    /// Worth saying in the listing rather than only when a combination is refused: an
    /// operator choosing between two forms is exactly who needs to know they are a choice.
    pub composable: bool,
    /// What it is for, in one line.
    pub description: String,
    /// What to type to start it.
    pub id: String,
    /// What it is called.
    pub name: String,
}

/// Every form this stack declares.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormsReport {
    /// The forms, in the order the stack declares them.
    pub forms: Vec<super::FormReport>,
}

/// The VPN's forwarded port across a start.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Forwarded {
    /// The port the guard forwards now, where it forwards one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub now: Option<u16>,
    /// Whether every client inside the guard takes peers on it.
    pub pushed: bool,
    /// The port every client last took, where they took one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<u16>,
}

/// How much a reading can be relied on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "as")]
#[non_exhaustive]
pub enum Freshness {
    /// Read off a local disk, so it is true as of now.
    #[serde(rename = "live")]
    Live,
    /// Read across a network share, which answers with what it was last told —
    /// carrying the moment it was taken, in seconds since the epoch, so a figure
    /// nobody can refresh is at least dated.
    #[serde(rename = "as_of")]
    AsOf {
        /// The `at` field.
        at: u64,
    },
    /// A value this client was not generated with, as it arrived.
    #[serde(
        untagged,
        deserialize_with = "crate::open::unrecognised::<Freshness, _>"
    )]
    Unrecognised(serde_json::Value),
}

impl crate::open::Tags for Freshness {
    const TAG: Option<&str> = Some("as");
    const KNOWN: &[&str] = &["live", "as_of"];
}

/// A service the household can reach that is not the front door, and why it is not.
///
/// Carried rather than left out, because the decision is the useful part: an operator
/// who can see that the index over every service was considered and refused has been
/// told something, where one shown a single name has only been given an answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontDoorBeside {
    /// The address to hand somebody for this service, read from this machine at the
    /// moment of asking rather than remembered.
    ///
    /// **Carried because not-the-door is not nowhere.** A household member wanting to
    /// watch something, or to ask for something, wants the service that faces them —
    /// and which of the two happens to be the front door is an operator's
    /// arrangement, not an answer to their question. Without this a surface can hand
    /// them only whichever one the door turned out to be, and say nothing at all
    /// about the other.
    ///
    /// Absent where the stack declares no port for it, for the reason the door's own
    /// address is absent then: an address with no port on it is one a browser answers
    /// with a refusal, and the manifest is where a port is declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// Why it is not somewhere to begin.
    pub because: String,
    /// What it is to the household.
    pub facing: super::Facing,
    /// The service, by the name it shows itself under.
    pub service: String,
}

/// The household's one front door: which service it is, where it stands, and what
/// else they can reach that is not it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontDoorReport {
    /// The address to hand them, read from this machine at the moment of asking
    /// rather than remembered. Absent where there is no door, and where there is
    /// one on a machine that will say neither what it is called nor where it is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// Everything else the household can reach, and why none of it is the door.
    pub beside: Vec<super::FrontDoorBeside>,
    /// How this came to be the door: worked out from what the stack declares, named
    /// by the operator, or named by them and refused.
    ///
    /// Carried as a state rather than left to the sentence beneath it, for the reason
    /// the standing is: an operator whose setting was refused reads the sentence, and
    /// a browser, a script or a dashboard reads this.
    pub chosen: super::Chosen,
    /// What that service is to them. Absent for the same reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facing: Option<super::Facing>,
    /// What this comes to, in the words an operator would say it in — including,
    /// where there is no door, that there is none.
    pub meaning: String,
    /// The service the household begins at, by the name it shows itself under.
    /// Absent where this stack publishes nothing they could begin at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Where the front door stands.
    pub standing: super::FrontDoorStanding,
}

/// Where the household's one front door stands.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum FrontDoorStanding {
    /// The door is the request surface, and it is running.
    #[serde(rename = "established")]
    Established,
    /// This stack has no request surface, so the library is the door — there is
    /// nothing here to ask for.
    #[serde(rename = "library-only")]
    LibraryOnly,
    /// There is a door and it is not answering.
    ///
    /// The service's own state. What fixes it is starting the service.
    #[serde(rename = "unreachable")]
    Unreachable,
    /// There is a door, it is answering, and nothing here can say where another
    /// device would reach it.
    ///
    /// The other way a door is unreachable, and a different thing to do about it:
    /// the service is fine and the way to it is what is missing. Told apart from
    /// [`Standing::Unreachable`] because the two are fixed at opposite ends — one
    /// by starting a service, the other by giving this machine an address the
    /// household's devices can use — and a household member who cannot arrive
    /// needs to know which of the two they are looking at before they start
    /// blaming their own device.
    #[serde(rename = "stranded")]
    Stranded,
    /// Nothing at all is published to the household: an operator-only configuration.
    #[serde(rename = "none")]
    None,
    /// A value this client was not generated with, as it arrived.
    #[serde(untagged)]
    Unrecognised(String),
}
