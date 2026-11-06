/// `KEY-1`: Raised when a key is asked for under a word that cannot name one.
pub const BAD_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-1",
    name: "BAD_NAME",
    status: 400,
    description: "Raised when a key is asked for under a word that cannot name one.",
};

/// `KEY-2`: Raised when a key is asked for with a scope that is none of the three.
pub const NOT_A_SCOPE: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-2",
    name: "NOT_A_SCOPE",
    status: 400,
    description: "Raised when a key is asked for with a scope that is none of the three.",
};

/// `KEY-3`: Raised when a key is asked for with a purpose that is none of the three.
pub const NOT_A_PURPOSE: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-3",
    name: "NOT_A_PURPOSE",
    status: 400,
    description: "Raised when a key is asked for with a purpose that is none of the three.",
};

/// `KEY-5`: Raised when a key is asked for under a name another key holds.
pub const NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-5",
    name: "NAME_TAKEN",
    status: 400,
    description: "Raised when a key is asked for under a name another key holds.",
};

/// `KEY-6`: Raised when a member's key names an account the household does not hold.
pub const NO_SUCH_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-6",
    name: "NO_SUCH_MEMBER",
    status: 404,
    description: "Raised when a member's key names an account the household does not hold.",
};

/// `KEY-8`: Raised when a household member asks for a key, or a revoke, that is not theirs alone.
pub const NOT_FOR_YOURSELF: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-8",
    name: "NOT_FOR_YOURSELF",
    status: 400,
    description:
        "Raised when a household member asks for a key, or a revoke, that is not theirs alone.",
};

/// `KEY-10`: Raised when no active key holds the name a revoke gave.
pub const NO_SUCH_KEY: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-10",
    name: "NO_SUCH_KEY",
    status: 404,
    description: "Raised when no active key holds the name a revoke gave.",
};

/// `KEY-11`: Raised when a household member asks to mint a key and the operator has not allowed members to.
pub const MEMBERS_MAY_NOT_MINT: crate::RefusalCode = crate::RefusalCode {
code: "KEY-11",
name: "MEMBERS_MAY_NOT_MINT",
status: 409,
description: "Raised when a household member asks to mint a key and the operator has not allowed members to.",
};

/// `KEY-12`: Raised when a key is asked to be collected with a claim that collects nothing.
pub const CLAIM_NOT_OPEN: crate::RefusalCode = crate::RefusalCode {
    code: "KEY-12",
    name: "CLAIM_NOT_OPEN",
    status: 401,
    description: "Raised when a key is asked to be collected with a claim that collects nothing.",
};

/// `KEY-13`: Raised when a key is asked to be minted as a claim and no address another machine reaches can go in its link.
pub const UNLINKABLE: crate::RefusalCode = crate::RefusalCode {
code: "KEY-13",
name: "UNLINKABLE",
status: 409,
description: "Raised when a key is asked to be minted as a claim and no address another machine reaches can go in its link.",
};
