use crate::define_internal_error;

// Define a couple general-purpose error types.
// --------------------------------------------------

// For unexpected, unrecoverable errors:
define_internal_error!(CriticalError, "CRITICAL");
