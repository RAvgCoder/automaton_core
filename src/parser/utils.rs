use std::fmt;

use crate::parser::ParserError::{
    MissingObjSeparator, NoObjName, ObjNameMismatch, ObjNameNotFound, ObjNameOverFlow,
    ObjNameSyntaxErr, OutOfInput, ScopeError, UnknownTransitionName,
};
use crate::parser::{ParserError, Scope, Separator, SkeletonState};

impl Into<char> for Separator {
    /// Converts the separator from a u8 to a char equivalent
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Scope {
    /// Gives the corresponding closing bracket for a given scope
    pub fn closing(&self) -> char {
        match self {
            Scope::BoxBracket => ']',
            Scope::CurlyBracket => '}',
        }
    }
}

impl Into<char> for Scope {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutOfInput(err_message) => write!(f, "OutOfInput {}", err_message),
            NoObjName(err_message) => write!(f, "NoObjName {}", err_message),
            ObjNameSyntaxErr(err_message) => write!(f, "ObjNameSyntaxErr {}", err_message),
            ObjNameNotFound(err_message) => write!(f, "ObjNameNotFound {}", err_message),
            ObjNameOverFlow(err_message) => write!(f, "ObjNameOverFlow {}", err_message),
            ObjNameMismatch(err_message) => write!(f, "ObjNameMismatch {}", err_message),
            MissingObjSeparator(err_message) => write!(f, "MissingObjSeparator {}", err_message),
            ScopeError(err_message) => write!(f, "NoScopeFound {}", err_message),
            UnknownTransitionName(err_message) => {
                write!(f, "UnknownTransitionName {}", err_message)
            }
        }
    }
}

impl SkeletonState {
    /// Converts the skeleton_state name from a string to its concrete type
    pub fn from_string(s: &str) -> Option<SkeletonState> {
        match s.to_lowercase().as_str() {
            "type" => Some(SkeletonState::Type),
            "dfa" => Some(SkeletonState::AutomatonType),
            "nfa" => Some(SkeletonState::AutomatonType),
            "pda" => Some(SkeletonState::AutomatonType),
            "transitions" => Some(SkeletonState::Transitions),
            "startstate" => Some(SkeletonState::StartState),
            "acceptstates" => Some(SkeletonState::AcceptStates),
            "states" => Some(SkeletonState::States),
            "bulktests" => Some(SkeletonState::BulkTests),
            _ => None,
        }
    }
}
