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

// "Simulates" running a program with arguments, collected by std::env::args::collect()
// NOTE: File path is omitted, would cause error as of 01-11
fn simulate(env_args: Vec<&str>) -> Result<Arguments<TestFlagType, TestActionType>, TestErrorType> {
    let env_args: Arguments<TestFlagType, TestActionType> =
        return match Arguments::parse(env_args, TestErrorType::Syntax) {
            Ok(a) => Ok(a),
            Err(e) => Err(e),
        };
}

// NOTE: Parsing flags is ok, all tests pass
#[test]
fn parse_flag_key_from_str() {
    let env_args = match simulate(vec!["-v"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.flags[0].key, TestFlagType::Verbose)
}

#[test]
fn parse_flag_val_from_str() {
    let env_args = match simulate(vec!["-h=test"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.flags[0].value, Some("test".to_string()));
}

#[test]
fn no_misinterpret_flag_as_action() {
    let env_args = match simulate(vec!["-v"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.actions.len(), 0);
}

#[test]
fn parse_noval_flag() {
    let env_args = match simulate(vec!["-v"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.flags[0].key, TestFlagType::Verbose);
    assert_eq!(env_args.flags[0].value, None);
    assert_eq!(env_args.actions.len(), 0);
}

#[test]
fn parse_val_flag() {
    let env_args = match simulate(vec!["-h=test"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.flags[0].key, TestFlagType::Help);
    assert_eq!(env_args.flags[0].value, Some("test".to_string()));
    assert_eq!(env_args.actions.len(), 0);
}

// FIXME: Automated testing fails here
// I believe it to be the from_str() function, though theres nothing concerning
// Maybe its a String/&str error? Create tests for each step in the process
// Extract simulate and do direct testing
#[test]
fn parse_action_from_str() {
    let env_args = match simulate(vec!["add"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.actions[0], TestActionType::Add)
}

#[test]
fn no_misinterpret_action_as_flag() {
    let env_args = match simulate(vec!["add"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.flags.len(), 0)
}

#[test]
fn parse_action() {
    let env_args = match simulate(vec!["add"]) {
        Ok(a) => a, Err(e) => return e.handle()
    };
    assert_eq!(env_args.actions[0], TestActionType::Add);
    assert_eq!(env_args.actions.len(), 1);
    assert_eq!(env_args.flags.len(), 0);
}
