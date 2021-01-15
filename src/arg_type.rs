/// A trait describing the shared methods of both Flags and Arguments

pub trait ArgumentType {
    /// A user implemented function that takes a string as input and returns an
    /// argument type.
    ///
    /// # Examples
    /// ```
    /// use adante::{ArgumentType, Error};
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
    ///     TestFail, // NOTE: For testing only
    ///               // Use Error
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
}
