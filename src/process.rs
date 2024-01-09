//! This module is for providing some method that is built in runner to `dj`

use dj::{builtin::*, parse};
use std::{fs, io::Read, process, any::Any};

use crate::EvalateResult;

pub fn builtin_method(env: &mut Environment) {
    let _ = builtin!(env, builtin_exit);
    let _ = builtin!(env, builtin_print);
    let _ = builtin!(env, builtin_println);
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
#[builtin_method("exit")]
fn builtin_exit(code: Option<i32>) -> EvalateResult {
    match code {
        Some(exit_code) => process::exit(exit_code),
        None => process::exit(0),
    }
}

/// load file with path and use `Environment` evaluate it after read file content
fn load_file(env: &mut Environment, path: &str) -> EvalateResult {
    // read file
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(RuntimeError::Custom(e.to_string())),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| RuntimeError::Custom(e.to_string()))?;
    // run in environment
    match parse(&buf) {
        Ok(expr) => expr.eval(env),
        Err(e) => Err(RuntimeError::Custom(format!("{:?}", e))),
    }
}

/// Command like:
/// ```dj
/// (load "sample.dj")
/// ```
#[builtin_method("load")]
fn builtin_load(env: &mut Environment, path: String) -> EvalateResult {
    load_file(env, &path)
}

/// Command like:
/// ```dj
/// (print "Hello")
/// (print 123)
/// ```
#[builtin_method("print")]
fn builtin_print(content: Value) -> EvalateResult {
    print!("{content}");
    Ok(Value::Nil)
}

/// Command like:
/// ```dj
/// (println "Hello, World")
/// ```
#[builtin_method("println")]
fn builtin_println(content: Option<Value>) -> EvalateResult {
    match content {
        Some(val) => println!("{val}"),
        None => println!(),
    }
    Ok(Value::Nil)
}

#[builtin_method("%")]
fn builtin_rem(a: Value, b: Value) -> EvalateResult {
    let a = match a {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::ValueTypeMismatch(a)),
    };

    let b = match b {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::ValueTypeMismatch(b)),
    };
    Ok((a % b).into())
}

#[builtin_method("^")]
fn builtin_pow(val: Value, pow: Value) -> EvalateResult {
    let val = match val {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::ValueTypeMismatch(val)),
    };

    let pow = match pow {
        Value::Integer(integer) => integer as f32,
        Value::Decimal(decimal) => decimal,
        _ => return Err(RuntimeError::ValueTypeMismatch(pow)),
    };

    Ok(val.powf(pow).into())
}

#[builtin_method("bitand")]
fn builtin_bitand(a: Value, b: Value) -> EvalateResult {
    match a {
        Value::Boolean(a_bool) => match b {
            Value::Boolean(b) => Ok((a_bool & b).into()),
            _ => Err(RuntimeError::ValueTypeMismatch(b)),
        },
        Value::Integer(a_i32) => match b {
            Value::Integer(b) => Ok((a_i32 & b).into()),
            _ => Err(RuntimeError::ValueTypeMismatch(b)),
        },
        _ => Err(RuntimeError::ValueTypeMismatch(a)),
    }
}

#[builtin_method("bitor")]
fn builtin_bitor(a: Value, b: Value) -> EvalateResult {
    a.type_id();
    todo!()
}
