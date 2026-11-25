//! The kinds the event stream carries, and how often it speaks when it has nothing to say.

/// The most seconds the stream stays silent before it sends a heartbeat.
pub const HEARTBEAT_SECONDS: u64 = 15;

/// Every kind the event stream carries.
pub const ALL: &[crate::EventKind] = &[
    crate::EventKind {
        kind: "alert",
        replaces: false,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "copying",
        replaces: true,
        audience: crate::Audience::OperatorAndKeys,
    },
    crate::EventKind {
        kind: "dashboard",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "held",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "household",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "log",
        replaces: false,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "news",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "playing",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "plugins",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "pull",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "replay",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "settings",
        replaces: true,
        audience: crate::Audience::Operator,
    },
    crate::EventKind {
        kind: "start",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "step",
        replaces: false,
        audience: crate::Audience::Anyone,
    },
    crate::EventKind {
        kind: "wiring",
        replaces: true,
        audience: crate::Audience::Anyone,
    },
];
