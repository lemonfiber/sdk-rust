/// `READ-1`: Raised where a read was given a parameter its answer has nowhere to put.
pub const UNWANTED: crate::RefusalCode = crate::RefusalCode {
    code: "READ-1",
    name: "UNWANTED",
    status: 400,
    description: "Raised where a read was given a parameter its answer has nowhere to put.",
};

/// `READ-2`: Raised where a parameter carrying one value was given more than once.
pub const REPEATED: crate::RefusalCode = crate::RefusalCode {
    code: "READ-2",
    name: "REPEATED",
    status: 400,
    description: "Raised where a parameter carrying one value was given more than once.",
};

/// `READ-3`: Raised where no read goes by the name that was asked for.
pub const NO_SUCH_READ: crate::RefusalCode = crate::RefusalCode {
    code: "READ-3",
    name: "NO_SUCH_READ",
    status: 404,
    description: "Raised where no read goes by the name that was asked for.",
};

/// `READ-4`: Raised where a trace was asked for and named nothing to follow.
pub const NO_TERM: crate::RefusalCode = crate::RefusalCode {
    code: "READ-4",
    name: "NO_TERM",
    status: 400,
    description: "Raised where a trace was asked for and named nothing to follow.",
};

/// `READ-5`: Raised where the season to narrow a trace to is not a number.
pub const NOT_A_SEASON: crate::RefusalCode = crate::RefusalCode {
    code: "READ-5",
    name: "NOT_A_SEASON",
    status: 400,
    description: "Raised where the season to narrow a trace to is not a number.",
};

/// `READ-6`: Raised where a setting was asked for by an empty name.
pub const NO_SETTING: crate::RefusalCode = crate::RefusalCode {
    code: "READ-6",
    name: "NO_SETTING",
    status: 400,
    description: "Raised where a setting was asked for by an empty name.",
};

/// `READ-7`: Raised where a household member was asked for by an empty name.
pub const NO_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "READ-7",
    name: "NO_MEMBER",
    status: 400,
    description: "Raised where a household member was asked for by an empty name.",
};

/// `READ-8`: Raised where a shelf was asked for and nobody was named whose it is.
pub const NO_SHELF_WITHOUT_A_MEMBER: crate::RefusalCode = crate::RefusalCode {
    code: "READ-8",
    name: "NO_SHELF_WITHOUT_A_MEMBER",
    status: 400,
    description: "Raised where a shelf was asked for and nobody was named whose it is.",
};

/// `READ-9`: Raised where how many holdings to answer with is not a whole number.
pub const NOT_A_COUNT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-9",
    name: "NOT_A_COUNT",
    status: 400,
    description: "Raised where how many holdings to answer with is not a whole number.",
};

/// `READ-10`: Raised where more holdings were asked for than one read answers with.
pub const TOO_MANY_AT_ONCE: crate::RefusalCode = crate::RefusalCode {
    code: "READ-10",
    name: "TOO_MANY_AT_ONCE",
    status: 400,
    description: "Raised where more holdings were asked for than one read answers with.",
};

/// `READ-11`: Raised where a diagnosis was narrowed to a group or check that is not one.
pub const NO_SUCH_GROUP: crate::RefusalCode = crate::RefusalCode {
    code: "READ-11",
    name: "NO_SUCH_GROUP",
    status: 400,
    description: "Raised where a diagnosis was narrowed to a group or check that is not one.",
};

/// `READ-12`: Raised where a removal was named that is none of the four there are.
pub const NO_SUCH_REMOVAL: crate::RefusalCode = crate::RefusalCode {
    code: "READ-12",
    name: "NO_SUCH_REMOVAL",
    status: 400,
    description: "Raised where a removal was named that is none of the four there are.",
};

/// `READ-13`: Raised where moving forward was asked about and neither stack nor self named.
pub const NO_UPDATE_OBJECT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-13",
    name: "NO_UPDATE_OBJECT",
    status: 400,
    description: "Raised where moving forward was asked about and neither stack nor self named.",
};

/// `READ-14`: Raised where how many log lines to begin with is not a number within the ceiling.
pub const NOT_A_LINE_COUNT: crate::RefusalCode = crate::RefusalCode {
    code: "READ-14",
    name: "NOT_A_LINE_COUNT",
    status: 400,
    description:
        "Raised where how many log lines to begin with is not a number within the ceiling.",
};

/// `READ-15`: Raised where a parameter that takes a yes or a no is neither true nor false.
pub const NOT_A_CHOICE: crate::RefusalCode = crate::RefusalCode {
    code: "READ-15",
    name: "NOT_A_CHOICE",
    status: 400,
    description: "Raised where a parameter that takes a yes or a no is neither true nor false.",
};

/// `READ-16`: Raised where a household read named a member and asked for the household's defaults as well.
pub const MEMBER_AND_DEFAULTS: crate::RefusalCode = crate::RefusalCode {
code: "READ-16",
name: "MEMBER_AND_DEFAULTS",
status: 400,
description: "Raised where a household read named a member and asked for the household's defaults as well.",
};
