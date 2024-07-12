pub mod common;
pub mod macros;

pub trait GenericServerErrorTrait:
    std::error::Error + std::fmt::Debug + std::fmt::Display + Send + Sync + 'static
{
    fn should_be_shown_to_client(&self) -> bool;
    fn into_std_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync>;
}

pub type GenericServerError = Box<dyn GenericServerErrorTrait>;

// Tests.
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestErrorType(&'static str);

    impl std::fmt::Display for TestErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let TestErrorType(text) = self;
            write!(f, "Example error message with child text: {}.", text)
        }
    }

    impl std::error::Error for TestErrorType {}

    impl GenericServerErrorTrait for TestErrorType {
        fn should_be_shown_to_client(&self) -> bool {
            true
        }
        fn into_std_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync> {
            self
        }
    }

    #[test]
    fn test_generic_server_error_trait() {
        let _error: GenericServerError = Box::new(TestErrorType("abc"));
        assert_eq!(
            format!("{}", _error),
            "Example error message with child text: abc."
        );
        assert_eq!(format!("{:?}", _error), "TestErrorType(\"abc\")");

        // Convert to std_error.
        let _as_std_error = _error.into_std_error();
        assert_eq!(
            format!("{}", _as_std_error),
            "Example error message with child text: abc."
        );
        assert_eq!(format!("{:?}", _as_std_error), "TestErrorType(\"abc\")");
    }
}
