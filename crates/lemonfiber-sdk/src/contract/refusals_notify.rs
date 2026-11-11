/// `NOTIFY-1`: Raised when a back-end is added under a word that cannot name one.
pub const NOT_A_BACKEND_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-1",
    name: "NOT_A_BACKEND_NAME",
    status: 400,
    description: "Raised when a back-end is added under a word that cannot name one.",
};

/// `NOTIFY-2`: Raised when a back-end is added under a name another holds.
pub const BACKEND_NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-2",
    name: "BACKEND_NAME_TAKEN",
    status: 409,
    description: "Raised when a back-end is added under a name another holds.",
};

/// `NOTIFY-3`: Raised when no back-end holds the name asked about.
pub const NO_SUCH_BACKEND: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-3",
    name: "NO_SUCH_BACKEND",
    status: 404,
    description: "Raised when no back-end holds the name asked about.",
};

/// `NOTIFY-4`: Raised when a back-end on a host that is not the household's own is added without saying so.
pub const HOSTED_UNACKNOWLEDGED: crate::RefusalCode = crate::RefusalCode {
code: "NOTIFY-4",
name: "HOSTED_UNACKNOWLEDGED",
status: 400,
description: "Raised when a back-end on a host that is not the household's own is added without saying so.",
};

/// `NOTIFY-5`: Raised when a back-end's address is not one it can be reached at.
pub const NOT_A_BACKEND_ADDRESS: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-5",
    name: "NOT_A_BACKEND_ADDRESS",
    status: 400,
    description: "Raised when a back-end's address is not one it can be reached at.",
};

/// `NOTIFY-6`: Raised when a back-end is added without what its kind needs.
pub const BACKEND_INCOMPLETE: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-6",
    name: "BACKEND_INCOMPLETE",
    status: 400,
    description: "Raised when a back-end is added without what its kind needs.",
};

/// `NOTIFY-7`: Raised when a back-end's attempts and delays do not agree.
pub const NOT_A_RETRY_SCHEDULE: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-7",
    name: "NOT_A_RETRY_SCHEDULE",
    status: 400,
    description: "Raised when a back-end's attempts and delays do not agree.",
};

/// `NOTIFY-8`: Raised when a member chooses a back-end that is not offered to members.
pub const NOT_OFFERED: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-8",
    name: "NOT_OFFERED",
    status: 400,
    description: "Raised when a member chooses a back-end that is not offered to members.",
};

/// `NOTIFY-9`: Raised when a phone confirms a wake nobody sent it.
pub const NO_SUCH_WAKE: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-9",
    name: "NO_SUCH_WAKE",
    status: 404,
    description: "Raised when a phone confirms a wake nobody sent it.",
};

/// `NOTIFY-10`: Raised when an event arrives without the key the core gave its sender.
pub const EVENTS_REFUSED: crate::RefusalCode = crate::RefusalCode {
    code: "NOTIFY-10",
    name: "EVENTS_REFUSED",
    status: 401,
    description: "Raised when an event arrives without the key the core gave its sender.",
};
