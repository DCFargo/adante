//! # adante
//!
//! 'adante' is a simple library that handles the logic of user defined types
//! in order to provide efficiency in parsing command line arguments.

#[cfg(test)]
mod tests;

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

pub trait Error {
    fn handle(&self);
    fn as_str(&self) -> &str;
}

#[derive(Debug)]
pub struct Flag<T: ArgumentType> {
    pub key: T,
    // NOTE: Thought making String generic here
    // may have been overdoing it a bit.
    // Consider.
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Arguments<F: ArgumentType, A: ArgumentType> {
    pub flags: Vec<Flag<F>>,
    pub actions: Vec<A>,
}

impl<F: ArgumentType, A: ArgumentType> Arguments<F, A> {
    fn new() -> Self {
        Arguments {
            flags: Vec::new(),
            actions: Vec::new(),
        }
    }
    fn parse<E: Error + Clone + Copy>(env_args: Vec<&str>, error: E) -> Result<Arguments<F, A>, E> {
        let mut args = Arguments::new();
        let mut eq_pos: usize = 0;
        for arg in env_args.iter() {
            // Detect if argument is option or action:
            if &arg[0..1] == "-" {
                // Assume flag, find seperator:
                for (i, &byte) in arg.as_bytes().iter().enumerate() {
                    if byte == b'=' {
                        eq_pos = i;
                    }
                }
                // Assume no value if no =:
                if eq_pos == 0 {
                    args.flags.push(Flag {
                        key: match F::from_str(arg, error.clone()) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        },
                        value: None,
                    })
                // Seperator found
                // FIXME: BREAKS HERE
                } else {
                    let key = &arg[0..eq_pos];
                    let val = &arg[(eq_pos + 1)..];
                    args.flags.push(Flag {
                        key: match F::from_str(key, error.clone()) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        },
                        // TODO: make value field a &str by default
                        value: Some(val.to_string()),
                    })
                }
            // TODO: Recognize file path, omit or save to output
            } else {
                // Assume action, match string to type
                args.actions.push(match A::from_str(arg, error) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                })
            }
        }

        Ok(args)
    }
}
