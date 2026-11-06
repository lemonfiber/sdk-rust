/// `MIGRATE-1`: Raised when a replacement was agreed to for an offer that is not the one standing now.
pub const OFFER_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "MIGRATE-1",
    name: "OFFER_MOVED",
    status: 400,
    description:
        "Raised when a replacement was agreed to for an offer that is not the one standing now.",
};
