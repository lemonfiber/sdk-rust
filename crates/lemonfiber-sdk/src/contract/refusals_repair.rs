/// `REPAIR-1`: Raised when consent was given for an offer that no longer stands.
pub const STALE: crate::RefusalCode = crate::RefusalCode {
    code: "REPAIR-1",
    name: "STALE",
    status: 400,
    description: "Raised when consent was given for an offer that no longer stands.",
};
