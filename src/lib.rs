use std::{error::Error, fmt::Display, io::Error as IoError};

use dj::{ast::Expr, Environment, InterpretError, Parser, RuntimeError, TokenStream};

pub mod commands;
pub mod handle_input;
pub mod process;

#[derive(Debug)]
pub enum RunnerError {
    IoError(IoError),
    RuntimeError(RuntimeError),
    ParseError(InterpretError),
}

impl Error for RunnerError {}

impl Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<IoError> for RunnerError {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

impl From<RuntimeError> for RunnerError {
    fn from(value: RuntimeError) -> Self {
        Self::RuntimeError(value)
    }
}

impl From<InterpretError> for RunnerError {
    fn from(value: InterpretError) -> Self {
        Self::ParseError(value)
    }
}

pub fn builtin_method(env: &mut Environment) {
    process::builtin_method(env);
}

pub fn parse_expr(str: &str) -> Result<Expr, InterpretError> {
    let mut parser = Parser::new(TokenStream::try_from(str)?);
    parser.parse_expr()
}
