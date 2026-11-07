/// `REHEARSE-1`: The flag cannot be honoured by this command, and never will be.
pub const CANNOT: crate::RefusalCode = crate::RefusalCode {
    code: "REHEARSE-1",
    name: "CANNOT",
    status: 400,
    description: "The flag cannot be honoured by this command, and never will be.",
};

/// `REHEARSE-2`: The flag is not honoured by this command yet.
pub const NOT_YET: crate::RefusalCode = crate::RefusalCode {
    code: "REHEARSE-2",
    name: "NOT_YET",
    status: 400,
    description: "The flag is not honoured by this command yet.",
};
