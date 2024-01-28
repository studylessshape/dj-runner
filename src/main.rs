use clap::Parser;
use dj::{parse, runtime::EnvExt, Env, Value};
use dj_runner::{
    builtin_method,
    commands::Commands,
    handle_input::{get_input, ExprInput},
};
use std::{fs::File, io::Read, rc::Rc};

fn main() {
    let env = Env::root();

    builtin_method(env.clone());

    let cli = Commands::parse();

    match cli.file_path {
        Some(path) => run_file(env, &path),
        None => console_runner(env, cli.cut_input),
    }
}

/// Rune [dj] from file. Same of `(load '<file>')`
fn run_file(env: Rc<Env>, path: &str) {
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

    let _ = match parse(src) {
        Ok(expr) => {
            if let Err(err) = env.eval(expr) {
                println!("{:?}", err);
            }
        }
        Err(err) => {
            println!("Parse Error: {err}\nToken Stream:");
        }
    };
}

fn console_runner(env: Rc<Env>, is_cut: bool) {
    println!("dj-runner -- Version {}", env!("CARGO_PKG_VERSION"));
    println!("(core) dj language(dj-rs) -- Version {}", "0.3");

    let mut expr_input = ExprInput::new(is_cut);
    loop {
        match get_input(&mut expr_input) {
            Ok(Some(ex)) => match env.eval(ex) {
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
