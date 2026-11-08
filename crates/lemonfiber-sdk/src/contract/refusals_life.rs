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

/// `LIFE-15`: Raised where a start would install a part whose install could not be read.
pub const OFFLINE_INSTALL_UNREAD: crate::RefusalCode = crate::RefusalCode {
    code: "LIFE-15",
    name: "OFFLINE_INSTALL_UNREAD",
    status: 409,
    description: "Raised where a start would install a part whose install could not be read.",
};
