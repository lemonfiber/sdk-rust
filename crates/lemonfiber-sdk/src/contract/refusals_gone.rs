/// `GONE-2`: Raised when an agreement names a reading of this machine that is not the one standing now.
pub const ANOTHER_READING: crate::RefusalCode = crate::RefusalCode {
    code: "GONE-2",
    name: "ANOTHER_READING",
    status: 400,
    description:
        "Raised when an agreement names a reading of this machine that is not the one standing now.",
};
