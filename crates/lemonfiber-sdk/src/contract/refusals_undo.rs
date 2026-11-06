/// `UNDO-5`: Raised when a run moved one of lemonfiber's own guards onto its plugin, which no undo or removal puts back.
pub const MOVED_FOR_GOOD: crate::RefusalCode = crate::RefusalCode {
code: "UNDO-5",
name: "MOVED_FOR_GOOD",
status: 409,
description: "Raised when a run moved one of lemonfiber's own guards onto its plugin, which no undo or removal puts back.",
};
