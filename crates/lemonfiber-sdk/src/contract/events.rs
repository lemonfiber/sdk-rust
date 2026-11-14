//! The kinds the event stream carries, and how often it speaks when it has nothing to say.

/// The most seconds the stream stays silent before it sends a heartbeat.
pub const HEARTBEAT_SECONDS: u64 = 15;

/// Every kind the event stream carries.
pub const ALL: &[crate::EventKind] = &[
    crate::EventKind {
        kind: "alert",
        replaces: false,
        operator: false,
    },
    crate::EventKind {
        kind: "dashboard",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "held",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "household",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "log",
        replaces: false,
        operator: false,
    },
    crate::EventKind {
        kind: "news",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "playing",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "plugins",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "pull",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "settings",
        replaces: true,
        operator: true,
    },
    crate::EventKind {
        kind: "start",
        replaces: true,
        operator: false,
    },
    crate::EventKind {
        kind: "step",
        replaces: false,
        operator: false,
    },
    crate::EventKind {
        kind: "wiring",
        replaces: true,
        operator: false,
    },
];
