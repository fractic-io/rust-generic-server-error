#[macro_export]
macro_rules! define_user_visible_error_type {
    ($name:ident, $msg:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub context: &'static str,
            pub message: &'static str,
            pub debug: Option<String>,
        }

        impl $name {
            pub fn default() -> GenericServerError {
                Box::new($name {
                    context: "",
                    message: "",
                    debug: None,
                })
            }
            pub fn new(context: &'static str, message: &'static str) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    debug: None,
                })
            }
            pub fn with_debug(
                context: &'static str,
                message: &'static str,
                debug: String,
            ) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    debug: Some(debug),
                })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $msg)
            }
        }

        impl GenericServerErrorTrait for $name {
            fn should_be_shown_to_client(&self) -> bool {
                true
            }
            fn into_std_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync> {
                self
            }
        }

        impl StdError for $name {}

        impl From<$name> for GenericServerError {
            fn from(error: $name) -> Self {
                Box::new(error)
            }
        }
    };
}

#[macro_export]
macro_rules! define_user_visible_error_type_with_visible_info {
    ($name:ident, $msg:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub context: &'static str,
            pub message: &'static str,
            pub user_visible_info: String,
            pub debug: Option<String>,
        }

        impl $name {
            pub fn default() -> GenericServerError {
                Box::new($name {
                    context: "",
                    message: "",
                    user_visible_info: "".to_string(),
                    debug: None,
                })
            }
            pub fn new(
                context: &'static str,
                message: &'static str,
                user_visible_info: String,
            ) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    user_visible_info,
                    debug: None,
                })
            }
            pub fn with_debug(
                context: &'static str,
                message: &'static str,
                user_visible_info: String,
                debug: String,
            ) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    user_visible_info,
                    debug: Some(debug),
                })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $msg, user_visible_info = self.user_visible_info)
            }
        }

        impl GenericServerErrorTrait for $name {
            fn should_be_shown_to_client(&self) -> bool {
                true
            }
            fn into_std_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync> {
                self
            }
        }

        impl StdError for $name {}

        impl From<$name> for GenericServerError {
            fn from(error: $name) -> Self {
                Box::new(error)
            }
        }
    };
}

#[macro_export]
macro_rules! define_internal_error_type {
    ($name:ident, $msg:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub context: &'static str,
            pub message: &'static str,
            pub debug: Option<String>,
        }

        impl $name {
            pub fn default() -> GenericServerError {
                Box::new($name {
                    context: "",
                    message: "",
                    debug: None,
                })
            }
            pub fn new(context: &'static str, message: &'static str) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    debug: None,
                })
            }
            pub fn with_debug(
                context: &'static str,
                message: &'static str,
                debug: String,
            ) -> GenericServerError {
                Box::new($name {
                    context,
                    message,
                    debug: Some(debug),
                })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $msg)?;
                write!(f, " | {:?}", self)
            }
        }

        impl GenericServerErrorTrait for $name {
            fn should_be_shown_to_client(&self) -> bool {
                false
            }
            fn into_std_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync> {
                self
            }
        }

        impl StdError for $name {}

        impl From<$name> for GenericServerError {
            fn from(error: $name) -> Self {
                Box::new(error)
            }
        }
    };
}

// Tests.
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::GenericServerError;
    use crate::GenericServerErrorTrait;
    use std::error::Error as StdError;
    use std::fmt;

    define_user_visible_error_type!(UserVisibleError, "User-visible error.");
    define_user_visible_error_type_with_visible_info!(
        UserVisibleErrorWithVisibleInfo,
        "User-visible error: {user_visible_info}."
    );
    define_internal_error_type!(InternalError, "Internal error.");

    #[test]
    fn test_user_visible_error() {
        let basic: GenericServerError = UserVisibleError::default();
        let normal: GenericServerError = UserVisibleError::new("cxt", "msg");
        let with_debug: GenericServerError =
            UserVisibleError::with_debug("cxt", "msg", "debug".to_string());

        // Basic (user-visible) printing:
        assert_eq!(format!("{}", basic), "User-visible error.");
        assert_eq!(format!("{}", normal), "User-visible error.");
        assert_eq!(format!("{}", with_debug), "User-visible error.");

        // Debug printing:
        assert_eq!(
            format!("{:?}", basic),
            "UserVisibleError { context: \"\", message: \"\", debug: None }"
        );
        assert_eq!(
            format!("{:?}", normal),
            "UserVisibleError { context: \"cxt\", message: \"msg\", debug: None }"
        );
        assert_eq!(
            format!("{:?}", with_debug),
            "UserVisibleError { context: \"cxt\", message: \"msg\", debug: Some(\"debug\") }"
        );

        assert!(basic.should_be_shown_to_client());
        assert!(normal.should_be_shown_to_client());
        assert!(with_debug.should_be_shown_to_client());

        // Same result after converting into Box<std_error>.
        assert_eq!(
            format!("{:?}", with_debug.into_std_error()),
            "UserVisibleError { context: \"cxt\", message: \"msg\", debug: Some(\"debug\") }"
        );
    }

    #[test]
    fn test_user_visible_error_with_visible_info() {
        let basic = UserVisibleErrorWithVisibleInfo::default();
        let normal = UserVisibleErrorWithVisibleInfo::new("cxt", "msg", "info".to_string());
        let with_debug = UserVisibleErrorWithVisibleInfo::with_debug(
            "cxt",
            "msg",
            "info".to_string(),
            "debug".to_string(),
        );

        // Basic (user-visible) printing:
        assert_eq!(format!("{}", basic), "User-visible error: .");
        assert_eq!(format!("{}", normal), "User-visible error: info.");
        assert_eq!(format!("{}", with_debug), "User-visible error: info.");

        // Debug printing:
        assert_eq!(
            format!("{:?}", basic),
            "UserVisibleErrorWithVisibleInfo { context: \"\", message: \"\", user_visible_info: \"\", debug: None }"
        );
        assert_eq!(
            format!("{:?}", normal),
            "UserVisibleErrorWithVisibleInfo { context: \"cxt\", message: \"msg\", user_visible_info: \"info\", debug: None }"
        );
        assert_eq!(
            format!("{:?}", with_debug),
            "UserVisibleErrorWithVisibleInfo { context: \"cxt\", message: \"msg\", user_visible_info: \"info\", debug: Some(\"debug\") }"
        );

        assert!(basic.should_be_shown_to_client());
        assert!(normal.should_be_shown_to_client());
        assert!(with_debug.should_be_shown_to_client());

        // Same result after converting into Box<std_error>.
        assert_eq!(
            format!("{:?}", with_debug.into_std_error()),
            "UserVisibleErrorWithVisibleInfo { context: \"cxt\", message: \"msg\", user_visible_info: \"info\", debug: Some(\"debug\") }"
        );
    }

    #[test]
    fn test_internal_error() {
        let basic = InternalError::default();
        let normal = InternalError::new("cxt", "msg");
        let with_debug = InternalError::with_debug("cxt", "msg", "debug".to_string());

        // Printing for internal use.
        assert_eq!(
            format!("{}", basic),
            "Internal error. | InternalError { context: \"\", message: \"\", debug: None }"
        );
        assert_eq!(
            format!("{}", normal),
            "Internal error. | InternalError { context: \"cxt\", message: \"msg\", debug: None }"
        );
        assert_eq!(
            format!("{}", with_debug),
            "Internal error. | InternalError { context: \"cxt\", message: \"msg\", debug: Some(\"debug\") }"
        );

        assert!(!basic.should_be_shown_to_client());
        assert!(!normal.should_be_shown_to_client());
        assert!(!with_debug.should_be_shown_to_client());

        // Same result after converting into Box<std_error>.
        assert_eq!(
            format!("{}", with_debug.into_std_error()),
            "Internal error. | InternalError { context: \"cxt\", message: \"msg\", debug: Some(\"debug\") }"
        );
    }
}
