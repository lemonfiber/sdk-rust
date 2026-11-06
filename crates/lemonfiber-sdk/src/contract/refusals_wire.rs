/// `WIRE-1`: A capability was named that no service in this stack provides.
pub const NO_SUCH_FILLER: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-1",
    name: "NO_SUCH_FILLER",
    status: 404,
    description: "A capability was named that no service in this stack provides.",
};

/// `WIRE-2`: The service named cannot do the thing it was asked to fill.
pub const CANNOT_FILL: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-2",
    name: "CANNOT_FILL",
    status: 400,
    description: "The service named cannot do the thing it was asked to fill.",
};

/// `WIRE-3`: Nothing in this stack asks for the capability, so a choice would change nothing.
pub const NOTHING_ASKS: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-3",
    name: "NOTHING_ASKS",
    status: 400,
    description: "Nothing in this stack asks for the capability, so a choice would change nothing.",
};

/// `WIRE-4`: The setting recording the choice could not be written.
pub const CHOICE_UNWRITABLE: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-4",
    name: "CHOICE_UNWRITABLE",
    status: 500,
    description: "The setting recording the choice could not be written.",
};

/// `WIRE-5`: Raised when a choice answers an offer that was read against a wiring that has since moved.
pub const WIRING_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "WIRE-5",
    name: "WIRING_MOVED",
    status: 400,
    description:
        "Raised when a choice answers an offer that was read against a wiring that has since moved.",
};

/// `WIRE-6`: Raised when the reason given for a choice is longer than a reason may be, or holds a line break or another control character.
pub const UNREASONABLE: crate::RefusalCode = crate::RefusalCode {
code: "WIRE-6",
name: "UNREASONABLE",
status: 400,
description: "Raised when the reason given for a choice is longer than a reason may be, or holds a line break or another control character.",
};

/// `WIRE-7`: Raised where the service chosen already fills the capability, so there is nothing to change.
pub const ALREADY_FILLS: crate::RefusalCode = crate::RefusalCode {
code: "WIRE-7",
name: "ALREADY_FILLS",
status: 400,
description: "Raised where the service chosen already fills the capability, so there is nothing to change.",
};
