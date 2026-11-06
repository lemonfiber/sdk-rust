/// `PLAY-1`: Said where the title named is not something the media server could hold.
pub const NOT_AN_ITEM: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-1",
    name: "NOT_AN_ITEM",
    status: 400,
    description: "Said where the title named is not something the media server could hold.",
};

/// `PLAY-2`: Said where the title is not one this member may watch, or is not there at all.
pub const NOT_ON_THEIR_SHELF: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-2",
    name: "NOT_ON_THEIR_SHELF",
    status: 404,
    description: "Said where the title is not one this member may watch, or is not there at all.",
};

/// `PLAY-3`: Said where what names a member's device is not a device id.
pub const NOT_A_DEVICE: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-3",
    name: "NOT_A_DEVICE",
    status: 400,
    description: "Said where what names a member's device is not a device id.",
};

/// `PLAY-4`: Said where there is no media server to play from, or it was never set up.
pub const NOTHING_TO_PLAY_FROM: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-4",
    name: "NOTHING_TO_PLAY_FROM",
    status: 503,
    description: "Said where there is no media server to play from, or it was never set up.",
};

/// `PLAY-5`: Said where the media server would not answer for a member.
pub const SERVER_SILENT: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-5",
    name: "SERVER_SILENT",
    status: 502,
    description: "Said where the media server would not answer for a member.",
};

/// `PLAY-6`: Said where nobody is named for something only a member can be.
pub const NOBODY_NAMED: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-6",
    name: "NOBODY_NAMED",
    status: 400,
    description: "Said where nobody is named for something only a member can be.",
};

/// `PLAY-7`: Said where the member named is not somebody in the household.
pub const NOT_IN_THE_HOUSEHOLD: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-7",
    name: "NOT_IN_THE_HOUSEHOLD",
    status: 404,
    description: "Said where the member named is not somebody in the household.",
};

/// `PLAY-8`: Said where the media server will not sign a device in by code.
pub const SIGNS_NO_DEVICE_IN: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-8",
    name: "SIGNS_NO_DEVICE_IN",
    status: 503,
    description: "Said where the media server will not sign a device in by code.",
};

/// `PLAY-9`: Said where a title on the member's shelf has no picture of the kind asked for.
pub const NO_SUCH_PICTURE: crate::RefusalCode = crate::RefusalCode {
    code: "PLAY-9",
    name: "NO_SUCH_PICTURE",
    status: 404,
    description: "Said where a title on the member's shelf has no picture of the kind asked for.",
};
