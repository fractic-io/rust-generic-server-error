use colored::Colorize as _;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerErrorBehaviour {
    ForwardToClient,
    LogWarningForwardToClient,
    LogErrorForwardToClient,
    LogWarningSendFixedMsgToClient(&'static str),
    LogErrorSendFixedMsgToClient(&'static str),
    ReturnInternalServerError,
}

pub trait ServerErrorTrait: std::fmt::Debug + Send + Sync + 'static {
    fn behaviour(&self) -> ServerErrorBehaviour;
    fn message(&self) -> &String;
    fn debug(&self) -> Option<&String>;
}

pub type ServerError = Box<dyn ServerErrorTrait>;

impl std::fmt::Display for dyn ServerErrorTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{:#?}", self.message().bold(), self,)
    }
}

impl std::error::Error for dyn ServerErrorTrait {}

// Tests
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        define_client_error, define_internal_error, define_sensitive_error, define_temporary_error,
        define_user_error, CLIENT_ERROR_MSG, SENSITIVE_ERROR_MSG,
    };

    use super::*;

    #[test]
    fn test_internal_error() {
        define_internal_error!(InternalError, "An internal error occurred");
        let error = InternalError::with_debug(&"debug info".to_string());

        assert_eq!(error.message(), "An internal error occurred");
        assert!(error.debug().is_some());
        assert!(error.debug().unwrap().contains("debug info"));
        assert_eq!(
            format!("{}", error),
            format!("{}\n{:#?}", "An internal error occurred".bold(), error)
        );
        assert_eq!(
            error.behaviour(),
            ServerErrorBehaviour::ReturnInternalServerError
        );
    }

    #[test]
    fn test_client_error() {
        define_client_error!(ClientError, "A client error occurred: {code}", { code: i32 });
        let error = ClientError::new(404);

        assert_eq!(error.message(), "A client error occurred: 404");
        assert!(error.debug().is_none());
        assert_eq!(
            format!("{}", error),
            format!("{}\n{:#?}", "A client error occurred: 404".bold(), error)
        );
        assert_eq!(
            error.behaviour(),
            ServerErrorBehaviour::LogErrorSendFixedMsgToClient(CLIENT_ERROR_MSG)
        );
    }

    #[test]
    fn test_sensitive_error() {
        define_sensitive_error!(SensitiveError, "Sensitive data error: {details} for user {user}", { details: &'static str, user: &'static str });
        let error =
            SensitiveError::with_debug("leak detected", "user123", &"debug info".to_string());

        assert_eq!(
            error.message(),
            "Sensitive data error: leak detected for user user123"
        );
        assert!(error.debug().is_some());
        assert!(error.debug().unwrap().contains("debug info"));
        assert_eq!(
            format!("{}", error),
            format!(
                "{}\n{:#?}",
                "Sensitive data error: leak detected for user user123".bold(),
                error
            )
        );
        assert_eq!(
            error.behaviour(),
            ServerErrorBehaviour::LogErrorSendFixedMsgToClient(SENSITIVE_ERROR_MSG)
        );
    }

    #[test]
    fn test_user_error() {
        define_user_error!(UserError, "User error occurred");
        let error = UserError::new();

        assert_eq!(error.message(), "User error occurred");
        assert!(error.debug().is_none());
        assert_eq!(
            format!("{}", error),
            format!("{}\n{:#?}", "User error occurred".bold(), error)
        );
        assert_eq!(
            error.behaviour(),
            ServerErrorBehaviour::LogWarningForwardToClient
        );
    }

    #[test]
    fn test_temporary_error() {
        define_temporary_error!(TemporaryError, "Temporary issue: {reason}, retry after {seconds} seconds by {method}", { reason: &'static str, seconds: u32, method: &'static str });
        let error = TemporaryError::new("network outage", 30, "reconnect");

        assert_eq!(
            error.message(),
            "Temporary issue: network outage, retry after 30 seconds by reconnect"
        );
        assert!(error.debug().is_none());
        assert_eq!(
            format!("{}", error),
            format!(
                "{}\n{:#?}",
                "Temporary issue: network outage, retry after 30 seconds by reconnect".bold(),
                error
            )
        );
        assert_eq!(
            error.behaviour(),
            ServerErrorBehaviour::LogWarningForwardToClient
        );
    }
}
