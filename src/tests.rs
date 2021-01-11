use crate::{ArgumentType, Arguments, Error, Flag};

#[derive(Debug, Clone, Copy)]
pub enum TestErrorType {
    Syntax,
    FlagVal,
    NoFlagVal,
    NotRecognized,
}
impl Error for TestErrorType {
    fn handle(&self) {
        println!("{:?}", self);
        std::process::exit(1);
    }
    fn as_str(&self) -> &str {
        match self {
            Self::FlagVal => "No associated value for given flag",
            Self::NoFlagVal => "Need associated value for given flag",
            Self::Syntax => "Improper syntax usage",
            Self::NotRecognized => "Action or flag is not recognized",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TestFlagType {
    Help,
    Verbose,
    Print,
}
impl ArgumentType for TestFlagType {
    fn from_str<TestErrorType>(key: &str, error: TestErrorType) -> Result<Self, TestErrorType> {
        match key {
            "-h" | "--help" => Ok(Self::Help),
            "-v" | "--verbose" => Ok(Self::Verbose),
            "-p" | "--print" => Ok(Self::Print),
            _ => Err(error),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TestActionType {
    Add,
    Remove,
    Edit,
}

impl ArgumentType for TestActionType {
    fn from_str<TestErrorType>(key: &str, error: TestErrorType) -> Result<Self, TestErrorType> {
        match key {
            "add" | "a" => Ok(Self::Add),
            "remove" | "r" => Ok(Self::Remove),
            "edit" | "e" => Ok(Self::Edit),
            _ => Err(error),
        }
    }
}

#[test]
fn parse_noval_flag() {
    let env_args: Vec<&str> = vec!["-v"];
    let env_args: Arguments<TestFlagType, TestActionType> =
        match Arguments::parse(env_args, TestErrorType::Syntax) {
            Ok(a) => a,
            Err(e) => return e.handle(),
        };
    assert_eq!(env_args.flags[0].key, TestFlagType::Verbose);
    assert_eq!(env_args.flags[0].value, None);
    assert_eq!(env_args.actions.len(), 0);
}

#[test]
fn parse_val_flag() {
    let env_args: Vec<&str> = vec!["-h=test"];
    let env_args: Arguments<TestFlagType, TestActionType> =
        match Arguments::parse(env_args, TestErrorType::Syntax) {
            Ok(a) => a,
            Err(e) => return e.handle(),
        };
    assert_eq!(env_args.flags[0].key, TestFlagType::Help);
    assert_eq!(env_args.flags[0].value, Some("test".to_string()));
    assert_eq!(env_args.actions.len(), 0);
}

#[test]
fn parse_action() {
    let env_args: Vec<&str> = vec!["add"];
    let env_args: Arguments<TestFlagType, TestActionType> =
        match Arguments::parse(env_args, TestErrorType::Syntax) {
            Ok(a) => a,
            Err(e) => return e.handle(),
        };
    assert_eq!(env_args.actions[0], TestActionType::Add);
    assert_eq!(env_args.actions.len(), 1);
    assert_eq!(env_args.flags.len(), 0);
}
