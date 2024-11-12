#[macro_export]
macro_rules! define_internal_error {
    ($name:ident, $msg:expr) => {
        define_internal_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // Since internal errors usually indicate more serious issues, use
        // expensive Backtrace::force_capture() to build context string, to
        // facilitate manual debugging.
        impl $name {
            #[allow(dead_code)]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                Box::new($name {
                    context: std::backtrace::Backtrace::force_capture().to_string(),
                    message: format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                Box::new($name {
                    context: std::backtrace::Backtrace::force_capture().to_string(),
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::ReturnInternalServerError
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! define_critical_error {
    ($name:ident, $msg:expr) => {
        define_critical_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // Since internal errors usually indicate more serious issues, use
        // expensive Backtrace::force_capture() to build context string, to
        // facilitate manual debugging.
        impl $name {
            #[allow(dead_code)]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                Box::new($name {
                    context: std::backtrace::Backtrace::force_capture().to_string(),
                    message: "CRITICAL; ".to_string() + &format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                Box::new($name {
                    context: std::backtrace::Backtrace::force_capture().to_string(),
                    message: "CRITICAL; ".to_string() + &format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::ReturnInternalServerError
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! define_client_error {
    ($name:ident, $msg:expr) => {
        define_client_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // This error type is usually less serious, and mainly indicates an
        // issue with client code (not server code), so use cheaper
        // Location::caller() instead of Backtrace::force_capture() to build
        // context string.
        impl $name {
            #[allow(dead_code)]
            #[track_caller]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            #[track_caller]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::LogErrorSendFixedMsgToClient($crate::CLIENT_ERROR_MSG)
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! define_sensitive_error {
    ($name:ident, $msg:expr) => {
        define_sensitive_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // To avoid leaking implementation details for sensitive errors, don't
        // provide execution context.
        impl $name {
            #[allow(dead_code)]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                Box::new($name {
                    context: "OMITTED".to_string(),
                    message: format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                Box::new($name {
                    context: "OMITTED".to_string(),
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::LogErrorSendFixedMsgToClient($crate::SENSITIVE_ERROR_MSG)
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! define_user_error {
    ($name:ident, $msg:expr) => {
        define_user_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // Since error type is usually not indicative of an error with the code,
        // use cheaper Location::caller() instead of Backtrace::force_capture()
        // to build context string.
        impl $name {
            #[allow(dead_code)]
            #[track_caller]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            #[track_caller]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::LogWarningForwardToClient
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}

#[macro_export]
macro_rules! define_temporary_error {
    ($name:ident, $msg:expr) => {
        define_temporary_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            #[allow(dead_code)]
            context: String,
            message: String,
            debug: Option<String>,
        }

        // Since error type is usually not indicative of an error with the code,
        // use cheaper Location::caller() instead of Backtrace::force_capture()
        // to build context string.
        impl $name {
            #[allow(dead_code)]
            #[track_caller]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: None,
                })
            }
            #[allow(dead_code)]
            #[track_caller]
            pub fn with_debug<D>(
                $($arg: $argtype,)*
                debug: &D,
            ) -> $crate::ServerError where D: std::fmt::Debug {
                let location = std::panic::Location::caller();
                Box::new($name {
                    context: format!("{}; {};", location.file(), location.line()),
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $crate::ServerErrorBehaviour::LogWarningForwardToClient
            }
            fn message(&self) -> &String {
                &self.message
            }
            fn debug(&self) -> Option<&String> {
                self.debug.as_ref()
            }
        }
    };
}
