## Important note

This package is no longer being maintained do to more active alternatives being available.
This package will likely no longer be updated on the crates.io registry, and may see removal. 
If you need an alternative, please check out [pico-args](https://crates.io/crates/pico-args), 
it is what I now opt to use.

# adante

`adante` is a simple library that handles the logic of user defined types
in order to provide efficiency in parsing command line arguments.
At its core, `adante` simply provides an interface of which command line
arguments can be transformed into a collection of three types.

- Flags, which consist of:
    - A flag key
    - An optional `String` value
- Actions
- Errors

This is achieved by implementing a simple, but widely versatile
set of tools laid out by the library. Here are the steps to making
your first parser!

## 1. Define an enum consisting of all the errors your program might run into.

This can be seperate from your application's general Error enum
if you have one, or it can be the same.

```
#[derive(Debug, Clone, Copy)] // Highly advised
pub enum ErrorType {
    Syntax,
    InvalidAction,
    InvalidFlag,
    NoFlagVal,
}
```

## 2. Imply the `Error` trait from the library.

```
use adante::Error;
#[derive(Debug, Clone, Copy)] // Highly advised
pub enum ErrorType {
    Syntax,
    InvalidAction,
    InvalidFlag,
    NoFlagVal,
}
impl Error for ErrorType {
    fn handle(&self) {
        println!("{}", self.as_str());
        std::process::exit(1);
    }
    fn as_str(&self) -> &str {
        match self {
            Self::Syntax => "Improper syntax usage.",
            Self::InvalidAction => "One or more of your actions entered is invalid.",
            Self::InvalidFlag => "One or more of your flags entered is invalid",
            Self::NoFlagVal => "One or more of your flags is missing a field",
        }
    }
}
```

## 3. Define an enum consisting of all of your flag and action keys.

```
enum FlagType {
    Help,
    Verbose,
    Print,
}

enum ActionType {
    Add,
    Remove,
    Edit,
}
```

## 4. Imply the `ArgumentType` trait from the library.

```
use adante::ArgumentType;
enum FlagType {
    Help,
    Verbose,
    Print,
}
impl ArgumentType for FlagType {
    fn from_str<ErrorType>(key: &str, error: ErrorType)
        -> Result<Self, ErrorType> {
        match key {
            "-h" | "--help" => Ok(Self::Help),
            "-v" | "--verbose" => Ok(Self::Verbose),
            "-p" | "--print" => Ok(Self::Print),
            _ => Err(error),
        }
    }
}

enum ActionType {
    Add,
    Remove,
    Edit,
}
impl ArgumentType for ActionType {
    fn from_str<ErrorType>(key: &str, error: ErrorType)
        -> Result<Self, ErrorType> {
        match key {
            "a" | "add" => Ok(Self::Add),
            "r" | "remove" => Ok(Self::Remove),
            "e" | "edit" => Ok(Self::Edit),
            _ => Err(error),
        }
    }
}
```

## And voila!

Now your parser is complete! By plugging `std::env::args::collect()` into
`Arguments::parse()`, you will get a working `Arguments` object!
