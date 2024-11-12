use crate::define_critical_error;

// Define a couple general-purpose error types.
// --------------------------------------------------

// For unexpected, unrecoverable errors:
define_critical_error!(CriticalError, "Unexpected: {details}.", { details: &str });

// When process fails to spawn child threads:
define_critical_error!(MultithreadingError, "Error executing child threads.");
