/// `FORM-1`: Raised when no form was named.
pub const NO_FORM_NAMED: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-1",
    name: "NO_FORM_NAMED",
    status: 400,
    description: "Raised when no form was named.",
};

/// `FORM-2`: Raised when a named form is not declared by the stack.
pub const NO_SUCH_FORM: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-2",
    name: "NO_SUCH_FORM",
    status: 404,
    description: "Raised when a named form is not declared by the stack.",
};

/// `FORM-3`: Raised when forms that cannot be combined are named together.
pub const FORMS_CONFLICT: crate::RefusalCode = crate::RefusalCode {
    code: "FORM-3",
    name: "FORMS_CONFLICT",
    status: 400,
    description: "Raised when forms that cannot be combined are named together.",
};
