/// `STORAGE-9`: Raised where `DATA_ROOT` names an NFS export lemonfiber cannot write into the mount options, so no project is written from it.
pub const ROOT_REFUSED: crate::RefusalCode = crate::RefusalCode {
code: "STORAGE-9",
name: "ROOT_REFUSED",
status: 409,
description: "Raised where `DATA_ROOT` names an NFS export lemonfiber cannot write into the mount options, so no project is written from it.",
};
