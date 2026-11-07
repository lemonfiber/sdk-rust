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
