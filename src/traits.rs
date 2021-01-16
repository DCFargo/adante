/// A trait describing the shared methods of both Flags and Arguments

pub trait ArgumentType {
    /// A user implemented function that takes a string as input and returns an
    /// argument type.
    ///
    /// # Examples
    /// ```
    /// use adante::traits::{ArgumentType, Error};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// enum ErrorType {
    ///     Syntax, // EXTREMELY simple example
    ///             // More complex examples are shown in
    ///             // the documentation for Error
    /// }
    /// impl Error for ErrorType {
    ///     fn handle(&self) {
    ///         ()
    ///     }
    ///     fn as_str(&self) -> &str {
    ///         "Syntax Error"
    ///     }
    /// }
    ///
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq)]
    /// enum FlagType {
    ///     Help,
    ///     Verbose,
    ///     Print,
    ///     TestFail, // NOTE: For testing only // Use Error
    /// }
    /// impl ArgumentType for FlagType {
    ///     fn from_str<ErrorType>(key: &str, error: ErrorType)
    ///                                -> Result<Self, ErrorType> {
    ///         match key {
    ///             "-h" | "--help" => Ok(Self::Help),
    ///             "-v" | "--verbose" => Ok(Self::Verbose),
    ///             "-p" | "--print" => Ok(Self::Print),
    ///             _ => Err(error),
    ///         }
    ///     }
    /// }
    /// let result = match FlagType::from_str("-v", ErrorType::Syntax) {
    ///     Ok(t) => t,
    ///     Err(_) => FlagType::TestFail // In actual usecase this would pipe into
    ///                                  // An Error's handle function
    /// };
    /// assert_eq!(result, FlagType::Verbose)
    ///
    /// ```
    fn from_str<E: Error>(key: &str, error: E) -> Result<Self, E>
    where
        Self: std::marker::Sized;

    // TODO: finish docs & test here
    /// A user implemented function that peforms a task or function
    /// depending on the type of error it is called on.
    ///
    /// In this case the function called will be an `assert_eq!` for testing purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use adante::
    /// ```
    fn handle(&self);
}

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
    /// use adante::traits::Error;
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
    /// use adante::traits::Error;
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
