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
