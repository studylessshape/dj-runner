use dj::{Environment, builtin::Value, RuntimeError};

pub mod commands;
pub mod handle_input;
pub mod process;

pub type EvalateResult = Result<Value, RuntimeError>;

pub fn builtin_method(env: &mut Environment) {
    process::builtin_method(env);
}