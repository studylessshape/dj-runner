//! This module is for providing some method that is built in runner to `dj`

use dj::{builtin::*, *};
use std::{fs, io::Read, process};

use crate::EvalateResult;

pub fn builtin_method(env: &mut Environment) {
    let _ = builtin!(env, builtin_exit);
    let _ = builtin!(env, builtin_print);
    let _ = builtin!(env, builtin_println);
    let _ = builtin!(env, builtin_load);
    let _ = builtin!(env, builtin_rem);
    let _ = builtin!(env, builtin_pow);
    let _ = builtin!(env, builtin_bitnot);
    let _ = builtin!(env, builtin_bitand);
    let _ = builtin!(env, builtin_bitor);
    let _ = builtin!(env, builtin_bitxor);
    let _ = builtin!(env, builtin_bitshl);
    let _ = builtin!(env, builtin_bitshr);
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

/// Command like:
/// ```dj
/// (load "sample.dj")
/// ```
#[builtin_method("load")]
fn builtin_load(path: String, env: &mut Environment) -> EvalateResult {
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
/// (print "Hello")
/// (print 123)
/// ```
#[builtin_method("print")]
fn builtin_print(content: Value, _env: &mut Environment, _exprs: Vec<Expr>) -> EvalateResult {
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

macro_rules! match_value_to_f32 {
    ($val: expr) => {
        match $val {
            Value::Integer(integer) => Ok(integer as f32),
            Value::Decimal(decimal) => Ok(decimal),
            _ => Err(RuntimeError::ValueTypeMismatch($val)),
        }
    };
}

#[builtin_method("rem")]
fn builtin_rem(a: Value, b: Value) -> EvalateResult {
    let a = match_value_to_f32!(a)?;
    let b = match_value_to_f32!(b)?;

    Ok((a % b).into())
}

#[builtin_method("pow")]
fn builtin_pow(val: Value, pow: Value) -> EvalateResult {
    let val = match_value_to_f32!(val)?;
    let pow = match_value_to_f32!(pow)?;

    Ok(val.powf(pow).into())
}

macro_rules! value_bit_op {
    ($val: tt, $op: tt) => {
        match $val {
            Value::Boolean(val_bool) => Ok(($op val_bool).into()),
            Value::Integer(val_i32) =>Ok(($op val_i32).into()),
            _ => Err(RuntimeError::ValueTypeMismatch($val))
        }
    };
    ($lhs: tt, $rhs: tt, $op: tt) => {
        match $lhs {
            Value::Boolean(lhs_bool) => match $rhs {
                Value::Boolean(rhs) => Ok((lhs_bool $op rhs).into()),
                _ => Err(RuntimeError::ValueTypeMismatch($rhs)),
            },
            Value::Integer(lhs_i32) => match $rhs {
                Value::Integer(rhs) => Ok((lhs_i32 $op rhs).into()),
                _ => Err(RuntimeError::ValueTypeMismatch($rhs)),
            },
            _ => Err(RuntimeError::ValueTypeMismatch($lhs)),
        }
    };
    ($lhs: tt, $rhs: tt, $op: tt, not bool) => {
        match $lhs {
            Value::Integer(lhs_i32) => match $rhs {
                Value::Integer(rhs) => Ok((lhs_i32 $op rhs).into()),
                _ => Err(RuntimeError::ValueTypeMismatch($rhs)),
            },
            _ => Err(RuntimeError::ValueTypeMismatch($lhs)),
        }
    };
}

#[builtin_method("bitnot")]
fn builtin_bitnot(val: Value) -> EvalateResult {
    value_bit_op!(val, !)
}

#[builtin_method("bitand")]
fn builtin_bitand(lhs: Value, rhs: Value) -> EvalateResult {
    value_bit_op!(lhs, rhs, &)
}

#[builtin_method("bitor")]
fn builtin_bitor(lhs: Value, rhs: Value) -> EvalateResult {
    value_bit_op!(lhs, rhs, |)
}

#[builtin_method("bitxor")]
fn builtin_bitxor(lhs: Value, rhs: Value) -> EvalateResult {
    value_bit_op!(lhs, rhs, ^)
}

#[builtin_method("bitshl")]
fn builtin_bitshl(lhs: Value, rhs: Value) -> EvalateResult {
    value_bit_op!(lhs, rhs, <<, not bool)
}

#[builtin_method("bitshr")]
fn builtin_bitshr(lhs: Value, rhs: Value) -> EvalateResult {
    value_bit_op!(lhs, rhs, >>, not bool)
}