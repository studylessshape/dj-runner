//! This module is for providing some method that is built in runner to `dj`

use std::{fs, io::Read, process};

use dj::{
    ast::{Expr, Value},
    Environment, RuntimeError,
};

use crate::parse_expr;

pub fn builtin_method(env: &mut Environment) {
    env.set("exit", Value::Builtin(builtin_exit));
    env.set("println", Value::Builtin(builtin_println));
    env.set("print", Value::Builtin(builtin_print));
    env.set("load", Value::Builtin(builtin_load));
}

/// exit program
/// 
/// Command like:
/// ```dj
/// (exit)
/// ```
fn builtin_exit(_env: &mut Environment, _para: &[Expr]) -> Result<Value, RuntimeError> {
    process::exit(0);
}

/// Command like:
/// ```dj
/// (load "sample.dj")
/// ```
fn builtin_load(env: &mut Environment, para: &[Expr]) -> Result<Value, RuntimeError> {
    let len = para.len();
    match len {
        1 => {
            // read path
            let path = {
                let p = para[0].eval(env)?;
                match p {
                    Value::Literal(str) => str.clone(),
                    _ => return Err(RuntimeError::TypeMismatch(p.clone())),
                }
            };

            let load_file = |env: &mut Environment, path: &str| -> Result<Value, RuntimeError> {
                // read file
                let mut file = match fs::File::open(path) {
                    Ok(f) => f,
                    Err(e) => return Err(RuntimeError::Custom(e.to_string())),
                };
                let mut buf = String::new();
                file.read_to_string(&mut buf)
                    .map_err(|e| RuntimeError::Custom(e.to_string()))?;
                // run in environment
                match parse_expr(&buf) {
                    Ok(expr) => expr.eval(env),
                    Err(e) => Err(RuntimeError::Custom(format!("{:?}", e))),
                }
            };

            load_file(env, &path)
        }
        _ => Err(RuntimeError::ArgumentsCountNotMatch {
            expect: 1,
            got: len,
        }),
    }
}

/// Command like:
/// ```dj
/// (print "Hello")
/// ```
fn builtin_print(env: &mut Environment, params: &[Expr]) -> Result<Value, RuntimeError> {
    let len = params.len();
    if len == 0 {
        return Err(RuntimeError::TooFewArguments { at_least: 1 });
    } else {
        let mut i = 0;
        while i < len {
            print!("{}", &params[i].eval(env)?);
            i += 1;
        }
    }
    Ok(Value::Nil)
}

/// Command like:
/// ```dj
/// (println "Hello, World")
/// ```
fn builtin_println(env: &mut Environment, params: &[Expr]) -> Result<Value, RuntimeError> {
    let len = params.len();
    if len == 0 {
        println!();
        return Ok(Value::Nil);
    } else {
        let mut i = 0;
        while i < len {
            println!("{}", &params[i].eval(env)?);
            i += 1;
        }
    }
    Ok(Value::Nil)
}
