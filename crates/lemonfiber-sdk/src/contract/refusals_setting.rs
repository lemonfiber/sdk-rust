/// `SETTING-1`: Raised where nothing in this stack reads a setting by the name given.
pub const NO_SUCH_SETTING: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-1",
    name: "NO_SUCH_SETTING",
    status: 404,
    description: "Raised where nothing in this stack reads a setting by the name given.",
};

/// `SETTING-2`: Raised where the name given is a secret's, which this command never writes.
pub const A_SECRET: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-2",
    name: "A_SECRET",
    status: 400,
    description: "Raised where the name given is a secret's, which this command never writes.",
};

/// `SETTING-3`: Raised where the name given belongs to the other of the two commands that write settings.
pub const TAKEN_ELSEWHERE: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-3",
    name: "TAKEN_ELSEWHERE",
    status: 400,
    description:
        "Raised where the name given belongs to the other of the two commands that write settings.",
};

/// `SETTING-4`: Raised where a value cannot be written as the setting asked for.
pub const UNWRITABLE_VALUE: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-4",
    name: "UNWRITABLE_VALUE",
    status: 400,
    description: "Raised where a value cannot be written as the setting asked for.",
};

/// `SETTING-5`: Raised when a change answers an offer read against settings that have since moved.
pub const SETTINGS_MOVED: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-5",
    name: "SETTINGS_MOVED",
    status: 400,
    description:
        "Raised when a change answers an offer read against settings that have since moved.",
};

/// `SETTING-6`: Raised where the setting already holds what the change asks for.
pub const ALREADY_HOLDS: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-6",
    name: "ALREADY_HOLDS",
    status: 400,
    description: "Raised where the setting already holds what the change asks for.",
};

/// `SETTING-7`: Raised where a change would leave a setting holding a value outside its choices.
pub const OUTSIDE_ITS_CHOICES: crate::RefusalCode = crate::RefusalCode {
    code: "SETTING-7",
    name: "OUTSIDE_ITS_CHOICES",
    status: 400,
    description: "Raised where a change would leave a setting holding a value outside its choices.",
};

/// `SETTING-9`: Raised where `PLEX_LINK_MODE` would enter or leave `owner` for a request service already set up.
pub const OWNER_SET_UP: crate::RefusalCode = crate::RefusalCode {
code: "SETTING-9",
name: "OWNER_SET_UP",
status: 409,
description: "Raised where `PLEX_LINK_MODE` would enter or leave `owner` for a request service already set up.",
};
