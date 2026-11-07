//! The actions a key may call, and what the contract says of each.

/// Every action a key may call.
pub const ALL: &[crate::KeyCallable] = &[
    crate::KeyCallable {
        action: "restart",
        disturbs: true,
        rehearsal: true,
        idempotent: true,
        moved: Some(&super::refusals::RESTART_MOVED),
    },
    crate::KeyCallable {
        action: "diagnose",
        disturbs: true,
        rehearsal: false,
        idempotent: false,
        moved: None,
    },
    crate::KeyCallable {
        action: "update",
        disturbs: true,
        rehearsal: true,
        idempotent: false,
        moved: Some(&super::refusals::UPDATE_MOVED),
    },
    crate::KeyCallable {
        action: "downloads-pause",
        disturbs: false,
        rehearsal: true,
        idempotent: true,
        moved: Some(&super::refusals::PAUSING_MOVED),
    },
    crate::KeyCallable {
        action: "downloads-resume",
        disturbs: false,
        rehearsal: true,
        idempotent: true,
        moved: Some(&super::refusals::PAUSING_MOVED),
    },
];
