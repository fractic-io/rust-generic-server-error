use crate::{define_internal_error_type, GenericServerError, GenericServerErrorTrait};

// Define a couple general-purpose error types.
// --------------------------------------------------

// For unexpected, unrecoverable errors:
define_internal_error_type!(CriticalError, "CRITICAL");
