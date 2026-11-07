/// `DEVICE-1`: Raised when a device key is asked for under a word that cannot name one.
pub const NOT_A_DEVICE_NAME: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-1",
    name: "NOT_A_DEVICE_NAME",
    status: 400,
    description: "Raised when a device key is asked for under a word that cannot name one.",
};

/// `DEVICE-2`: Raised when a device key is registered under a name another device key holds.
pub const DEVICE_NAME_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-2",
    name: "DEVICE_NAME_TAKEN",
    status: 409,
    description: "Raised when a device key is registered under a name another device key holds.",
};

/// `DEVICE-3`: Raised when a device key is registered with a public half another device key holds.
pub const PUBLIC_HALF_TAKEN: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-3",
    name: "PUBLIC_HALF_TAKEN",
    status: 409,
    description:
        "Raised when a device key is registered with a public half another device key holds.",
};

/// `DEVICE-4`: Raised when the public half given is not a P-256 key.
pub const NOT_P256: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-4",
    name: "NOT_P256",
    status: 400,
    description: "Raised when the public half given is not a P-256 key.",
};

/// `DEVICE-5`: Raised when a device key is registered with no attestation.
pub const NO_ATTESTATION: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-5",
    name: "NO_ATTESTATION",
    status: 400,
    description: "Raised when a device key is registered with no attestation.",
};

/// `DEVICE-6`: Raised when an attestation does not verify, or does not cover the public half.
pub const ATTESTATION_UNVERIFIED: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-6",
    name: "ATTESTATION_UNVERIFIED",
    status: 400,
    description: "Raised when an attestation does not verify, or does not cover the public half.",
};

/// `DEVICE-7`: Raised when an attestation places the key in software, or says it was not made in the hardware that keeps it.
pub const IN_SOFTWARE: crate::RefusalCode = crate::RefusalCode {
code: "DEVICE-7",
name: "IN_SOFTWARE",
status: 400,
description: "Raised when an attestation places the key in software, or says it was not made in the hardware that keeps it.",
};

/// `DEVICE-8`: Raised when an attestation names an app this stack does not trust.
pub const UNTRUSTED_APP: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-8",
    name: "UNTRUSTED_APP",
    status: 403,
    description: "Raised when an attestation names an app this stack does not trust.",
};

/// `DEVICE-9`: Raised when a device key named is not registered, or was revoked.
pub const NOT_REGISTERED: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-9",
    name: "NOT_REGISTERED",
    status: 404,
    description: "Raised when a device key named is not registered, or was revoked.",
};

/// `DEVICE-10`: Raised when a mint carries a challenge that does not stand.
pub const CHALLENGE_NOT_STANDING: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-10",
    name: "CHALLENGE_NOT_STANDING",
    status: 403,
    description: "Raised when a mint carries a challenge that does not stand.",
};

/// `DEVICE-11`: Raised when a mint's signature does not verify against the device key.
pub const SIGNATURE_WRONG: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-11",
    name: "SIGNATURE_WRONG",
    status: 401,
    description: "Raised when a mint's signature does not verify against the device key.",
};

/// `DEVICE-12`: Raised when this machine would not supply the bytes a challenge is made of.
pub const NO_CHALLENGE: crate::RefusalCode = crate::RefusalCode {
    code: "DEVICE-12",
    name: "NO_CHALLENGE",
    status: 500,
    description: "Raised when this machine would not supply the bytes a challenge is made of.",
};

/// `DEVICE-13`: Raised when an attestation's chain carries a certificate Google lists as revoked or suspended.
pub const ATTESTATION_REVOKED: crate::RefusalCode = crate::RefusalCode {
code: "DEVICE-13",
name: "ATTESTATION_REVOKED",
status: 403,
description: "Raised when an attestation's chain carries a certificate Google lists as revoked or suspended.",
};
