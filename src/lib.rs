use anyhow::Result;
use dj::{ast::Expr, Environment, InterpretError, Parser, TokenStream};

pub mod commands;
pub mod handle_input;
pub mod process;

pub fn builtin_method(env: &mut Environment) {
    process::builtin_method(env);
}

pub fn parse_expr(str: &str) -> Result<Expr, InterpretError> {
    let mut parser = Parser::new(TokenStream::try_from(str)?);
    parser.parse_expr()
}
