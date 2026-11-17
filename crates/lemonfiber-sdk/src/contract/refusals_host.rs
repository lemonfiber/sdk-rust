/// `HOST-7`: Raised when the service named to run on its schedule is no run-and-exit service.
pub const NOT_SCHEDULED: crate::RefusalCode = crate::RefusalCode {
    code: "HOST-7",
    name: "NOT_SCHEDULED",
    status: 404,
    description: "Raised when the service named to run on its schedule is no run-and-exit service.",
};
