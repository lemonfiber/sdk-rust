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
