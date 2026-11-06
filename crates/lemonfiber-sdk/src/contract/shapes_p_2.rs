/// What an install came to, and what it took to get there.
///
/// **The three lists below are stated whether the run wrote anything or not, and that
/// is the whole of what makes a rehearsal worth running.** A rehearsal that reported
/// less than the real run would be a preview of a different operation; one that
/// reported it from code of its own would be a second derivation free to disagree
/// with the one that acts. So they are filled in one place, from the manifest and from
/// the very list of writes the install is carried out from, and the surface says them
/// in whichever tense `recorded` calls for.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstall {
    /// What those verdicts were reached against, or nothing where none were reached.
    ///
    /// Carried rather than assumed, because the two kinds of evidence are not the
    /// same claim: an author's read asks the recordings a plugin ships, and an
    /// install asks the service running on this machine. The weaker must not be
    /// readable as the stronger, and a reader handed a verdict has nothing else in
    /// the document to tell them apart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub against: Option<super::PluginEvidence>,
    /// Every ask its services would make, each with what it would reach and how that
    /// would be settled.
    pub asks: Vec<super::Wired>,
    /// Every change it makes to the machine, in the order it makes them.
    pub changes: Vec<super::PluginChange>,
    /// Every ask of the stack's the install leaves contested that is not contested
    /// now, as it would then stand.
    ///
    /// A plugin's service that claims what the stack asks for is a candidate like any
    /// other, so installing it leaves the ask refused until somebody chooses — which is
    /// a change to what the stack does, and stated with the rest before it happens.
    pub contests: Vec<super::WiringContest>,
    /// Every credential of another service that would cross to the plugin, each approved
    /// apart from the offer as its `approval` names it.
    pub granting: Vec<super::PluginGranting>,
    /// Every bundled thing the plugin declares it will change.
    ///
    /// The full extent rather than a sample of it: a manifest may change a bundled
    /// setting only through a recipe, and a recipe reaching one no `[[override]]`
    /// names is refused before anything is written.
    pub overrides: Vec<super::PluginOverriding>,
    /// Every proof that has to hold before the plugin is installed, and on a run
    /// that asked them, what each came to.
    pub proofs: Vec<super::PluginProving>,
    /// Every install recipe the act ran, in order, with what each step came to; an
    /// empty list where none ran, which is every rehearsal and every plugin declaring no
    /// install recipe.
    ///
    /// A recipe that did not hold ends the act with a problem carrying the same account,
    /// so this is the account of recipes that held.
    pub recipes_ran: Vec<super::PluginRecipeRan>,
    /// Whether it was written down. A rehearsal leaves this false.
    pub recorded: bool,
    /// What putting the install back came to, where something failed and it was.
    ///
    /// The rollback layer's own report rather than a shape of this verb's: what went
    /// back, and what did not with the reason each is still standing. Absent on a run
    /// that had nothing to put back, which is both a rehearsal and an install that
    /// held.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reversed: Option<super::UndoReversal>,
    /// What asking the egress guard among its services to hold its control server to the
    /// credential its adapter was handed came to, where it was more than held at once:
    /// the guard restarted to take it up, or why it did not. Absent where none of its
    /// services is a guard speaking its contract, and on every rehearsal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secured: Option<String>,
    /// Every adapter that writes the files of the service it fronts.
    pub sharing: Vec<super::PluginSharing>,
    /// Every service taking a privileged shape, with what it is given, each approved
    /// apart from the offer.
    pub taking: Vec<super::PluginTaking>,
    /// What the stack's own checks made of the install, or nothing on a run that
    /// asked them nothing.
    ///
    /// The other half of what an install has to establish, and the half a plugin
    /// cannot establish for itself: its proofs say the plugin works, and this says the
    /// stack still does. Absent on a rehearsal, which writes nothing and so has
    /// nothing to hold a reading against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<super::PluginVerification>,
    /// The services held back once the install is made, each with what it waits for.
    pub waiting: Vec<super::Waiting>,
    /// What the install settled, said whether or not it was written down.
    pub would: super::PluginInstalled,
}

/// One plugin's install, as it was decided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstalled {
    /// Every row it adds to a register lemonfiber already runs, as the install settled
    /// them, so the doctor reads them off this record rather than off the plugin's files.
    /// Defaulted for a register written before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributions: Option<Vec<super::Contribution>>,
    /// What the plugin declared about itself: where it is published, whether it was
    /// reviewed, what it claims, what it may change, where it may reach and what it
    /// will hold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declared: Option<super::PluginDeclaration>,
    /// What it does for the operator, in its author's words. Absent alike.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The source it was installed from, as the operator named it.
    ///
    /// Empty for a record written before this was kept. A rehearsal's account carries
    /// the source it was asked about, because that is what it would record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Each credential of another service the operator granted it, as a third-party plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted: Option<Vec<super::PluginGrant>>,
    /// When it was installed, as the record stamps every change: whole seconds since
    /// the epoch.
    ///
    /// The install's own stamp, the one its changes are journalled under, so the
    /// listing and the history name the same moment. Empty where the record predates
    /// it; a rehearsal's account carries the moment it was asked, which is the stamp
    /// the install would have run under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<String>,
    /// The SHA-256 of the manifest it was installed from, in lower-case hexadecimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    /// What the plugin calls itself, for a person to read.
    ///
    /// Absent on a record written before it was kept, and never filled in from the id:
    /// a name is the author's, and one made up here would be lemonfiber's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plugin's id: the name it is installed and journalled under.
    pub plugin: String,
    /// Every core capability its services fill, as the install settled them, core names
    /// only, which is what a removal reads to say what would go unfilled. Defaulted for a
    /// register written before the field existed, which reads as filling nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    /// Every recipe it declares: each call in order with the adapter it reaches through,
    /// and every value that could leave for somewhere else.
    ///
    /// Empty for a plugin that declares none and for a record written before these were
    /// kept, so a list is always there to read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<super::PluginRecipe>>,
    /// Every proof it declares that asks one of its services a question.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_proofs: Option<Vec<super::PluginRequestProof>>,
    /// The commit it was installed at, where it came from a git source.
    ///
    /// The one commit the revision named at install resolved to, so what was installed
    /// can be told from whatever that source serves now. Empty for a plugin installed
    /// from a directory, which has no revision to name, and for a record written before
    /// this was kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// Every proof it declares that runs the image's own program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_proofs: Option<Vec<super::PluginRunProof>>,
    /// What was placed, one entry per service the plugin declares.
    pub services: Vec<super::PluginPlaced>,
    /// What signed it: the key the catalogue index it was resolved through verified
    /// against, named with its fingerprint; empty where nothing signed it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed: Option<String>,
    /// The digest of the plugin's tree it was installed from, `sha256:<hex>`; empty
    /// where the tree could not be read whole, or the record predates it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<String>,
    /// The plugin's own content version, as it stood when it was installed.
    pub version: String,
}

/// What is installed, and what installing one came to.
///
/// One answer for the reading and for the verb, because they are one question: an
/// operator who has just installed something wants to see it among what they had, and
/// a rehearsal that showed only the new entry would not say what it is joining.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginInstalls {
    /// What this run's reading names itself, so an answer to it can say which reading
    /// it answered; nothing on the reading of what is installed, which offers nothing.
    ///
    /// Named part by part, so an answer refused because something moved is told which
    /// part did.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement: Option<String>,
    /// What asking for one of a plugin's demand recipes came to, or nothing where none
    /// was asked for.
    ///
    /// Boxed for the reason the install is: it carries a whole recipe, and every other
    /// run's report would otherwise be as large as the one run that asks for one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub demand: Option<super::PluginDemand>,
    /// Each of lemonfiber's own guards this run moved onto its plugin after its own
    /// change, or on a reading would move, for good and with no yes of its own; one it
    /// could not move stays on offer and says why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guards: Option<Vec<super::StackMoveTaken>>,
    /// What this run's install came to, or nothing where it only read.
    ///
    /// Boxed for the reason the update is: it carries a whole account, and every other
    /// run's report would otherwise be as large as the one run that installs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install: Option<super::PluginInstall>,
    /// Every plugin the record holds.
    ///
    /// What it holds, rather than what it would hold: a rehearsal wrote nothing, so
    /// what it settled is in `install` and not here. A listing that counted it
    /// would report an install that did not happen.
    pub installed: Vec<super::PluginInstalled>,
    /// Every answer an installed plugin's adapter gave outside its contract, kept until a
    /// proof it passes clears it: the plugin fills none of those capabilities meanwhile.
    ///
    /// Filled on the reading of what is installed, as `substituted` is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonconforming: Option<Vec<super::PluginNonconforming>>,
    /// What proving a plugin again came to, or nothing where nothing was proved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<super::PluginReproof>,
    /// Whether this was a rehearsal: what would have happened, with none of it done.
    ///
    /// Said in a field of its own so that a rehearsal is never told from the real run by
    /// its wording alone.
    pub rehearsed: bool,
    /// What this run's removal came to, or nothing where it removed nothing.
    ///
    /// Beside the install rather than in place of it, and never both at once: an
    /// install and a removal are two verbs with two accounts, and a field that held
    /// whichever happened would make a reader ask which one this was before they could
    /// read it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal: Option<super::PluginRemoval>,
    /// Whether each installed plugin's source can still be fetched, asked now.
    ///
    /// Filled on the reading of what is installed and nowhere else, for the reason
    /// `substituted` is: it is the one read an operator makes of what each plugin is
    /// doing, and the one moment this machine asks anybody where a plugin came from. A
    /// run that installs, updates or removes one leaves it empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<super::PluginSource>>,
    /// Every capability the operator chose an installed plugin's service to fill.
    ///
    /// Filled on the reading of what is installed, which is the one read of what each
    /// plugin is doing; a run that installs, updates or removes one leaves it empty,
    /// because none of them changes a choice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substituted: Option<Vec<super::PluginSubstituted>>,
    /// The operator's overlay and every service it changes, where there is one: merged
    /// over the project this run would start, and held to nothing lemonfiber vets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unvetted: Option<super::Unvetted>,
    /// What this run's update came to, or nothing where it updated nothing.
    ///
    /// A third field rather than an install and a removal filled in together, for the
    /// reason those two are apart: an update is one operation with one account.
    ///
    /// Boxed because it carries a whole install's account beside the reversal, and
    /// every other run's report would otherwise be as large as the one run that updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<super::PluginUpdate>,
}

/// One answer a plugin's adapter gave outside a contract it speaks.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginNonconforming {
    /// When it answered so, as RFC 3339.
    pub at: String,
    /// The capability it was asked as.
    pub capability: String,
    /// The operation it was asked.
    pub operation: String,
    /// The plugin whose adapter answered.
    pub plugin: String,
    /// What was outside the contract.
    pub why: String,
}

/// One bundled thing a plugin declares it will change.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginOverriding {
    /// Which bundled setting it changes.
    pub setting: String,
    /// What changing it is for.
    pub why: String,
}

/// One value a recipe could carry to one destination, as the operator agrees to it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPair {
    /// What approving this pair is written as, on the command line and over the web,
    /// where it carries the value to a host outside the stack or carries a release.
    /// Absent on any other pair to a service in this stack, which takes nothing off the
    /// machine or away from the service it came from, and asks for no approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<String>,
    /// The service in this stack the released value was read from, where the pair
    /// releases it. Absent on every other pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Whose value it is, as its input or the step that captures it says; empty where
    /// neither does.
    pub origin: String,
    /// Why the value is carried away from the service it was read from, in the manifest's
    /// own sentence, where the pair releases it. Absent on every other pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    /// Where it may be carried, by the name the manifest gives it.
    pub to: String,
    /// What the value is called within the recipe.
    pub value: String,
}

/// One service of an installed plugin, as it was placed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPlaced {
    /// Every capability it asks for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<super::PluginAsking>>,
    /// What it asks for there, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asks_for: Option<String>,
    /// Where inside the container its one configuration directory is mounted.
    ///
    /// Resolved rather than optional. The record answers where the directory is, and
    /// a run that re-derived the fallback would answer for a container it did not
    /// write the day that fallback moved.
    pub config_path: String,
    /// The port inside its container it speaks its contracts on, where it declared one
    /// apart from where it listens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<u16>,
    /// The line its dashboard entry shows beside it: the one its wiring names, or what
    /// the plugin says it does for the operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The digest that fixes what runs.
    pub digest: String,
    /// Its container fields beyond those above.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<super::ServiceFields>,
    /// The service of the same plugin it stands in front of, as an adapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fronts: Option<String>,
    /// The path it answers over HTTP to say it is up, where its health is asked that way.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// Whether its wiring hid it from the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// The icon its dashboard entry shows, where its wiring names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The registry path, carrying no pin of its own.
    pub image: String,
    /// The port it answers on inside the stack's network, where it declared one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listens: Option<u16>,
    /// The media it files, in the stack manifest's vocabulary, which decides what it
    /// comes to in each service that asks for what it provides; none in an older record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_types: Option<Vec<String>>,
    /// What it is called, for a reader, which is what its dashboard entry is listed as;
    /// a record written before this was kept lists it by its id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The API it answers the stack's other services in, where it names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    /// Networks an older record named for it. Nothing reads them: each service's place on
    /// the networks is derived with the project, and a record written now holds none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    /// Every core capability this one service fills, which is what makes it a candidate
    /// when the stack asks for one.
    ///
    /// Per service rather than read off the plugin's whole list, because a wiring
    /// reaches a service and not a plugin: of a plugin's two services, the one that
    /// fills a capability is the one an ask for it would reach. Defaulted for a record
    /// written before this was kept, which reads as filling nothing — a service nothing
    /// is wired to, rather than one wired to on a guess.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    /// How it is reached, or nothing where it has no listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reached: Option<super::PluginReached>,
    /// Where it reaches outside the machine, where it says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reaches: Option<String>,
    /// The stack's own service it moves in place of, where it names one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaces: Option<String>,
    /// The service's id, which is the name its container is written under.
    pub service: String,
    /// The privileged shape lemonfiber writes for it, where it took one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<super::PluginShape>,
    /// Each capability contract it answers as an adapter, as `capability@major`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaks: Option<Vec<String>>,
    /// The digest of the starting files kept for its run proofs, where it has any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting: Option<String>,
    /// The readable name that digest went by when it was installed.
    pub tag: String,
    /// Whether the library is mounted for it.
    pub takes_data: bool,
    /// Each field of the `upstream.json` lemonfiber writes for it as an adapter, to the
    /// id of the plugin's secret that field holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<std::collections::BTreeMap<String, String>>,
}

/// One placeholder a starting file names a secret by, as the plugin ships it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginPlaceholder {
    /// The form its value is written in.
    pub form: super::PluginPlaceholderForm,
    /// The secret it names, by its id.
    pub secret: String,
}

/// The form a placeholder asks its secret to be written in.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginPlaceholderForm {
    /// The value itself.
    #[serde(rename = "plain")]
    Plain,
    /// PBKDF2-HMAC-SHA512 of the value under a fresh salt, as a service keeping only a
    /// hash of its password reads it.
    #[serde(rename = "pbkdf2")]
    Pbkdf2,
}

/// One proof that has to hold before a plugin is reported installed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginProving {
    /// What it asks, as the method and the path it is asked at.
    pub asks: String,
    /// What asking it came to, or nothing where it was not asked.
    ///
    /// Absent on a rehearsal, which asks nothing. That is a different fact from a
    /// proof that was asked and established nothing, and the two must not read alike:
    /// one is an account of what would happen, the other is a service that did not
    /// answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub came_to: Option<super::PluginVerdict>,
    /// What it establishes, in one line.
    pub establishes: String,
    /// Which of the plugin's services it asks, where the manifest settles that.
    ///
    /// Nothing where it does not, which is a manifest the reader has already refused
    /// — carried as an absence rather than as a guess, so that a report built from a
    /// manifest nobody held to the reader says *this was not settled* instead of
    /// naming whichever service came first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub of: Option<String>,
    /// The proof's id, which its verdict is reported against.
    pub proof: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// What an install puts at one path.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginPuts {
    /// A directory brought into being, which is where one service keeps its own
    /// configuration.
    #[serde(rename = "directory")]
    Directory,
    /// The project written again, holding the container lemonfiber derives for the
    /// plugin.
    #[serde(rename = "document")]
    Document,
    /// The key lemonfiber asks one of the plugin's adapters with, minted as it is
    /// written and never shown.
    #[serde(rename = "key")]
    Key,
    /// A service's own secrets file, readable by its owner alone, its values never shown.
    #[serde(rename = "secrets")]
    Secrets,
    /// A file the plugin ships for one of its services, written where none is yet; one
    /// naming a secret is readable by its owner alone and shown with its placeholders.
    #[serde(rename = "starting")]
    Starting,
    /// The credentials an adapter reads its upstream's with, readable by its owner alone,
    /// their values never shown.
    #[serde(rename = "upstream")]
    Upstream,
}

/// How an installed service is reached, where it is reached at all.
///
/// The tier is the arm, so the label a tier earns lives only in the arm entitled to
/// one. Only a household service is proxied — the bundled policy is that an admin
/// surface does not get a name on the household network — and a record able to carry
/// a loopback service with a hostname would be a record able to describe the thing
/// that policy exists to prevent.
///
/// **The group is on both arms, and that is not an oversight.** Only the proxy is
/// the household tier's alone; the bundled dashboard carries an entry for an
/// operator surface too, with the address it links to rendered from the tier — nine
/// of the shipped stack's own entries point at this machine. A record that kept the
/// group for the wider tier alone would leave a loopback service off the panel its
/// bundled neighbours are on.
///
/// A tier and never an address, either way: lemonfiber renders one from the other
/// exactly as it does for a bundled service, so the two-tier policy stays a property
/// of the system rather than a request a plugin made.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "tier")]
pub enum PluginReached {
    /// From this machine and nowhere else. No route, and no label to route to.
    #[serde(rename = "loopback")]
    Loopback {
        /// The group on the bundled dashboard, where the manifest named one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        /// The port the service listens on.
        port: u16,
    },
    /// From the household, through the stack's own proxy, at this label.
    #[serde(rename = "household")]
    Household {
        /// The group on the bundled dashboard, where the manifest named one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        group: Option<String>,
        /// The single label in front of the operator's domain.
        hostname: String,
        /// The port the service listens on.
        port: u16,
    },
}

/// One recipe, as an operator agrees to what it does.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipe {
    /// The recipe's id within the plugin.
    pub id: String,
    /// Every value it asks the operator for when it runs, in the order it declares them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<super::PluginRecipeInput>>,
    /// When it runs: as part of an install and an update, or only when asked for by name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on: Option<super::PluginRecipeOn>,
    /// Every value it could carry, and where to.
    pub pairs: Vec<super::PluginPair>,
    /// Every call, in the order the recipe makes them.
    pub steps: Vec<super::PluginStep>,
    /// What it accomplishes, in one line.
    pub title: String,
    /// Why it is worth running.
    pub why: String,
}

/// One value a recipe asks the operator for when it runs.
///
/// Never the value: a reading says what will be asked, and what is given for it is
/// handed to the recipe and said back by nothing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipeInput {
    /// What the operator is asked for it.
    pub ask: String,
    /// What the input is called, which is what a value is given for it under.
    pub name: String,
    /// Whether the value may be a secret, so a surface takes it without showing it.
    pub secret: bool,
}

/// When a recipe runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginRecipeOn {
    /// During an install and an update, after the plugin's proofs and the stack's checks
    /// hold and before the install is recorded. What a recipe that says nothing does,
    /// because a first-run flow is what a recipe is for.
    #[serde(rename = "install")]
    Install,
    /// Only when the operator asks for it by name, with approvals of its own.
    #[serde(rename = "demand")]
    Demand,
}

/// What one recipe came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRecipeRan {
    /// Whether every step it made came to what it should.
    pub held: bool,
    /// The recipe's id.
    pub recipe: String,
    /// Every step it declares, in order, with what each came to.
    pub steps: Vec<super::PluginStepRan>,
    /// Why it did not, where it did not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// What taking a plugin off the machine came to, or would come to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRemoval {
    /// Every service that stops when it goes, named before any of them does.
    ///
    /// By service rather than by plugin, because a service is what an operator notices
    /// stopping: a plugin that brought two containers takes two things away, and the
    /// plugin's name alone would not say which of the addresses they use goes quiet.
    /// None of them comes back — a removal is not a restart — which is why this is
    /// stated before the run rather than discovered after it.
    pub interrupts: Vec<String>,
    /// Every capability that would have nothing filling it afterwards.
    ///
    /// Stated before it happens rather than reported after, which is the requirement
    /// and also the only useful order: an operator told afterwards that their requests
    /// no longer reach anything has been informed rather than asked.
    pub leaves: Vec<super::PluginUnfilled>,
    /// The plugin this is about.
    pub plugin: String,
    /// Whether the record of what is installed was written without it.
    ///
    /// False on a rehearsal and on a run that got as far as putting the files back and
    /// no further, which are two different machines and are told apart by what the
    /// reversal says rather than by a second flag here.
    pub removed: bool,
    /// What putting its changes back came to, or would come to.
    ///
    /// The rollback layer's own report rather than a shape of this verb's: a removal is
    /// a reversal with a name on it, and an account of its own would be a second
    /// description of the same work.
    pub went_back: super::UndoReversal,
}

/// What asking an installed plugin's adapters again came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginReproof {
    /// Whether its adapters were asked, rather than only said to be.
    pub asked: bool,
    /// Whether every answer kept against the plugin was cleared, which a proof that did
    /// not hold never does.
    pub cleared: bool,
    /// Every answer kept against the plugin when it was proved, which a pass clears.
    pub kept: Vec<super::PluginNonconforming>,
    /// The plugin proved.
    pub plugin: String,
    /// Each proof asked, or that would be asked, with what it came to.
    pub proofs: Vec<super::PluginProving>,
}

/// What is asked, and where.
///
/// Three fields and no more, and the third is the one worth explaining. A service that
/// answers XML unless a caller asks for JSON cannot satisfy a capability whose probe
/// requires a JSON assertion, and until this field there was nowhere to ask: Plex
/// answers `text/xml` at every path, including the one its health probe uses, unless
/// the request carries `Accept: application/json`.
///
/// **It is one media type and not a header map, and the difference is the point.** A
/// probe declares who it is asked as, and `none` on every `guarded` probe has meant what
/// it says partly because nothing could be presented. A map of headers would make that a
/// convention a reviewer has to hold — any service may name its credential header
/// whatever it likes, so no list of refused names could ever be closed — where one named
/// field keeps it a property of the format. A probe still cannot present anything,
/// because there is nowhere to write it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRequest {
    /// The one representation the answer is asked for, as a media type.
    ///
    /// Absent where the service needs no asking, which is most of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// The HTTP method.
    pub method: String,
    /// The path on the service being asked.
    pub path: String,
    /// Each place in the answer whose value the service makes afresh on every run, a JSON
    /// Pointer into its JSON, to the RE2 shape the whole value there is held to wherever a
    /// recording is taken again. Never sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub varies: Option<std::collections::BTreeMap<String, String>>,
}

/// One proof that asks one of the plugin's services a question, kept so the doctor can ask
/// it again without the plugin's source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRequestProof {
    /// What the answer must be.
    pub expect: super::Expect,
    /// The proof's id.
    pub proof: String,
    /// What it asks.
    pub request: super::PluginRequest,
    /// The service it asks.
    pub service: String,
    /// What it establishes, in one line.
    pub title: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// One starting file whose placeholders an update changes, as the version installed
/// shipped it and as the one coming ships it: the placeholders alone, never what the
/// file on the machine holds.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginReshipped {
    /// Its placeholders as the version coming ships it, none where it ships none.
    pub becomes: Vec<super::PluginPlaceholder>,
    /// Where the file is, in full.
    pub path: String,
    /// Its placeholders as the version installed shipped it, none where it shipped none.
    pub was: Vec<super::PluginPlaceholder>,
}

/// Where an update did not hold: what putting the version it replaced back came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRestored {
    /// Whether everything its record says it placed is on the machine again.
    pub placed: bool,
    /// Whether its containers are running again.
    ///
    /// Apart from `placed`, because the two fail differently: a document that would not
    /// land is a disk, and a container that would not start is the engine — and an
    /// operator fixes them in different places.
    pub running: bool,
    /// The version put back, which is the one the record still names.
    pub version: String,
}

/// A run of the image's own program, as a proof declares it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRun {
    /// Each argument handed to the image's entrypoint, one string each: one to 64, each at
    /// most 4096 bytes and none holding a NUL.
    pub args: Vec<String>,
    /// How long the run may take, from 1 to 120 seconds. Default 30.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<u32>,
}

/// One proof that runs the image's own program, kept so the doctor and a later proof can
/// run it again without the plugin's source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginRunProof {
    /// What the run must come to.
    pub expect: super::Expect,
    /// The proof's id.
    pub proof: String,
    /// What the image's own program is handed.
    pub run: super::PluginRun,
    /// The service whose image it runs.
    pub service: String,
    /// What it establishes, in one line.
    pub title: String,
    /// Why it is worth asserting.
    pub why: String,
}

/// One credential a plugin says it will hold, without a value and with no place for one.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSecret {
    /// Whether a recipe of the plugin's captures the value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured: Option<bool>,
    /// The capability of the plugin's own that a credential crossing to it is needed to fill.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#for: Option<String>,
    /// What the value is, within the plugin.
    pub id: String,
    /// How many seconds a code the operator enters to claim a part holds, where it is
    /// one: asked for only to claim the part, and held no longer than the claim or this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lapses_s: Option<u32>,
    /// Whether lemonfiber made the value at install.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minted: Option<bool>,
    /// Whether only the operator holds the value and the service reading it needs one to
    /// start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needed_to_start: Option<bool>,
    /// The values of the plugin's settings under which it is asked for and written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs: Option<std::collections::BTreeMap<String, Vec<super::ServiceConstant>>>,
    /// Whose credential it is.
    pub of: String,
    /// The variable an earlier release kept its value under in the stack's environment
    /// file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<String>,
    /// What holding it is for.
    pub why: String,
}

/// A privileged shape lemonfiber writes for a service.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PluginShape {
    /// `NET_ADMIN` and the tunnel device.
    #[serde(rename = "egress-guard")]
    EgressGuard,
    /// The host's render devices and the group that owns them.
    #[serde(rename = "render")]
    Render,
}

/// An adapter that writes the files of the service it fronts, through that service's
/// whole configuration directory mounted into it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSharing {
    /// The adapter.
    pub adapter: String,
    /// Where that service's configuration directory lands inside the adapter.
    pub at: String,
    /// The service whose files it writes.
    pub upstream: String,
}

/// Whether one installed plugin's source can still be fetched.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginSource {
    /// The source the record says it came from.
    pub from: String,
    /// The plugin, by its id.
    pub plugin: String,
    /// What asking it came to.
    pub standing: super::PluginSourceStanding,
}

/// What asking a plugin's source came to.
///
/// **Unreachable is not untrusted.** A plugin whose source has gone keeps running as it
/// was installed; what it loses is the way to a newer version, and that is said now
/// rather than found out at the next update.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "standing")]
pub enum PluginSourceStanding {
    /// It answered, so the plugin can be updated from it.
    #[serde(rename = "reachable")]
    Reachable,
    /// It did not answer, or is no longer there, so the plugin cannot be updated from it.
    #[serde(rename = "unreachable")]
    Unreachable {
        /// What asking it said.
        why: String,
    },
    /// Nothing was asked, so nothing is known either way.
    #[serde(rename = "unasked")]
    Unasked {
        /// Why nothing was asked.
        why: String,
    },
}

/// One call a recipe makes, in the order it makes them.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginStep {
    /// The step's id within its recipe.
    pub id: String,
    /// The HTTP method it calls with.
    pub method: String,
    /// The API the destination is reached in, as the installed plugin providing it
    /// declares it, or nothing where none does or it is a name outside the stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    /// The path it calls.
    pub path: String,
    /// Where it calls: a service of this stack's or the plugin's own, or a name outside
    /// both, as the manifest wrote it. Never a resolved address.
    pub to: String,
}

/// What one step came to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginStepRan {
    /// What it came to.
    pub came: super::StepCame,
    /// Whether it reached somewhere other than this plugin's own services, which an
    /// install going back cannot undo.
    pub landed: bool,
    /// The status its last answer carried, where anything answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// The step's id.
    pub step: String,
    /// Where it calls, by the name the manifest gives it.
    pub to: String,
    /// How many times it was made.
    pub tries: u32,
    /// Why it came to what it did, where that wants saying.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}
