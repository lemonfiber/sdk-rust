//! Every code a refusal may carry, with the status it is answered with.

/// `ADMIT-4`: Raised when a request carried no token, session or key this run admits.
pub const NOT_ADMITTED: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-4",
    name: "NOT_ADMITTED",
    status: 403,
    description: "Raised when a request carried no token, session or key this run admits.",
};

/// `ADMIT-5`: Raised when a request said it came from somewhere this server is not.
pub const ELSEWHERE: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-5",
    name: "ELSEWHERE",
    status: 403,
    description: "Raised when a request said it came from somewhere this server is not.",
};

/// `ADMIT-6`: Raised when an account asked for something that is not its to ask for.
pub const NOT_YOURS: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-6",
    name: "NOT_YOURS",
    status: 403,
    description: "Raised when an account asked for something that is not its to ask for.",
};

/// `ADMIT-7`: Raised when the media server could not say whether an account is still one.
pub const UNCONFIRMED: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-7",
    name: "UNCONFIRMED",
    status: 403,
    description: "Raised when the media server could not say whether an account is still one.",
};

/// `ADMIT-8`: Raised when the password offered at the door was wrong, or none is set.
pub const NOT_THE_PASSWORD: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-8",
    name: "NOT_THE_PASSWORD",
    status: 401,
    description: "Raised when the password offered at the door was wrong, or none is set.",
};

/// `ADMIT-9`: Raised when the door has been given too many wrong passwords lately.
pub const TOO_MANY_ATTEMPTS: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-9",
    name: "TOO_MANY_ATTEMPTS",
    status: 429,
    description: "Raised when the door has been given too many wrong passwords lately.",
};

/// `ADMIT-10`: Raised when what was offered at the door is not a password.
pub const NOT_A_PASSWORD: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-10",
    name: "NOT_A_PASSWORD",
    status: 400,
    description: "Raised when what was offered at the door is not a password.",
};

/// `ADMIT-11`: Raised when a key arrived from another machine over a connection its pin does not verify.
pub const KEY_IN_THE_CLEAR: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-11",
    name: "KEY_IN_THE_CLEAR",
    status: 403,
    description:
        "Raised when a key arrived from another machine over a connection its pin does not verify.",
};

/// `ADMIT-12`: Raised when a key asked for something its scope does not reach.
pub const NOT_FOR_A_KEY: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-12",
    name: "NOT_FOR_A_KEY",
    status: 403,
    description: "Raised when a key asked for something its scope does not reach.",
};

/// `ADMIT-13`: Raised when a claim names an invitation that is not open.
pub const NOT_OPEN: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-13",
    name: "NOT_OPEN",
    status: 401,
    description: "Raised when a claim names an invitation that is not open.",
};

/// `ADMIT-14`: Raised when the password chosen at a claim is shorter than the least this takes.
pub const SHORT_CHOICE: crate::RefusalCode = crate::RefusalCode {
    code: "ADMIT-14",
    name: "SHORT_CHOICE",
    status: 400,
    description: "Raised when the password chosen at a claim is shorter than the least this takes.",
};

/// `ASK-1`: Raised where no action goes by the name that was asked for.
pub const NO_SUCH_ACTION: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-1",
    name: "NO_SUCH_ACTION",
    status: 404,
    description: "Raised where no action goes by the name that was asked for.",
};

/// `ASK-2`: Raised where an action was not given an argument it needs.
pub const MISSING_ARGUMENT: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-2",
    name: "MISSING_ARGUMENT",
    status: 400,
    description: "Raised where an action was not given an argument it needs.",
};

/// `ASK-3`: Raised where an argument was given a value that names nothing.
pub const UNRECOGNISED_ARGUMENT: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-3",
    name: "UNRECOGNISED_ARGUMENT",
    status: 400,
    description: "Raised where an argument was given a value that names nothing.",
};

/// `ASK-4`: Raised where an action was given an argument its command has nowhere to put.
pub const UNWANTED_ARGUMENT: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-4",
    name: "UNWANTED_ARGUMENT",
    status: 400,
    description: "Raised where an action was given an argument its command has nowhere to put.",
};

/// `ASK-5`: Raised where two arguments that each name a different request arrived together.
pub const ARGUMENTS_TOGETHER: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-5",
    name: "ARGUMENTS_TOGETHER",
    status: 400,
    description: "Raised where two arguments that each name a different request arrived together.",
};

/// `ASK-6`: Raised where the body of an action is not arguments it can read.
pub const NOT_ARGUMENTS: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-6",
    name: "NOT_ARGUMENTS",
    status: 400,
    description: "Raised where the body of an action is not arguments it can read.",
};

/// `ASK-7`: Raised where a job was asked about that this run did not start.
pub const NO_SUCH_JOB: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-7",
    name: "NO_SUCH_JOB",
    status: 404,
    description: "Raised where a job was asked about that this run did not start.",
};

/// `ASK-8`: Raised where the body of a setup step is not an answer it can read.
pub const NOT_AN_ANSWER: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-8",
    name: "NOT_AN_ANSWER",
    status: 400,
    description: "Raised where the body of a setup step is not an answer it can read.",
};

/// `ASK-9`: Raised where a path under the endpoints is one no endpoint answers.
pub const NO_ENDPOINT: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-9",
    name: "NO_ENDPOINT",
    status: 404,
    description: "Raised where a path under the endpoints is one no endpoint answers.",
};

/// `ASK-10`: Raised where an endpoint was asked with a method it does not answer.
pub const WRONG_METHOD: crate::RefusalCode = crate::RefusalCode {
    code: "ASK-10",
    name: "WRONG_METHOD",
    status: 405,
    description: "Raised where an endpoint was asked with a method it does not answer.",
};

/// `ASK-11`: Raised where the body of a mint is not a key's name, scope, purpose and the password, or the body of a collection is not a claim alone.
pub const NOT_A_KEY_REQUEST: crate::RefusalCode = crate::RefusalCode {
code: "ASK-11",
name: "NOT_A_KEY_REQUEST",
status: 400,
description: "Raised where the body of a mint is not a key's name, scope, purpose and the password, or the body of a collection is not a claim alone.",
};

/// `ASK-12`: Raised where an action's `Idempotency-Key` is not one to 255 visible characters, or is given more than once.
pub const NOT_AN_IDEMPOTENCY_KEY: crate::RefusalCode = crate::RefusalCode {
code: "ASK-12",
name: "NOT_AN_IDEMPOTENCY_KEY",
status: 400,
description: "Raised where an action's `Idempotency-Key` is not one to 255 visible characters, or is given more than once.",
};

/// `ASK-13`: Raised where an `Idempotency-Key` already sent with one action and its arguments is sent with another.
pub const IDEMPOTENCY_KEY_REUSED: crate::RefusalCode = crate::RefusalCode {
code: "ASK-13",
name: "IDEMPOTENCY_KEY_REUSED",
status: 400,
description: "Raised where an `Idempotency-Key` already sent with one action and its arguments is sent with another.",
};

/// `BUNDLE-8`: Raised when a name does not name one of the bundles this run kept.
pub const BUNDLE_NOT_HELD: crate::RefusalCode = crate::RefusalCode {
    code: "BUNDLE-8",
    name: "BUNDLE_NOT_HELD",
    status: 404,
    description: "Raised when a name does not name one of the bundles this run kept.",
};

/// `CRED-4`: Raised where a replacement of the media server's internal certificate answers an offer that is not the one a fresh reading builds.
pub const REPLACEMENT_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "CRED-4",
name: "REPLACEMENT_MOVED",
status: 400,
description: "Raised where a replacement of the media server's internal certificate answers an offer that is not the one a fresh reading builds.",
};

/// `DEVICE-1`: Raised when a device key is asked for under a word that cannot name one.
pub const NOT_A_DEVICE_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-1",
    name: "NOT_A_DEVICE_NAME",
    status: 400,
    description: "Raised when a device key is asked for under a word that cannot name one.",
};

/// `DEVICE-2`: Raised when a device key is registered under a name another device key holds.
pub const DEVICE_NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-2",
    name: "DEVICE_NAME_TAKEN",
    status: 409,
    description: "Raised when a device key is registered under a name another device key holds.",
};

/// `DEVICE-3`: Raised when a device key is registered with a public half another device key holds.
pub const PUBLIC_HALF_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-3",
    name: "PUBLIC_HALF_TAKEN",
    status: 409,
    description:
        "Raised when a device key is registered with a public half another device key holds.",
};

/// `DEVICE-4`: Raised when the public half given is not a P-256 key.
pub const NOT_P256: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-4",
    name: "NOT_P256",
    status: 400,
    description: "Raised when the public half given is not a P-256 key.",
};

/// `DEVICE-5`: Raised when a device key is registered with no attestation.
pub const NO_ATTESTATION: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-5",
    name: "NO_ATTESTATION",
    status: 400,
    description: "Raised when a device key is registered with no attestation.",
};

/// `DEVICE-6`: Raised when an attestation does not verify, or does not cover the public half.
pub const ATTESTATION_UNVERIFIED: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-6",
    name: "ATTESTATION_UNVERIFIED",
    status: 400,
    description: "Raised when an attestation does not verify, or does not cover the public half.",
};

/// `DEVICE-7`: Raised when an attestation places the key in software, or says it was not made in the hardware that keeps it.
pub const IN_SOFTWARE: crate::RefusalCode = crate::RefusalCode {
code: "DEVICE-7",
name: "IN_SOFTWARE",
status: 400,
description: "Raised when an attestation places the key in software, or says it was not made in the hardware that keeps it.",
};

/// `DEVICE-8`: Raised when an attestation names an app this stack does not trust.
pub const UNTRUSTED_APP: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-8",
    name: "UNTRUSTED_APP",
    status: 403,
    description: "Raised when an attestation names an app this stack does not trust.",
};

/// `DEVICE-9`: Raised when a device key named is not registered, or was revoked.
pub const NOT_REGISTERED: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-9",
    name: "NOT_REGISTERED",
    status: 404,
    description: "Raised when a device key named is not registered, or was revoked.",
};

/// `DEVICE-10`: Raised when a mint carries a challenge that does not stand.
pub const CHALLENGE_NOT_STANDING: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-10",
    name: "CHALLENGE_NOT_STANDING",
    status: 403,
    description: "Raised when a mint carries a challenge that does not stand.",
};

/// `DEVICE-11`: Raised when a mint's signature does not verify against the device key.
pub const SIGNATURE_WRONG: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-11",
    name: "SIGNATURE_WRONG",
    status: 401,
    description: "Raised when a mint's signature does not verify against the device key.",
};

/// `DEVICE-12`: Raised when this machine would not supply the bytes a challenge is made of.
pub const NO_CHALLENGE: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-12",
    name: "NO_CHALLENGE",
    status: 500,
    description: "Raised when this machine would not supply the bytes a challenge is made of.",
};

/// `DEVICE-13`: Raised when an attestation's chain carries a certificate Google lists as revoked or suspended.
pub const ATTESTATION_REVOKED: crate::RefusalCode = crate::RefusalCode {
code: "DEVICE-13",
name: "ATTESTATION_REVOKED",
status: 403,
description: "Raised when an attestation's chain carries a certificate Google lists as revoked or suspended.",
};

/// `FORM-1`: Raised when no form was named.
pub const NO_FORM_NAMED: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-1",
    name: "NO_FORM_NAMED",
    status: 400,
    description: "Raised when no form was named.",
};

/// `FORM-2`: Raised when a named form is not declared by the stack.
pub const NO_SUCH_FORM: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-2",
    name: "NO_SUCH_FORM",
    status: 404,
    description: "Raised when a named form is not declared by the stack.",
};

/// `FORM-3`: Raised when forms that cannot be combined are named together.
pub const FORMS_CONFLICT: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-3",
    name: "FORMS_CONFLICT",
    status: 400,
    description: "Raised when forms that cannot be combined are named together.",
};

/// `GONE-2`: Raised when an agreement names a reading of this machine that is not the one standing now.
pub const ANOTHER_READING: crate::RefusalCode = crate::RefusalCode {
    code: "GONE-2",
    name: "ANOTHER_READING",
    status: 400,
    description:
        "Raised when an agreement names a reading of this machine that is not the one standing now.",
};

/// `KEY-1`: Raised when a key is asked for under a word that cannot name one.
pub const BAD_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-1",
    name: "BAD_NAME",
    status: 400,
    description: "Raised when a key is asked for under a word that cannot name one.",
};

/// `KEY-2`: Raised when a key is asked for with a scope that is none of the three.
pub const NOT_A_SCOPE: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-2",
    name: "NOT_A_SCOPE",
    status: 400,
    description: "Raised when a key is asked for with a scope that is none of the three.",
};

/// `KEY-3`: Raised when a key is asked for with a purpose that is none of the three.
pub const NOT_A_PURPOSE: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-3",
    name: "NOT_A_PURPOSE",
    status: 400,
    description: "Raised when a key is asked for with a purpose that is none of the three.",
};

/// `KEY-5`: Raised when a key is asked for under a name another key holds.
pub const NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-5",
    name: "NAME_TAKEN",
    status: 400,
    description: "Raised when a key is asked for under a name another key holds.",
};

/// `KEY-6`: Raised when a member's key names an account the household does not hold.
pub const NO_SUCH_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-6",
    name: "NO_SUCH_MEMBER",
    status: 404,
    description: "Raised when a member's key names an account the household does not hold.",
};

/// `KEY-8`: Raised when a household member asks for a key, or a revoke, that is not theirs alone.
pub const NOT_FOR_YOURSELF: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-8",
    name: "NOT_FOR_YOURSELF",
    status: 400,
    description:
        "Raised when a household member asks for a key, or a revoke, that is not theirs alone.",
};

/// `KEY-10`: Raised when no active key holds the name a revoke gave.
pub const NO_SUCH_KEY: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-10",
    name: "NO_SUCH_KEY",
    status: 404,
    description: "Raised when no active key holds the name a revoke gave.",
};

/// `KEY-11`: Raised when a household member asks to mint a key and the operator has not allowed members to.
pub const MEMBERS_MAY_NOT_MINT: crate::RefusalCode = crate::RefusalCode {
code: "KEY-11",
name: "MEMBERS_MAY_NOT_MINT",
status: 409,
description: "Raised when a household member asks to mint a key and the operator has not allowed members to.",
};

/// `KEY-12`: Raised when a key is asked to be collected with a claim that collects nothing.
pub const CLAIM_NOT_OPEN: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-12",
    name: "CLAIM_NOT_OPEN",
    status: 401,
    description: "Raised when a key is asked to be collected with a claim that collects nothing.",
};

/// `KEY-13`: Raised when a key is asked to be minted as a claim and no address another machine reaches can go in its link.
pub const UNLINKABLE: crate::RefusalCode = crate::RefusalCode {
code: "KEY-13",
name: "UNLINKABLE",
status: 409,
description: "Raised when a key is asked to be minted as a claim and no address another machine reaches can go in its link.",
};

/// `LIFE-3`: Another run is already working on this stack.
pub const ALREADY_WORKING: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-3",
    name: "ALREADY_WORKING",
    status: 409,
    description: "Another run is already working on this stack.",
};

/// `LIFE-4`: Fetching images is switched off, so there was nothing to fetch with.
pub const REGISTRY_REFUSED: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-4",
    name: "REGISTRY_REFUSED",
    status: 400,
    description: "Fetching images is switched off, so there was nothing to fetch with.",
};

/// `LIFE-10`: Raised where a restart names an offer that is not the one a fresh look at the stack builds.
pub const RESTART_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "LIFE-10",
name: "RESTART_MOVED",
status: 400,
description: "Raised where a restart names an offer that is not the one a fresh look at the stack builds.",
};

/// `LIFE-11`: Raised when a start finds a service's mount source reached through a link.
pub const MOUNT_LINKED: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-11",
    name: "MOUNT_LINKED",
    status: 409,
    description: "Raised when a start finds a service's mount source reached through a link.",
};

/// `LIFE-12`: Raised where an `up` or a `switch` names an offer that is not the install a fresh look at the stack offers.
pub const OFFLINE_INSTALL_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "LIFE-12",
name: "OFFLINE_INSTALL_MOVED",
status: 400,
description: "Raised where an `up` or a `switch` names an offer that is not the install a fresh look at the stack offers.",
};

/// `LIFE-13`: Raised where a start would install a part this build carries and nobody said yes.
pub const OFFLINE_INSTALL_UNANSWERED: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-13",
    name: "OFFLINE_INSTALL_UNANSWERED",
    status: 409,
    description:
        "Raised where a start would install a part this build carries and nobody said yes.",
};

/// `LIFE-14`: Raised where an install a start was agreed to did not hold.
pub const OFFLINE_INSTALL_UNHELD: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-14",
    name: "OFFLINE_INSTALL_UNHELD",
    status: 409,
    description: "Raised where an install a start was agreed to did not hold.",
};

/// `MIGRATE-1`: Raised when a replacement was agreed to for an offer that is not the one standing now.
pub const OFFER_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "MIGRATE-1",
    name: "OFFER_MOVED",
    status: 400,
    description:
        "Raised when a replacement was agreed to for an offer that is not the one standing now.",
};

/// `PLAY-1`: Said where the title named is not something the media server could hold.
pub const NOT_AN_ITEM: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-1",
    name: "NOT_AN_ITEM",
    status: 400,
    description: "Said where the title named is not something the media server could hold.",
};

/// `PLAY-2`: Said where the title is not one this member may watch, or is not there at all.
pub const NOT_ON_THEIR_SHELF: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-2",
    name: "NOT_ON_THEIR_SHELF",
    status: 404,
    description: "Said where the title is not one this member may watch, or is not there at all.",
};

/// `PLAY-3`: Said where what names a member's device is not a device id.
pub const NOT_A_DEVICE: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-3",
    name: "NOT_A_DEVICE",
    status: 400,
    description: "Said where what names a member's device is not a device id.",
};

/// `PLAY-4`: Said where there is no media server to play from, or it was never set up.
pub const NOTHING_TO_PLAY_FROM: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-4",
    name: "NOTHING_TO_PLAY_FROM",
    status: 503,
    description: "Said where there is no media server to play from, or it was never set up.",
};

/// `PLAY-5`: Said where the media server would not answer for a member.
pub const SERVER_SILENT: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-5",
    name: "SERVER_SILENT",
    status: 502,
    description: "Said where the media server would not answer for a member.",
};

/// `PLAY-6`: Said where nobody is named for something only a member can be.
pub const NOBODY_NAMED: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-6",
    name: "NOBODY_NAMED",
    status: 400,
    description: "Said where nobody is named for something only a member can be.",
};

/// `PLAY-7`: Said where the member named is not somebody in the household.
pub const NOT_IN_THE_HOUSEHOLD: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-7",
    name: "NOT_IN_THE_HOUSEHOLD",
    status: 404,
    description: "Said where the member named is not somebody in the household.",
};

/// `PLAY-8`: Said where the media server will not sign a device in by code.
pub const SIGNS_NO_DEVICE_IN: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-8",
    name: "SIGNS_NO_DEVICE_IN",
    status: 503,
    description: "Said where the media server will not sign a device in by code.",
};

/// `PLAY-9`: Said where a title on the member's shelf has no picture of the kind asked for.
pub const NO_SUCH_PICTURE: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-9",
    name: "NO_SUCH_PICTURE",
    status: 404,
    description: "Said where a title on the member's shelf has no picture of the kind asked for.",
};

/// `PLUGIN-2`: The source names no plugin this build can read.
pub const UNREADABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-2",
    name: "UNREADABLE",
    status: 404,
    description: "The source names no plugin this build can read.",
};

/// `PLUGIN-3`: The manifest is read and this build refuses what it declares.
pub const REFUSED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-3",
    name: "REFUSED",
    status: 400,
    description: "The manifest is read and this build refuses what it declares.",
};

/// `PLUGIN-4`: The record of what is installed cannot be read.
pub const UNRECORDED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-4",
    name: "UNRECORDED",
    status: 500,
    description: "The record of what is installed cannot be read.",
};

/// `PLUGIN-5`: The plugin is installed already.
pub const ALREADY: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-5",
    name: "ALREADY",
    status: 400,
    description: "The plugin is installed already.",
};

/// `PLUGIN-6`: There is no stack on this machine to put a plugin's container in.
pub const NOWHERE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-6",
    name: "NOWHERE",
    status: 500,
    description: "There is no stack on this machine to put a plugin's container in.",
};

/// `PLUGIN-7`: A directory or a document the install decided on would not land.
pub const UNWRITABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-7",
    name: "UNWRITABLE",
    status: 500,
    description: "A directory or a document the install decided on would not land.",
};

/// `PLUGIN-8`: The wiring went down and the record of what is installed did not.
pub const UNRECORDABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-8",
    name: "UNRECORDABLE",
    status: 500,
    description: "The wiring went down and the record of what is installed did not.",
};

/// `PLUGIN-9`: The plugin's own service would not start, so nothing about it could be proved.
pub const UNPROVED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-9",
    name: "UNPROVED",
    status: 500,
    description: "The plugin's own service would not start, so nothing about it could be proved.",
};

/// `PLUGIN-10`: Nothing by that name is installed on this machine.
pub const NOTHING_TO_REMOVE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-10",
    name: "NOTHING_TO_REMOVE",
    status: 404,
    description: "Nothing by that name is installed on this machine.",
};

/// `PLUGIN-11`: Nothing by that id is installed, so there is no version to replace.
pub const NOTHING_TO_UPDATE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-11",
    name: "NOTHING_TO_UPDATE",
    status: 404,
    description: "Nothing by that id is installed, so there is no version to replace.",
};

/// `PLUGIN-12`: The version installed would not come off, so nothing else was touched.
pub const STUCK: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-12",
    name: "STUCK",
    status: 500,
    description: "The version installed would not come off, so nothing else was touched.",
};

/// `PLUGIN-13`: Raised when a plugin's service would answer on a label another plugin's already does.
pub const ANSWERED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-13",
    name: "ANSWERED",
    status: 400,
    description:
        "Raised when a plugin's service would answer on a label another plugin's already does.",
};

/// `PLUGIN-14`: Raised when a plugin is installed from a source other than the one its name is already installed from.
pub const TWO_SOURCES: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-14",
name: "TWO_SOURCES",
status: 400,
description: "Raised when a plugin is installed from a source other than the one its name is already installed from.",
};

/// `PLUGIN-15`: Raised when a plugin is named from a git source and fetching from one is switched off.
pub const SOURCE_OFF: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-15",
    name: "SOURCE_OFF",
    status: 400,
    description:
        "Raised when a plugin is named from a git source and fetching from one is switched off.",
};

/// `PLUGIN-16`: Raised when a git source could not be reached or would not hand over a revision.
pub const UNFETCHED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-16",
    name: "UNFETCHED",
    status: 500,
    description: "Raised when a git source could not be reached or would not hand over a revision.",
};

/// `PLUGIN-17`: Raised when a git source holds no branch, tag or commit by the name given.
pub const NO_REVISION: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-17",
    name: "NO_REVISION",
    status: 404,
    description: "Raised when a git source holds no branch, tag or commit by the name given.",
};

/// `PLUGIN-18`: Raised when a plugin is installed by name and asking the catalogue is switched off.
pub const CATALOGUE_OFF: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-18",
    name: "CATALOGUE_OFF",
    status: 400,
    description:
        "Raised when a plugin is installed by name and asking the catalogue is switched off.",
};

/// `PLUGIN-19`: Raised when the catalogue's index or its signature could not be fetched.
pub const CATALOGUE_UNREACHABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-19",
    name: "CATALOGUE_UNREACHABLE",
    status: 500,
    description: "Raised when the catalogue's index or its signature could not be fetched.",
};

/// `PLUGIN-20`: Raised when the catalogue's index has no signature, one that does not verify, or none this build carries a key to check.
pub const SIGNATURE_UNVERIFIED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-20",
name: "SIGNATURE_UNVERIFIED",
status: 500,
description: "Raised when the catalogue's index has no signature, one that does not verify, or none this build carries a key to check.",
};

/// `PLUGIN-21`: Raised when the catalogue's index verified and is not one this build reads.
pub const CATALOGUE_UNREADABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-21",
    name: "CATALOGUE_UNREADABLE",
    status: 500,
    description: "Raised when the catalogue's index verified and is not one this build reads.",
};

/// `PLUGIN-22`: Raised when the catalogue holds no plugin by the name given.
pub const NOT_CATALOGUED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-22",
    name: "NOT_CATALOGUED",
    status: 404,
    description: "Raised when the catalogue holds no plugin by the name given.",
};

/// `PLUGIN-23`: Raised when what the catalogue's origin served is not what the catalogue reviewed.
pub const NOT_AS_REVIEWED: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-23",
    name: "NOT_AS_REVIEWED",
    status: 500,
    description:
        "Raised when what the catalogue's origin served is not what the catalogue reviewed.",
};

/// `PLUGIN-24`: Raised when a plugin's service would be named, where lemonfiber keeps what a service holds, as another installed plugin's service already is.
pub const SPELLED_ALIKE: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-24",
name: "SPELLED_ALIKE",
status: 400,
description: "Raised when a plugin's service would be named, where lemonfiber keeps what a service holds, as another installed plugin's service already is.",
};

/// `PLUGIN-25`: Raised when an install, an update or a removal answers an offer that was read against a plugin, a stack or a record that has since moved.
pub const PLUGIN_OFFER_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-25",
name: "PLUGIN_OFFER_MOVED",
status: 400,
description: "Raised when an install, an update or a removal answers an offer that was read against a plugin, a stack or a record that has since moved.",
};

/// `PLUGIN-26`: Raised when a value a recipe would carry to a destination, or the egress guard's shape a service would take, was not approved as itself, or an approval names something the reading does not list.
pub const UNAPPROVED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-26",
name: "UNAPPROVED",
status: 400,
description: "Raised when a value a recipe would carry to a destination, or the egress guard's shape a service would take, was not approved as itself, or an approval names something the reading does not list.",
};

/// `PLUGIN-27`: Raised when the source an update names holds a different plugin from the one it was asked to update.
pub const ANOTHER_PLUGIN: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-27",
name: "ANOTHER_PLUGIN",
status: 400,
description: "Raised when the source an update names holds a different plugin from the one it was asked to update.",
};

/// `PLUGIN-28`: Raised when a plugin's service would take a name, a port or a label something already on this machine holds: a service of the stack or of the operator's overlay, another plugin's port, or a site in the proxy's live configuration.
pub const OCCUPIED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-28",
name: "OCCUPIED",
status: 400,
description: "Raised when a plugin's service would take a name, a port or a label something already on this machine holds: a service of the stack or of the operator's overlay, another plugin's port, or a site in the proxy's live configuration.",
};

/// `PLUGIN-29`: Raised when the catalogue's index verifies and is older than the newest one this machine has verified.
pub const CATALOGUE_REPLACED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-29",
name: "CATALOGUE_REPLACED",
status: 500,
description: "Raised when the catalogue's index verifies and is older than the newest one this machine has verified.",
};

/// `PLUGIN-30`: Raised when the record of the newest catalogue index this machine verified cannot be read or written.
pub const NEWEST_UNKEPT: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-30",
name: "NEWEST_UNKEPT",
status: 500,
description: "Raised when the record of the newest catalogue index this machine verified cannot be read or written.",
};

/// `PLUGIN-31`: Raised when a git source is named over a transport other than https, before anything is asked of it.
pub const SCHEME_REFUSED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-31",
name: "SCHEME_REFUSED",
status: 400,
description: "Raised when a git source is named over a transport other than https, before anything is asked of it.",
};

/// `PLUGIN-32`: Raised when a git source's host is, or stands for, an address on this machine or on a network of its own: loopback, private, link-local or unspecified.
pub const ADDRESS_REFUSED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-32",
name: "ADDRESS_REFUSED",
status: 400,
description: "Raised when a git source's host is, or stands for, an address on this machine or on a network of its own: loopback, private, link-local or unspecified.",
};

/// `PLUGIN-33`: Raised when a recipe substitutes a value into a header's name, which is a fixed identifier of the protocol and written out; the manifest's every other fault is listed beside it.
pub const HEADER_NAMED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-33",
name: "HEADER_NAMED",
status: 400,
description: "Raised when a recipe substitutes a value into a header's name, which is a fixed identifier of the protocol and written out; the manifest's every other fault is listed beside it.",
};

/// `PLUGIN-34`: Raised when a recipe of the act asks the operator for a value that was not given, or a value was given that no recipe of the act asks for.
pub const INPUT_UNMATCHED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-34",
name: "INPUT_UNMATCHED",
status: 400,
description: "Raised when a recipe of the act asks the operator for a value that was not given, or a value was given that no recipe of the act asks for.",
};

/// `PLUGIN-35`: Raised when a recipe's call was not sent because its host stands for an address not out on the internet; the install or update was put back.
pub const CALL_REFUSED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-35",
name: "CALL_REFUSED",
status: 500,
description: "Raised when a recipe's call was not sent because its host stands for an address not out on the internet; the install or update was put back.",
};

/// `PLUGIN-36`: Raised when a recipe's step failed any other way — nothing answered, the answer was not the one it expects, a capture found nothing, or the answer was larger than a recipe reads; the install or update was put back.
pub const STEP_FAILED: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-36",
name: "STEP_FAILED",
status: 500,
description: "Raised when a recipe's step failed any other way — nothing answered, the answer was not the one it expects, a capture found nothing, or the answer was larger than a recipe reads; the install or update was put back.",
};

/// `PLUGIN-37`: Raised when a recipe's call path is not a plain absolute path; the manifest's every other fault is listed beside it.
pub const PATH_NOT_PLAIN: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-37",
name: "PATH_NOT_PLAIN",
status: 400,
description: "Raised when a recipe's call path is not a plain absolute path; the manifest's every other fault is listed beside it.",
};

/// `PLUGIN-38`: Raised when a recipe's call was not sent because a value it carries may not go where it was going: not where its pairs say, not back to the service a credential belongs to, or outside without its approval; the install or update was put back.
pub const VALUE_WITHHELD: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-38",
name: "VALUE_WITHHELD",
status: 400,
description: "Raised when a recipe's call was not sent because a value it carries may not go where it was going: not where its pairs say, not back to the service a credential belongs to, or outside without its approval; the install or update was put back.",
};

/// `PLUGIN-39`: Nothing by that id is installed, so there is nothing to prove again.
pub const NOTHING_TO_PROVE: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-39",
    name: "NOTHING_TO_PROVE",
    status: 404,
    description: "Nothing by that id is installed, so there is nothing to prove again.",
};

/// `PLUGIN-40`: Raised when a plugin that is not first-party declares a service replacing one the stack ships.
pub const STRANGER_REPLACING: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-40",
name: "STRANGER_REPLACING",
status: 403,
description: "Raised when a plugin that is not first-party declares a service replacing one the stack ships.",
};

/// `PLUGIN-44`: Raised when a run names a plugin that is not installed, or a demand recipe the plugin does not declare.
pub const NOTHING_TO_RUN: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-44",
name: "NOTHING_TO_RUN",
status: 404,
description: "Raised when a run names a plugin that is not installed, or a demand recipe the plugin does not declare.",
};

/// `PLUGIN-45`: Raised when the manifest kept for an installed plugin is missing or is not the one that was installed, so its demand recipes cannot be read.
pub const MANIFEST_UNKEPT: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-45",
name: "MANIFEST_UNKEPT",
status: 409,
description: "Raised when the manifest kept for an installed plugin is missing or is not the one that was installed, so its demand recipes cannot be read.",
};

/// `PLUGIN-46`: Raised when a third-party plugin declares a secret of another service that names no capability of its own it is for.
pub const UNAIMED_CROSSING: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-46",
name: "UNAIMED_CROSSING",
status: 403,
description: "Raised when a third-party plugin declares a secret of another service that names no capability of its own it is for.",
};

/// `PLUGIN-47`: Raised when a grant asked to be revoked is not one the plugin holds.
pub const NO_GRANT: crate::RefusalCode = crate::RefusalCode {
    code: "PLUGIN-47",
    name: "NO_GRANT",
    status: 404,
    description: "Raised when a grant asked to be revoked is not one the plugin holds.",
};

/// `PLUGIN-48`: Raised when a plugin's setting or secret names, as the variable an earlier release kept its value under, one another installed plugin's setting or secret names.
pub const EARLIER_TAKEN: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-48",
name: "EARLIER_TAKEN",
status: 400,
description: "Raised when a plugin's setting or secret names, as the variable an earlier release kept its value under, one another installed plugin's setting or secret names.",
};

/// `PLUGIN-49`: Raised when a plugin carrying the id of a part this build pins comes from anywhere but that pin.
pub const ANOTHER_SOURCE: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-49",
name: "ANOTHER_SOURCE",
status: 403,
description: "Raised when a plugin carrying the id of a part this build pins comes from anywhere but that pin.",
};

/// `PLUGIN-50`: Raised when a case of a contract a plugin's adapter speaks fails, or no recording the plugin ships answers it.
pub const UNCONFORMING: crate::RefusalCode = crate::RefusalCode {
code: "PLUGIN-50",
name: "UNCONFORMING",
status: 400,
description: "Raised when a case of a contract a plugin's adapter speaks fails, or no recording the plugin ships answers it.",
};

/// `QUOTA-2`: Raised where a policy that lives inside a limit was chosen without one.
pub const NO_LIMIT: crate::RefusalCode = crate::RefusalCode {
    code: "QUOTA-2",
    name: "NO_LIMIT",
    status: 400,
    description: "Raised where a policy that lives inside a limit was chosen without one.",
};

/// `QUOTA-4`: Raised where the request named is not one that is waiting on anybody.
pub const NOT_WAITING: crate::RefusalCode = crate::RefusalCode {
    code: "QUOTA-4",
    name: "NOT_WAITING",
    status: 404,
    description: "Raised where the request named is not one that is waiting on anybody.",
};

/// `QUOTA-5`: Raised where a request was turned down and the reason said nothing.
pub const NO_REASON: crate::RefusalCode = crate::RefusalCode {
    code: "QUOTA-5",
    name: "NO_REASON",
    status: 400,
    description: "Raised where a request was turned down and the reason said nothing.",
};

/// `QUOTA-6`: Raised where nobody in the household goes by the name that was given.
pub const NOBODY: crate::RefusalCode = crate::RefusalCode {
    code: "QUOTA-6",
    name: "NOBODY",
    status: 404,
    description: "Raised where nobody in the household goes by the name that was given.",
};

/// `QUOTA-7`: Raised where the request service holds no account for somebody who has one here.
pub const NEVER_HERE: crate::RefusalCode = crate::RefusalCode {
    code: "QUOTA-7",
    name: "NEVER_HERE",
    status: 404,
    description: "Raised where the request service holds no account for somebody who has one here.",
};

/// `RATE-1`: Raised when a limit is expressed as a share of a line nothing has measured.
pub const NOTHING_MEASURED: crate::RefusalCode = crate::RefusalCode {
    code: "RATE-1",
    name: "NOTHING_MEASURED",
    status: 400,
    description: "Raised when a limit is expressed as a share of a line nothing has measured.",
};

/// `RATE-2`: Raised when a schedule is asked for and nothing says which zone the clients would read it in.
pub const NO_ZONE: crate::RefusalCode = crate::RefusalCode {
code: "RATE-2",
name: "NO_ZONE",
status: 400,
description: "Raised when a schedule is asked for and nothing says which zone the clients would read it in.",
};

/// `RATE-3`: Raised when what was asked for could not be read as a limit, a window or a cap.
pub const LIMIT_UNREADABLE: crate::RefusalCode = crate::RefusalCode {
    code: "RATE-3",
    name: "LIMIT_UNREADABLE",
    status: 400,
    description: "Raised when what was asked for could not be read as a limit, a window or a cap.",
};

/// `RATE-4`: Raised when there is no download client to limit.
pub const NOTHING_TO_LIMIT: crate::RefusalCode = crate::RefusalCode {
    code: "RATE-4",
    name: "NOTHING_TO_LIMIT",
    status: 400,
    description: "Raised when there is no download client to limit.",
};

/// `RATE-5`: Raised when there is no download client to pause or resume.
pub const NOTHING_TO_PAUSE: crate::RefusalCode = crate::RefusalCode {
    code: "RATE-5",
    name: "NOTHING_TO_PAUSE",
    status: 400,
    description: "Raised when there is no download client to pause or resume.",
};

/// `RATE-6`: Raised where pausing or resuming the download clients names an offer that is not the one a fresh look at them builds.
pub const PAUSING_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "RATE-6",
name: "PAUSING_MOVED",
status: 400,
description: "Raised where pausing or resuming the download clients names an offer that is not the one a fresh look at them builds.",
};

/// `READ-1`: Raised where a read was given a parameter its answer has nowhere to put.
pub const UNWANTED: crate::RefusalCode = crate::RefusalCode {
    code: "READ-1",
    name: "UNWANTED",
    status: 400,
    description: "Raised where a read was given a parameter its answer has nowhere to put.",
};

/// `READ-2`: Raised where a parameter carrying one value was given more than once.
pub const REPEATED: crate::RefusalCode = crate::RefusalCode {
    code: "READ-2",
    name: "REPEATED",
    status: 400,
    description: "Raised where a parameter carrying one value was given more than once.",
};

/// `READ-3`: Raised where no read goes by the name that was asked for.
pub const NO_SUCH_READ: crate::RefusalCode = crate::RefusalCode {
    code: "READ-3",
    name: "NO_SUCH_READ",
    status: 404,
    description: "Raised where no read goes by the name that was asked for.",
};

/// `READ-4`: Raised where a trace was asked for and named nothing to follow.
pub const NO_TERM: crate::RefusalCode = crate::RefusalCode {
    code: "READ-4",
    name: "NO_TERM",
    status: 400,
    description: "Raised where a trace was asked for and named nothing to follow.",
};

/// `READ-5`: Raised where the season to narrow a trace to is not a number.
pub const NOT_A_SEASON: crate::RefusalCode = crate::RefusalCode {
    code: "READ-5",
    name: "NOT_A_SEASON",
    status: 400,
    description: "Raised where the season to narrow a trace to is not a number.",
};

/// `READ-6`: Raised where a setting was asked for by an empty name.
pub const NO_SETTING: crate::RefusalCode = crate::RefusalCode {
    code: "READ-6",
    name: "NO_SETTING",
    status: 400,
    description: "Raised where a setting was asked for by an empty name.",
};

/// `READ-7`: Raised where a household member was asked for by an empty name.
pub const NO_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "READ-7",
    name: "NO_MEMBER",
    status: 400,
    description: "Raised where a household member was asked for by an empty name.",
};

/// `READ-8`: Raised where a shelf was asked for and nobody was named whose it is.
pub const NO_SHELF_WITHOUT_A_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "READ-8",
    name: "NO_SHELF_WITHOUT_A_MEMBER",
    status: 400,
    description: "Raised where a shelf was asked for and nobody was named whose it is.",
};

/// `READ-9`: Raised where how many holdings to answer with is not a whole number.
pub const NOT_A_COUNT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-9",
    name: "NOT_A_COUNT",
    status: 400,
    description: "Raised where how many holdings to answer with is not a whole number.",
};

/// `READ-10`: Raised where more holdings were asked for than one read answers with.
pub const TOO_MANY_AT_ONCE: crate::RefusalCode = crate::RefusalCode {
    code: "READ-10",
    name: "TOO_MANY_AT_ONCE",
    status: 400,
    description: "Raised where more holdings were asked for than one read answers with.",
};

/// `READ-11`: Raised where a diagnosis was narrowed to a group or check that is not one.
pub const NO_SUCH_GROUP: crate::RefusalCode = crate::RefusalCode {
    code: "READ-11",
    name: "NO_SUCH_GROUP",
    status: 400,
    description: "Raised where a diagnosis was narrowed to a group or check that is not one.",
};

/// `READ-12`: Raised where a removal was named that is none of the four there are.
pub const NO_SUCH_REMOVAL: crate::RefusalCode = crate::RefusalCode {
    code: "READ-12",
    name: "NO_SUCH_REMOVAL",
    status: 400,
    description: "Raised where a removal was named that is none of the four there are.",
};

/// `READ-13`: Raised where moving forward was asked about and neither stack nor self named.
pub const NO_UPDATE_OBJECT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-13",
    name: "NO_UPDATE_OBJECT",
    status: 400,
    description: "Raised where moving forward was asked about and neither stack nor self named.",
};

/// `READ-14`: Raised where how many log lines to begin with is not a number within the ceiling.
pub const NOT_A_LINE_COUNT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-14",
    name: "NOT_A_LINE_COUNT",
    status: 400,
    description:
        "Raised where how many log lines to begin with is not a number within the ceiling.",
};

/// `READ-15`: Raised where a parameter that takes a yes or a no is neither true nor false.
pub const NOT_A_CHOICE: crate::RefusalCode = crate::RefusalCode {
    code: "READ-15",
    name: "NOT_A_CHOICE",
    status: 400,
    description: "Raised where a parameter that takes a yes or a no is neither true nor false.",
};

/// `READ-16`: Raised where a household read named a member and asked for the household's defaults as well.
pub const MEMBER_AND_DEFAULTS: crate::RefusalCode = crate::RefusalCode {
code: "READ-16",
name: "MEMBER_AND_DEFAULTS",
status: 400,
description: "Raised where a household read named a member and asked for the household's defaults as well.",
};

/// `REHEARSE-1`: The flag cannot be honoured by this command, and never will be.
pub const CANNOT: crate::RefusalCode = crate::RefusalCode {
    code: "REHEARSE-1",
    name: "CANNOT",
    status: 400,
    description: "The flag cannot be honoured by this command, and never will be.",
};

/// `REHEARSE-2`: The flag is not honoured by this command yet.
pub const NOT_YET: crate::RefusalCode = crate::RefusalCode {
    code: "REHEARSE-2",
    name: "NOT_YET",
    status: 400,
    description: "The flag is not honoured by this command yet.",
};

/// `REMOVE-7`: Said where the offer answered is not the one a reading of the removal makes now.
pub const REMOVAL_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "REMOVE-7",
    name: "REMOVAL_MOVED",
    status: 400,
    description: "Said where the offer answered is not the one a reading of the removal makes now.",
};

/// `REPAIR-1`: Raised when consent was given for an offer that no longer stands.
pub const STALE: crate::RefusalCode = crate::RefusalCode {
    code: "REPAIR-1",
    name: "STALE",
    status: 400,
    description: "Raised when consent was given for an offer that no longer stands.",
};

/// `RESTORE-11`: Raised when consent was given for a listing that no longer stands.
pub const MOVED_ON: crate::RefusalCode = crate::RefusalCode {
    code: "RESTORE-11",
    name: "MOVED_ON",
    status: 400,
    description: "Raised when consent was given for a listing that no longer stands.",
};

/// `SERVE-6`: Raised when an answer could not be rendered.
pub const UNRENDERABLE: crate::RefusalCode = crate::RefusalCode {
    code: "SERVE-6",
    name: "UNRENDERABLE",
    status: 500,
    description: "Raised when an answer could not be rendered.",
};

/// `SERVE-7`: Raised when this machine will not supply the randomness a job is named with.
pub const NO_JOB_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "SERVE-7",
    name: "NO_JOB_NAME",
    status: 500,
    description: "Raised when this machine will not supply the randomness a job is named with.",
};

/// `SERVE-8`: Raised when an action's work ended before it had an answer to give.
pub const UNANSWERED: crate::RefusalCode = crate::RefusalCode {
    code: "SERVE-8",
    name: "UNANSWERED",
    status: 500,
    description: "Raised when an action's work ended before it had an answer to give.",
};

/// `SETTING-1`: Raised where nothing in this stack reads a setting by the name given.
pub const NO_SUCH_SETTING: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-1",
    name: "NO_SUCH_SETTING",
    status: 404,
    description: "Raised where nothing in this stack reads a setting by the name given.",
};

/// `SETTING-2`: Raised where the name given is a secret's, which this command never writes.
pub const A_SECRET: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-2",
    name: "A_SECRET",
    status: 400,
    description: "Raised where the name given is a secret's, which this command never writes.",
};

/// `SETTING-3`: Raised where the name given belongs to the other of the two commands that write settings.
pub const TAKEN_ELSEWHERE: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-3",
    name: "TAKEN_ELSEWHERE",
    status: 400,
    description:
        "Raised where the name given belongs to the other of the two commands that write settings.",
};

/// `SETTING-4`: Raised where a value cannot be written as the setting asked for.
pub const UNWRITABLE_VALUE: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-4",
    name: "UNWRITABLE_VALUE",
    status: 400,
    description: "Raised where a value cannot be written as the setting asked for.",
};

/// `SETTING-5`: Raised when a change answers an offer read against settings that have since moved.
pub const SETTINGS_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-5",
    name: "SETTINGS_MOVED",
    status: 400,
    description:
        "Raised when a change answers an offer read against settings that have since moved.",
};

/// `SETTING-6`: Raised where the setting already holds what the change asks for.
pub const ALREADY_HOLDS: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-6",
    name: "ALREADY_HOLDS",
    status: 400,
    description: "Raised where the setting already holds what the change asks for.",
};

/// `SETTING-7`: Raised where a change would leave a setting holding a value outside its choices.
pub const OUTSIDE_ITS_CHOICES: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-7",
    name: "OUTSIDE_ITS_CHOICES",
    status: 400,
    description: "Raised where a change would leave a setting holding a value outside its choices.",
};

/// `SETTING-9`: Raised where `PLEX_LINK_MODE` would enter or leave `owner` for a request service already set up.
pub const OWNER_SET_UP: crate::RefusalCode = crate::RefusalCode {
code: "SETTING-9",
name: "OWNER_SET_UP",
status: 409,
description: "Raised where `PLEX_LINK_MODE` would enter or leave `owner` for a request service already set up.",
};

/// `SETUP-1`: Raised when apply is asked for before the answers have been reviewed.
pub const NOT_REVIEWED: crate::RefusalCode = crate::RefusalCode {
    code: "SETUP-1",
    name: "NOT_REVIEWED",
    status: 400,
    description: "Raised when apply is asked for before the answers have been reviewed.",
};

/// `SETUP-5`: Raised when an answer is not meaningful on the platform setup is running on.
pub const DOES_NOT_APPLY: crate::RefusalCode = crate::RefusalCode {
    code: "SETUP-5",
    name: "DOES_NOT_APPLY",
    status: 400,
    description: "Raised when an answer is not meaningful on the platform setup is running on.",
};

/// `SETUP-7`: Raised when setup is answered on a machine that is already set up.
pub const ALREADY_SET_UP: crate::RefusalCode = crate::RefusalCode {
    code: "SETUP-7",
    name: "ALREADY_SET_UP",
    status: 400,
    description: "Raised when setup is answered on a machine that is already set up.",
};

/// `SETUP-8`: Raised when a recovery is asked for and no apply stopped part-way.
pub const NOTHING_TO_RECOVER: crate::RefusalCode = crate::RefusalCode {
    code: "SETUP-8",
    name: "NOTHING_TO_RECOVER",
    status: 400,
    description: "Raised when a recovery is asked for and no apply stopped part-way.",
};

/// `SETUP-16`: Raised when a VPN answer leaves the egress guard without what it needs to start.
pub const GUARD_UNANSWERED: crate::RefusalCode = crate::RefusalCode {
    code: "SETUP-16",
    name: "GUARD_UNANSWERED",
    status: 400,
    description: "Raised when a VPN answer leaves the egress guard without what it needs to start.",
};

/// `SPACE-2`: Raised when there is no data location to measure.
pub const NOWHERE_TO_MEASURE: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-2",
    name: "NOWHERE_TO_MEASURE",
    status: 400,
    description: "Raised when there is no data location to measure.",
};

/// `SPACE-4`: Raised when there is no torrent client here to be holding a completed download.
pub const NOTHING_TO_ASK: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-4",
    name: "NOTHING_TO_ASK",
    status: 400,
    description: "Raised when there is no torrent client here to be holding a completed download.",
};

/// `SPACE-5`: Raised when the client answers and is holding nothing of the name given.
pub const NOT_HELD: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-5",
    name: "NOT_HELD",
    status: 400,
    description: "Raised when the client answers and is holding nothing of the name given.",
};

/// `SPACE-6`: Raised when an agreement names an offer that is not the one standing now.
pub const ANOTHER_OFFER: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-6",
    name: "ANOTHER_OFFER",
    status: 400,
    description: "Raised when an agreement names an offer that is not the one standing now.",
};

/// `STACK-1`: Raised when a stack directory holds no readable manifest.
pub const STACK_UNREADABLE: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-1",
    name: "STACK_UNREADABLE",
    status: 500,
    description: "Raised when a stack directory holds no readable manifest.",
};

/// `STACK-2`: Raised when a manifest is readable and this build cannot use it.
pub const STACK_UNUSABLE: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-2",
    name: "STACK_UNUSABLE",
    status: 500,
    description: "Raised when a manifest is readable and this build cannot use it.",
};

/// `STACK-4`: Raised when lemonfiber has nowhere to write the stack.
pub const STACK_NOT_SET_UP: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-4",
    name: "STACK_NOT_SET_UP",
    status: 500,
    description: "Raised when lemonfiber has nowhere to write the stack.",
};

/// `STACK-5`: Raised when the stack could not be written to disk.
pub const STACK_NOT_WRITTEN: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-5",
    name: "STACK_NOT_WRITTEN",
    status: 500,
    description: "Raised when the stack could not be written to disk.",
};

/// `STACK-6`: Raised when a manifest parses and breaks the contract.
pub const STACK_INVALID: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-6",
    name: "STACK_INVALID",
    status: 500,
    description: "Raised when a manifest parses and breaks the contract.",
};

/// `STACK-7`: Raised when a manifest is not TOML at all.
pub const STACK_MALFORMED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-7",
    name: "STACK_MALFORMED",
    status: 500,
    description: "Raised when a manifest is not TOML at all.",
};

/// `STACK-8`: Raised when a manifest declares names this build does not know.
pub const STACK_UNRECOGNISED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-8",
    name: "STACK_UNRECOGNISED",
    status: 500,
    description: "Raised when a manifest declares names this build does not know.",
};

/// `STACK-9`: Raised when a stack names a newer lemonfiber than the one running.
pub const STACK_NEEDS_NEWER: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-9",
    name: "STACK_NEEDS_NEWER",
    status: 500,
    description: "Raised when a stack names a newer lemonfiber than the one running.",
};

/// `STACK-10`: Raised when a manifest's files are not laid out as the contract says.
pub const STACK_UNASSEMBLED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-10",
    name: "STACK_UNASSEMBLED",
    status: 500,
    description: "Raised when a manifest's files are not laid out as the contract says.",
};

/// `STACK-11`: Raised when a change would write over a generated project the operator has edited.
pub const PROJECT_EDITED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-11",
    name: "PROJECT_EDITED",
    status: 409,
    description:
        "Raised when a change would write over a generated project the operator has edited.",
};

/// `STACK-13`: Raised where the offer a reset answers is not the one a reading of it makes now.
pub const RESET_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-13",
    name: "RESET_MOVED",
    status: 400,
    description: "Raised where the offer a reset answers is not the one a reading of it makes now.",
};

/// `STACK-15`: Raised where a start names a fenced service no fence can be written for.
pub const SERVICE_UNFENCED: crate::RefusalCode = crate::RefusalCode {
    code: "STACK-15",
    name: "SERVICE_UNFENCED",
    status: 409,
    description: "Raised where a start names a fenced service no fence can be written for.",
};

/// `STORAGE-9`: Raised where `DATA_ROOT` names an NFS export lemonfiber cannot write into the mount options, so no project is written from it.
pub const ROOT_REFUSED: crate::RefusalCode = crate::RefusalCode {
code: "STORAGE-9",
name: "ROOT_REFUSED",
status: 409,
description: "Raised where `DATA_ROOT` names an NFS export lemonfiber cannot write into the mount options, so no project is written from it.",
};

/// `UNDO-5`: Raised when a run moved one of lemonfiber's own guards onto its plugin, which no undo or removal puts back.
pub const MOVED_FOR_GOOD: crate::RefusalCode = crate::RefusalCode {
code: "UNDO-5",
name: "MOVED_FOR_GOOD",
status: 409,
description: "Raised when a run moved one of lemonfiber's own guards onto its plugin, which no undo or removal puts back.",
};

/// `UPDATE-5`: Raised where an update names an offer that is not the one a fresh look at the releases builds.
pub const UPDATE_MOVED: crate::RefusalCode = crate::RefusalCode {
code: "UPDATE-5",
name: "UPDATE_MOVED",
status: 400,
description: "Raised where an update names an offer that is not the one a fresh look at the releases builds.",
};

/// `WIRE-1`: A capability was named that no service in this stack provides.
pub const NO_SUCH_FILLER: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-1",
    name: "NO_SUCH_FILLER",
    status: 404,
    description: "A capability was named that no service in this stack provides.",
};

/// `WIRE-2`: The service named cannot do the thing it was asked to fill.
pub const CANNOT_FILL: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-2",
    name: "CANNOT_FILL",
    status: 400,
    description: "The service named cannot do the thing it was asked to fill.",
};

/// `WIRE-3`: Nothing in this stack asks for the capability, so a choice would change nothing.
pub const NOTHING_ASKS: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-3",
    name: "NOTHING_ASKS",
    status: 400,
    description: "Nothing in this stack asks for the capability, so a choice would change nothing.",
};

/// `WIRE-4`: The setting recording the choice could not be written.
pub const CHOICE_UNWRITABLE: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-4",
    name: "CHOICE_UNWRITABLE",
    status: 500,
    description: "The setting recording the choice could not be written.",
};

/// `WIRE-5`: Raised when a choice answers an offer that was read against a wiring that has since moved.
pub const WIRING_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-5",
    name: "WIRING_MOVED",
    status: 400,
    description:
        "Raised when a choice answers an offer that was read against a wiring that has since moved.",
};

/// `WIRE-6`: Raised when the reason given for a choice is longer than a reason may be, or holds a line break or another control character.
pub const UNREASONABLE: crate::RefusalCode = crate::RefusalCode {
code: "WIRE-6",
name: "UNREASONABLE",
status: 400,
description: "Raised when the reason given for a choice is longer than a reason may be, or holds a line break or another control character.",
};

/// `WIRE-7`: Raised where the service chosen already fills the capability, so there is nothing to change.
pub const ALREADY_FILLS: crate::RefusalCode = crate::RefusalCode {
code: "WIRE-7",
name: "ALREADY_FILLS",
status: 400,
description: "Raised where the service chosen already fills the capability, so there is nothing to change.",
};

/// `WISH-1`: Said where the words searched for are too few or too many to search with.
pub const NOT_A_SEARCH: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-1",
    name: "NOT_A_SEARCH",
    status: 400,
    description: "Said where the words searched for are too few or too many to search with.",
};

/// `WISH-2`: Said where the place a search was asked to go on from is not one a page answered.
pub const NOT_A_PLACE: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-2",
    name: "NOT_A_PLACE",
    status: 400,
    description:
        "Said where the place a search was asked to go on from is not one a page answered.",
};

/// `WISH-3`: Said where no title this member may ask for goes by the id and kind named.
pub const NOT_ASKABLE: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-3",
    name: "NOT_ASKABLE",
    status: 404,
    description: "Said where no title this member may ask for goes by the id and kind named.",
};

/// `WISH-4`: Said where the house cannot fetch that kind of title, or anything, right now.
pub const CANNOT_FETCH: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-4",
    name: "CANNOT_FETCH",
    status: 409,
    description: "Said where the house cannot fetch that kind of title, or anything, right now.",
};

/// `WISH-5`: Said where the title asked for is already in the house.
pub const ALREADY_HERE: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-5",
    name: "ALREADY_HERE",
    status: 409,
    description: "Said where the title asked for is already in the house.",
};

/// `WISH-6`: Said where the member has asked for all their allowance lets them, until it frees up.
pub const NOTHING_LEFT: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-6",
    name: "NOTHING_LEFT",
    status: 409,
    description:
        "Said where the member has asked for all their allowance lets them, until it frees up.",
};

/// `WISH-7`: Said where the house has no room for new things, which is the house's and nobody's limit.
pub const NO_ROOM: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-7",
    name: "NO_ROOM",
    status: 503,
    description:
        "Said where the house has no room for new things, which is the house's and nobody's limit.",
};

/// `WISH-8`: Said where what answers the household's asks did not answer, so nothing was asked.
pub const NOT_WORKING: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-8",
    name: "NOT_WORKING",
    status: 503,
    description:
        "Said where what answers the household's asks did not answer, so nothing was asked.",
};

/// `WISH-9`: Said where the house has nothing to search or ask with at all.
pub const NOT_SET_UP: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-9",
    name: "NOT_SET_UP",
    status: 503,
    description: "Said where the house has nothing to search or ask with at all.",
};

/// `WISH-10`: Said where a season named is not one the title has, or seasons were named for a film.
pub const NOT_ITS_SEASON: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-10",
    name: "NOT_ITS_SEASON",
    status: 400,
    description:
        "Said where a season named is not one the title has, or seasons were named for a film.",
};

/// `WISH-11`: Said where nobody is named for something only a member can do.
pub const NOBODY_ASKING: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-11",
    name: "NOBODY_ASKING",
    status: 400,
    description: "Said where nobody is named for something only a member can do.",
};

/// `WISH-12`: Said where the member named is not somebody in the household.
pub const NOT_A_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-12",
    name: "NOT_A_MEMBER",
    status: 404,
    description: "Said where the member named is not somebody in the household.",
};

/// `WISH-13`: Said where a title a member may ask for has no picture to show them.
pub const NO_PICTURE: crate::RefusalCode = crate::RefusalCode {
    code: "WISH-13",
    name: "NO_PICTURE",
    status: 404,
    description: "Said where a title a member may ask for has no picture to show them.",
};

/// `WORD-1`: A word this product does not explain, as a refusal that says what it does. Through the error model rather than a bare line, so it carries a code and a way forward like every other refusal — and the way forward is the list itself, which is short enough to be the answer rather than a pointer at one. It lies in the naming: the word is the whole of what was asked for, and there is no entry for it. A surface that reported this as its own failure would be telling a caller to try again at something that will never work.
pub const UNRECOGNISED: crate::RefusalCode = crate::RefusalCode {
code: "WORD-1",
name: "UNRECOGNISED",
status: 404,
description: "A word this product does not explain, as a refusal that says what it does. Through the error model rather than a bare line, so it carries a code and a way forward like every other refusal — and the way forward is the list itself, which is short enough to be the answer rather than a pointer at one. It lies in the naming: the word is the whole of what was asked for, and there is no entry for it. A surface that reported this as its own failure would be telling a caller to try again at something that will never work.",
};

/// Every code a refusal may carry.
pub const ALL: &[crate::RefusalCode] = &[
    NOT_ADMITTED,
    ELSEWHERE,
    NOT_YOURS,
    UNCONFIRMED,
    NOT_THE_PASSWORD,
    TOO_MANY_ATTEMPTS,
    NOT_A_PASSWORD,
    KEY_IN_THE_CLEAR,
    NOT_FOR_A_KEY,
    NOT_OPEN,
    SHORT_CHOICE,
    NO_SUCH_ACTION,
    MISSING_ARGUMENT,
    UNRECOGNISED_ARGUMENT,
    UNWANTED_ARGUMENT,
    ARGUMENTS_TOGETHER,
    NOT_ARGUMENTS,
    NO_SUCH_JOB,
    NOT_AN_ANSWER,
    NO_ENDPOINT,
    WRONG_METHOD,
    NOT_A_KEY_REQUEST,
    NOT_AN_IDEMPOTENCY_KEY,
    IDEMPOTENCY_KEY_REUSED,
    BUNDLE_NOT_HELD,
    REPLACEMENT_MOVED,
    NOT_A_DEVICE_NAME,
    DEVICE_NAME_TAKEN,
    PUBLIC_HALF_TAKEN,
    NOT_P256,
    NO_ATTESTATION,
    ATTESTATION_UNVERIFIED,
    IN_SOFTWARE,
    UNTRUSTED_APP,
    NOT_REGISTERED,
    CHALLENGE_NOT_STANDING,
    SIGNATURE_WRONG,
    NO_CHALLENGE,
    ATTESTATION_REVOKED,
    NO_FORM_NAMED,
    NO_SUCH_FORM,
    FORMS_CONFLICT,
    ANOTHER_READING,
    BAD_NAME,
    NOT_A_SCOPE,
    NOT_A_PURPOSE,
    NAME_TAKEN,
    NO_SUCH_MEMBER,
    NOT_FOR_YOURSELF,
    NO_SUCH_KEY,
    MEMBERS_MAY_NOT_MINT,
    CLAIM_NOT_OPEN,
    UNLINKABLE,
    ALREADY_WORKING,
    REGISTRY_REFUSED,
    RESTART_MOVED,
    MOUNT_LINKED,
    OFFLINE_INSTALL_MOVED,
    OFFLINE_INSTALL_UNANSWERED,
    OFFLINE_INSTALL_UNHELD,
    OFFER_MOVED,
    NOT_AN_ITEM,
    NOT_ON_THEIR_SHELF,
    NOT_A_DEVICE,
    NOTHING_TO_PLAY_FROM,
    SERVER_SILENT,
    NOBODY_NAMED,
    NOT_IN_THE_HOUSEHOLD,
    SIGNS_NO_DEVICE_IN,
    NO_SUCH_PICTURE,
    UNREADABLE,
    REFUSED,
    UNRECORDED,
    ALREADY,
    NOWHERE,
    UNWRITABLE,
    UNRECORDABLE,
    UNPROVED,
    NOTHING_TO_REMOVE,
    NOTHING_TO_UPDATE,
    STUCK,
    ANSWERED,
    TWO_SOURCES,
    SOURCE_OFF,
    UNFETCHED,
    NO_REVISION,
    CATALOGUE_OFF,
    CATALOGUE_UNREACHABLE,
    SIGNATURE_UNVERIFIED,
    CATALOGUE_UNREADABLE,
    NOT_CATALOGUED,
    NOT_AS_REVIEWED,
    SPELLED_ALIKE,
    PLUGIN_OFFER_MOVED,
    UNAPPROVED,
    ANOTHER_PLUGIN,
    OCCUPIED,
    CATALOGUE_REPLACED,
    NEWEST_UNKEPT,
    SCHEME_REFUSED,
    ADDRESS_REFUSED,
    HEADER_NAMED,
    INPUT_UNMATCHED,
    CALL_REFUSED,
    STEP_FAILED,
    PATH_NOT_PLAIN,
    VALUE_WITHHELD,
    NOTHING_TO_PROVE,
    STRANGER_REPLACING,
    NOTHING_TO_RUN,
    MANIFEST_UNKEPT,
    UNAIMED_CROSSING,
    NO_GRANT,
    EARLIER_TAKEN,
    ANOTHER_SOURCE,
    UNCONFORMING,
    NO_LIMIT,
    NOT_WAITING,
    NO_REASON,
    NOBODY,
    NEVER_HERE,
    NOTHING_MEASURED,
    NO_ZONE,
    LIMIT_UNREADABLE,
    NOTHING_TO_LIMIT,
    NOTHING_TO_PAUSE,
    PAUSING_MOVED,
    UNWANTED,
    REPEATED,
    NO_SUCH_READ,
    NO_TERM,
    NOT_A_SEASON,
    NO_SETTING,
    NO_MEMBER,
    NO_SHELF_WITHOUT_A_MEMBER,
    NOT_A_COUNT,
    TOO_MANY_AT_ONCE,
    NO_SUCH_GROUP,
    NO_SUCH_REMOVAL,
    NO_UPDATE_OBJECT,
    NOT_A_LINE_COUNT,
    NOT_A_CHOICE,
    MEMBER_AND_DEFAULTS,
    CANNOT,
    NOT_YET,
    REMOVAL_MOVED,
    STALE,
    MOVED_ON,
    UNRENDERABLE,
    NO_JOB_NAME,
    UNANSWERED,
    NO_SUCH_SETTING,
    A_SECRET,
    TAKEN_ELSEWHERE,
    UNWRITABLE_VALUE,
    SETTINGS_MOVED,
    ALREADY_HOLDS,
    OUTSIDE_ITS_CHOICES,
    OWNER_SET_UP,
    NOT_REVIEWED,
    DOES_NOT_APPLY,
    ALREADY_SET_UP,
    NOTHING_TO_RECOVER,
    GUARD_UNANSWERED,
    NOWHERE_TO_MEASURE,
    NOTHING_TO_ASK,
    NOT_HELD,
    ANOTHER_OFFER,
    STACK_UNREADABLE,
    STACK_UNUSABLE,
    STACK_NOT_SET_UP,
    STACK_NOT_WRITTEN,
    STACK_INVALID,
    STACK_MALFORMED,
    STACK_UNRECOGNISED,
    STACK_NEEDS_NEWER,
    STACK_UNASSEMBLED,
    PROJECT_EDITED,
    RESET_MOVED,
    SERVICE_UNFENCED,
    ROOT_REFUSED,
    MOVED_FOR_GOOD,
    UPDATE_MOVED,
    NO_SUCH_FILLER,
    CANNOT_FILL,
    NOTHING_ASKS,
    CHOICE_UNWRITABLE,
    WIRING_MOVED,
    UNREASONABLE,
    ALREADY_FILLS,
    NOT_A_SEARCH,
    NOT_A_PLACE,
    NOT_ASKABLE,
    CANNOT_FETCH,
    ALREADY_HERE,
    NOTHING_LEFT,
    NO_ROOM,
    NOT_WORKING,
    NOT_SET_UP,
    NOT_ITS_SEASON,
    NOBODY_ASKING,
    NOT_A_MEMBER,
    NO_PICTURE,
    UNRECOGNISED,
];
