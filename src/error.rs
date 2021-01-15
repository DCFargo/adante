/// A trait that describes the functions an error must implement to be valid

pub trait Error {
    /// A user implemented function that performs a task then exits
    /// depending on the type of error it is called on.
    ///
    /// In proper usage `std::process:exit(1)` would be used; however, this
    /// example just uses `assert_eq!(2 + 2, 4)` to validate the test.
    ///
    /// # Examples
    ///
    /// ```
    /// use adante::Error;
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// enum ErrorType {
    ///     Syntax,
    ///     InvalidAction,
    ///     InvalidFlag,
    ///     NoFlagVal,
    /// }
    /// impl Error for ErrorType {
    ///     fn handle(&self) {
    ///         // Handle code goes here:
    ///         match self {
    ///             Self::Syntax => assert_eq!(2 + 2, 4),           // Only branch that should work
    ///             Self::InvalidAction => assert_eq!(2 + 2, 5),
    ///             Self::InvalidFlag => assert_eq!(2 + 2, 5),
    ///             Self::NoFlagVal => assert_eq!(2 + 2, 5),
    ///         }
    ///     }
    ///     fn as_str(&self) -> &str {" "}
    /// }
    ///
    /// let test_error = ErrorType::Syntax;
    /// test_error.handle();
    /// ```
    fn handle(&self);
    /// A user implemented function that returns a &str (usually an error message)
    /// depending ont he type of error it is called on.
    ///
    /// # Examples
    ///
    /// ```
    /// use adante::Error;
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// enum ErrorType {
    ///     Syntax,
    ///     InvalidAction,
    ///     InvalidFlag,
    ///     NoFlagVal,
    /// }
    /// impl Error for ErrorType {
    ///     fn handle(&self) {  }
    ///     fn as_str(&self) -> &str {
    ///         match self {
    ///             Self::Syntax => "Good!",
    ///             Self::InvalidAction => "Bad!",
    ///             Self::InvalidFlag => "Bad!",
    ///             Self::NoFlagVal => "Bad!",
    ///         }
    ///     }
    /// }
    ///
    /// let test_error = ErrorType::Syntax;
    /// assert_eq!(test_error.as_str(), "Good!");
    /// ```
    fn as_str(&self) -> &str;
}
