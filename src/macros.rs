#[macro_export]
macro_rules! define_server_error {
    (
        $name:ident,
        $msg:expr,
        { $($arg:ident : $argtype:ty),* $(,)? },
        $context_type:expr,
        $behaviour:expr,
        $tag:expr
    ) => {
        #[derive(Debug)]
        pub struct $name {
            context: String,
            message: String,
            debug: Option<String>,
        }

        impl $name {
            #[allow(dead_code)]
            #[track_caller]
            pub fn new($($arg: $argtype),*) -> $crate::ServerError {
                let context = match $context_type {
                    $crate::ServerErrorContext::Omit => "OMITTED".to_string(),
                    $crate::ServerErrorContext::Location => {
                        let location = std::panic::Location::caller();
                        format!("{}; {};", location.file(), location.line())
                    }
                    $crate::ServerErrorContext::Backtrace => {
                        std::backtrace::Backtrace::force_capture().to_string()
                    }
                };
                Box::new($name {
                    context,
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
                let context = match $context_type {
                    $crate::ServerErrorContext::Omit => "OMITTED".to_string(),
                    $crate::ServerErrorContext::Location => {
                        let location = std::panic::Location::caller();
                        format!("{}; {};", location.file(), location.line())
                    }
                    $crate::ServerErrorContext::Backtrace => {
                        std::backtrace::Backtrace::force_capture().to_string()
                    }
                };
                Box::new($name {
                    context,
                    message: format!($msg, $($arg = $arg),*),
                    debug: Some(format!("{:?}", debug)),
                })
            }
        }

        impl $crate::ServerErrorTrait for $name {
            fn behaviour(&self) -> $crate::ServerErrorBehaviour {
                $behaviour
            }
            fn tag(&self) -> $crate::ServerErrorTag {
                $tag
            }
            fn context(&self) -> &String {
                &self.context
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
macro_rules! define_internal_error {
    ($name:ident, $msg:expr) => {
        define_internal_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // Since internal errors usually indicate more serious issues, use
            // expensive Backtrace::force_capture() to build context string, to
            // facilitate manual debugging.
            $crate::ServerErrorContext::Backtrace,
            $crate::ServerErrorBehaviour::ReturnInternalServerError,
            $crate::ServerErrorTag::None
        );
    };
}

#[macro_export]
macro_rules! define_critical_error {
    ($name:ident, $msg:expr) => {
        define_critical_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // Since critical errors indicate serious and rare issues, use
            // expensive Backtrace::force_capture() to build context string.
            $crate::ServerErrorContext::Backtrace,
            $crate::ServerErrorBehaviour::ReturnInternalServerError,
            $crate::ServerErrorTag::Critical
        );
    };
}

#[macro_export]
macro_rules! define_client_error {
    ($name:ident, $msg:expr) => {
        define_client_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // This error type is usually less serious, and mainly indicates an
            // issue with client code (not server code), so use cheaper
            // Location::caller() instead of Backtrace::force_capture() to build
            // context string.
            $crate::ServerErrorContext::Location,
            $crate::ServerErrorBehaviour::LogErrorSendFixedMsgToClient($crate::CLIENT_ERROR_MSG),
            $crate::ServerErrorTag::None
        );
    };
}

#[macro_export]
macro_rules! define_sensitive_error {
    ($name:ident, $msg:expr) => {
        define_sensitive_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // To avoid leaking implementation details for sensitive errors,
            // don't provide execution context.
            $crate::ServerErrorContext::Omit,
            $crate::ServerErrorBehaviour::LogErrorSendFixedMsgToClient($crate::SENSITIVE_ERROR_MSG),
            $crate::ServerErrorTag::None
        );
    };
}

#[macro_export]
macro_rules! define_user_error {
    ($name:ident, $msg:expr) => {
        define_user_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // This error type is usually not indicative of an error with the
            // code, so use cheaper Location::caller() to build context string.
            $crate::ServerErrorContext::Location,
            $crate::ServerErrorBehaviour::LogWarningForwardToClient,
            $crate::ServerErrorTag::None
        );
    };
}

#[macro_export]
macro_rules! define_temporary_error {
    ($name:ident, $msg:expr) => {
        define_temporary_error!($name, $msg, {});
    };
    ($name:ident, $msg:expr, { $($arg:ident : $argtype:ty),* $(,)? }) => {
        $crate::define_server_error!(
            $name,
            $msg,
            { $($arg : $argtype),* },
            // This error type is usually not indicative of an error with the
            // code, so use cheaper Location::caller() to build context string.
            $crate::ServerErrorContext::Location,
            $crate::ServerErrorBehaviour::LogWarningForwardToClient,
            $crate::ServerErrorTag::None
        );
    };
}
