/// `SPACE-2`: Raised when there is no data location to measure.
pub const NOWHERE_TO_MEASURE: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-2",
    name: "NOWHERE_TO_MEASURE",
    status: 400,
    description: "Raised when there is no data location to measure.",
};

/// `SPACE-4`: Raised when there is no torrent client here to be holding a completed download.
pub const NOTHING_TO_ASK: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-4",
    name: "NOTHING_TO_ASK",
    status: 400,
    description: "Raised when there is no torrent client here to be holding a completed download.",
};

/// `SPACE-5`: Raised when the client answers and is holding nothing of the name given.
pub const NOT_HELD: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-5",
    name: "NOT_HELD",
    status: 400,
    description: "Raised when the client answers and is holding nothing of the name given.",
};

/// `SPACE-6`: Raised when an agreement names an offer that is not the one standing now.
pub const ANOTHER_OFFER: crate::RefusalCode = crate::RefusalCode {
    code: "SPACE-6",
    name: "ANOTHER_OFFER",
    status: 400,
    description: "Raised when an agreement names an offer that is not the one standing now.",
};
