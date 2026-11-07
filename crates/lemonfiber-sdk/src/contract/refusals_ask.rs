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
