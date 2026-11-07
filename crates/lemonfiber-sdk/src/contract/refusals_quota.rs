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
