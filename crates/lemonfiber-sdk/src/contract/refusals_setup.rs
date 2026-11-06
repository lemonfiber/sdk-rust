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
