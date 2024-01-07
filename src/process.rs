//! This module is for providing some method that is built in runner to `dj`

use std::{fs, io::Read, process};

use dj::builtin::*;

use crate::parse_expr;

pub fn builtin_method(env: &mut Environment) {
    env.set("exit", Value::Builtin(builtin_exit));
    env.set("println", Value::Builtin(builtin_println));
    let _ = builtin!(env, builtin_print);
    let _ = builtin!(env, builtin_load);
    let _ = builtin!(env, builtin_rem);
    let _ = builtin!(env, builtin_pow);
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
fn builtin_exit(env: &mut Environment, para: &[Expr]) -> Result<Value, RuntimeError> {
    let len = para.len();
    let exit_code;
    match len {
        0 => exit_code = 0,
        1 => {
            let code = para[0].eval(env)?;
            match code {
                Value::Integer(code) => exit_code = code,
                _ => return Err(RuntimeError::TypeMismatch(code)),
            }
        }
        _ => return Err(RuntimeError::TooManyArguments { at_most: 1 }),
    }
    process::exit(exit_code);
}

/// load file with path and use `Environment` evaluate it after read file content
fn load_file(env: &mut Environment, path: &str) -> Result<Value, RuntimeError> {
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
}

/// Command like:
/// ```dj
/// (load "sample.dj")
/// ```
#[builtin_method("load")]
fn builtin_load(env: &mut Environment, path: String) -> Result<Value, RuntimeError> {
    load_file(env, &path)
}

/// Command like:
/// ```dj
/// (print "Hello")
/// (print 123)
/// ```
#[builtin_method("print")]
fn builtin_print(content: Value) -> Result<Value, RuntimeError> {
    print!("{content}");
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

#[builtin_method("%")]
fn builtin_rem(a: Value, b: Value) -> Result<Value, RuntimeError> {
    let a = match a {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::TypeMismatch(a)),
    };

    let b = match b {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::TypeMismatch(b)),
    };
    Ok((a % b).into())
}

#[builtin_method("^")]
fn builtin_pow(val: Value, pow: Value) -> Result<Value, RuntimeError> {
    let val = match val {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::TypeMismatch(val)),
    };

    let pow = match pow {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::TypeMismatch(pow)),
    };

    Ok(val.powf(pow).into())
}