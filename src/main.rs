use clap::Parser;
use dj::{Value, Environment, TokenStream, parse};
use dj_runner::{
    builtin_method,
    commands::Commands,
    handle_input::{get_input, ExprInput},
};
use std::{fs::File, io::Read};

fn main() {
    let mut env = Environment::new();

    builtin_method(&mut env);

    let cli = Commands::parse();

    match cli.file_path {
        Some(path) => run_file(&mut env, &path),
        None => console_runner(&mut env, cli.cut_input),
    }
}

/// Rune [dj] from file. Same of `(load '<file>')`
fn run_file(env: &mut Environment, path: &str) {
    let src = {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let mut buffer = String::new();

        if let Err(e) = file.read_to_string(&mut buffer) {
            println!("{}", e);
            return;
        }

        buffer
    };

    let _ = match parse(&src) {
        Ok(expr) => {
            if let Err(err) = expr.eval(env) {
                println!("{:?}", err);
            }
        }
        Err(err) => {
            let mut fullts = TokenStream::try_from(src.as_str()).unwrap();
            println!("Runtime Error:{err:?}\nToken Stream:");
            while let Some(tok) = fullts.pop() {
                println!("{tok:?}");
            }
            println!("parsing error.");
        }
    };
}

fn console_runner(env: &mut Environment, is_cut: bool) {
    println!("dj-runner -- Version {}", env!("CARGO_PKG_VERSION"));
    println!("(core) dj language(dj-rs) -- Version {}", "0.2.1");

    let mut expr_input = ExprInput::new(is_cut);
    loop {
        match get_input(&mut expr_input) {
            Ok(Some(ex)) => match ex.eval(env) {
                Ok(val) => {
                    match val {
                        Value::Nil => println!(),
                        _ => println!("{}", val),
                    };
                }
                Err(err) => println!("{}", err),
            },
            Err(err) => println!("{}", err),
            _ => {}
        }
        expr_input.reset();
    }
}
