use std::fmt::Debug;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub struct KError<T: Debug> {
    pub location: usize,
    pub code: T,
}

#[derive(Debug)]
pub enum LexerErrorCode {
    UnterminatedString,
    UnterminatedEscape,
    UnterminatedFloatExponent,
    UnrecognizedEscape,
    UnrecognizedToken,
    InvalidNumber,
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
}

impl From<ParseFloatError> for LexerErrorCode {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl From<ParseIntError> for LexerErrorCode {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

#[derive(Debug)]
pub enum ParserErrorCode {
    UnclosedParens,
    UnclosedBrackets,
    UnexpectedToken,
}

#[derive(Debug)]
pub enum RuntimeErrorCode {
    Length,
    Nyi,
    Rank,
    Type,
    NameExpected,
    ExpressionExpected,
    UndefinedVariable,
}

pub type LexerError = KError<LexerErrorCode>;
pub type ParserError = KError<ParserErrorCode>;
pub type RuntimeError = KError<RuntimeErrorCode>;

impl RuntimeError {
    pub fn new(location: usize, code: RuntimeErrorCode) -> Self {
        Self { location, code }
    }
}
