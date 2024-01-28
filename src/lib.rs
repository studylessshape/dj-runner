use std::rc::Rc;

use dj::Env;

pub mod commands;
pub mod handle_input;
pub mod process;

pub fn builtin_method(env: Rc<Env>) {
    process::builtin_method(env);
}