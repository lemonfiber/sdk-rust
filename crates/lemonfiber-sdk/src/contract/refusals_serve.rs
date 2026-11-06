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
