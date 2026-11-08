/// `PUSH-1`: Raised when a push endpoint is registered under a word that cannot name one.
pub const NOT_A_PUSH_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "PUSH-1",
    name: "NOT_A_PUSH_NAME",
    status: 400,
    description: "Raised when a push endpoint is registered under a word that cannot name one.",
};

/// `PUSH-2`: Raised when a push endpoint is registered under a name another holds.
pub const PUSH_NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "PUSH-2",
    name: "PUSH_NAME_TAKEN",
    status: 409,
    description: "Raised when a push endpoint is registered under a name another holds.",
};

/// `PUSH-3`: Raised when a push endpoint's address is not an `https` address.
pub const PUSH_NOT_HTTPS: crate::RefusalCode = crate::RefusalCode {
    code: "PUSH-3",
    name: "PUSH_NOT_HTTPS",
    status: 400,
    description: "Raised when a push endpoint's address is not an `https` address.",
};

/// `PUSH-4`: Raised when the keys a push endpoint gave cannot be encrypted to.
pub const PUSH_KEYS_UNREADABLE: crate::RefusalCode = crate::RefusalCode {
    code: "PUSH-4",
    name: "PUSH_KEYS_UNREADABLE",
    status: 400,
    description: "Raised when the keys a push endpoint gave cannot be encrypted to.",
};

/// `PUSH-5`: Raised when no push endpoint holds the name asked about.
pub const PUSH_NOT_REGISTERED: crate::RefusalCode = crate::RefusalCode {
    code: "PUSH-5",
    name: "PUSH_NOT_REGISTERED",
    status: 404,
    description: "Raised when no push endpoint holds the name asked about.",
};
