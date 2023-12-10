use crate::{parse_expr, RunnerError};
use crossterm::{
    cursor::{self, Hide, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use dj::{ast::Expr, InterpretError, Token};
use std::io::{stdout, StdoutLock, Write};
use tui_input::backend::crossterm as backend;
use tui_input::{backend::crossterm::EventHandler, Input};

pub fn get_input() -> Result<Option<Expr>, RunnerError> {
    let mut stdout = stdout().lock();
    // enter input and set input
    enter_input_mode(&mut stdout)?;
    // position and width will help to show the input
    let mut position = cursor::position()?;
    let width = 50.min(terminal::size()?.0) - 2;

    let mut expr_input = String::new();
    let mut input = Input::new(String::new());

    backend::write(&mut stdout, input.value(), input.cursor(), position, width)?;
    stdout.flush()?;

    loop {
        let event = read()?;

        if let Event::Key(KeyEvent {
            code,
            kind,
            modifiers,
            ..
        }) = event
        {
            if kind != KeyEventKind::Press {
                continue;
            }
            if let (KeyCode::Char('c'), KeyModifiers::CONTROL) = (code, modifiers) {
                leave_input_mode(&mut stdout)?;
                return Ok(Some(parse_expr("(exit)")?));
            }
            match code {
                KeyCode::Esc => {
                    leave_input_mode(&mut stdout)?;
                    return Ok(None);
                }
                KeyCode::Enter => {
                    if expr_input.len() != 0 {
                        expr_input.push('\n');
                    } else {
                        position.0 = 0;
                    }
                    expr_input.push_str(input.value());
                    position.1 += 1;
                    input.reset();

                    queue!(stdout, style::Print("\n"))?;
                    stdout.flush()?;

                    match parse_expr(&expr_input) {
                        Ok(expr) => {
                            // leave input
                            leave_input_mode(&mut stdout)?;
                            return Ok(Some(expr));
                        }
                        Err(InterpretError::Syntax(dj::SyntaxError::RequrieToken(Token::End(
                            _,
                        )))) => {
                            backend::write(
                                &mut stdout,
                                input.value(),
                                input.cursor(),
                                position,
                                width,
                            )?;
                            stdout.flush()?;
                        }
                        Err(err) => {
                            leave_input_mode(&mut stdout)?;
                            return Err(RunnerError::from(err));
                        }
                    }
                }
                _ => {
                    if input.handle_event(&event).is_some() {
                        backend::write(
                            &mut stdout,
                            input.value(),
                            input.cursor(),
                            position,
                            width,
                        )?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }
}

fn enter_input_mode(stdout: &mut StdoutLock) -> Result<(), RunnerError> {
    enable_raw_mode()?;
    queue!(stdout, style::Print("> "), Hide)?;
    stdout.flush()?;
    Ok(())
}

fn leave_input_mode(stdout: &mut StdoutLock) -> Result<(), RunnerError> {
    disable_raw_mode()?;
    queue!(stdout, Show)?;
    stdout.flush()?;
    Ok(())
}
