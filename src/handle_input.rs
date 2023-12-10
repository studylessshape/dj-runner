use crate::{parse_expr, RunnerError};
use crossterm::{
    cursor::{self, Hide, Show, MoveTo},
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
                    print_input(&mut stdout, &input, position, width)?;
                    if expr_input.len() != 0 {
                        expr_input.push('\n');
                    } else {
                        position.0 = 0;
                    }
                    expr_input.push_str(input.value());
                    position.1 += 1;
                    input.reset();

                    match parse_expr(&expr_input) {
                        Ok(expr) => {
                            // leave input
                            leave_input_mode(&mut stdout)?;
                            return Ok(Some(expr));
                        }
                        Err(InterpretError::Syntax(dj::SyntaxError::RequrieToken(Token::End(
                            _,
                        )))) => {
                            queue!(stdout, style::Print("\n"))?;
                            stdout.flush()?;

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
    queue!(stdout, Show, style::Print("\n"))?;
    stdout.flush()?;
    Ok(())
}

fn print_input(stdout: &mut StdoutLock, input: &Input, (column, row): (u16, u16), width: u16) -> Result<(), RunnerError> {
    let width = width as usize;
    let mut print_str = String::from(input.value());
    if print_str.len() <= width {
        print_str.push_str(&vec![' '; width - print_str.len() + 1].into_iter().collect::<String>());
    }
    queue!(stdout, MoveTo(column, row), style::Print(print_str))?;
    Ok(())
}