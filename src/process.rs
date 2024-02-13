//! This module is for providing some method that is built in runner to `dj`

use dj::{builtin::{*, dependency::*}, parse, runtime::Error as RuntimeError, Env, EvalResult};
use std::{fs, io::Read, process, rc::Rc};

pub fn builtin_method(env: Rc<Env>) {
    builtin!(env, builtin_exit);
    builtin!(env, builtin_print);
    builtin!(env, builtin_println);
    builtin!(env, builtin_load);
    builtin!(env, builtin_rem);
}

/// exit program
///
/// Command like:
/// ```dj
/// (exit)
/// ```
///
/// Or you can specify the exit code like:
/// ```dj
/// (exit 1)
/// ```
#[builtin_method("exit")]
fn builtin_exit(code: Option<i32>) -> EvalResult {
    match code {
        Some(exit_code) => process::exit(exit_code),
        None => process::exit(0),
    }
}

/// Command like:
/// ```dj
/// (load "sample.dj")
/// ```
#[builtin_method("load")]
fn builtin_load(path: String, env: Rc<Env>) -> EvalResult {
    // read file
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::Custom(e.to_string())),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| RuntimeError::Custom(e.to_string()))?;
    // run in environment
    match parse(buf) {
        Ok(expr) => env.eval(expr),
        Err(e) => Err(RuntimeError::Custom(format!("{:?}", e))),
    }
}

/// Command like:
/// ```dj
/// (print "Hello")
/// (print 123)
/// ```
#[builtin_method("print")]
fn builtin_print(content: Value) -> EvalResult {
    print!("{content}");
    Ok(Value::Nil)
}

/// Command like:
/// ```dj
/// (println "Hello, World")
/// ```
#[builtin_method("println")]
fn builtin_println(content: Option<Value>) -> EvalResult {
    match content {
        Some(val) => println!("{val}"),
        None => println!(),
    }
    Ok(Value::Nil)
}

#[builtin_method("%")]
fn builtin_rem(lhs: f32, rhs: f32) -> EvalResult {
    Ok(Value::from(lhs % rhs))
}