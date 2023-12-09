use crate::{parse_expr, RunnerError};
use crossterm::{
    cursor::{self, Hide, Show},
    event::{read, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    queue, style, terminal,
};
use dj::{ast::Expr, InterpretError, Token};
use std::io::{stdout, Write};
use tui_input::backend::crossterm as backend;
use tui_input::{backend::crossterm::EventHandler, Input};

pub fn get_input() -> Result<Option<Expr>, RunnerError> {
    let mut stdout = stdout().lock();
    // enter input and set input
    queue!(stdout, style::Print("> "), Hide, EnableMouseCapture)?;
    stdout.flush()?;
    // position and width will help to show the input
    let mut position = cursor::position()?;
    let width = 100.min(terminal::size()?.0) - 2;

    let mut expr_input = String::new();
    let mut input = Input::new(String::new());

    backend::write(&mut stdout, input.value(), input.cursor(), position, width)?;
    stdout.flush()?;

    loop {
        let event = read()?;

        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != KeyEventKind::Press {
                continue;
            }
            match code {
                KeyCode::Esc => {
                    return Ok(None);
                }
                KeyCode::Enter => {
                    if expr_input.len() != 0 {
                        expr_input.push('\n');
                        position.0 = 0;
                    }
                    expr_input.push_str(input.value());
                    position.1 += 1;
                    input.reset();

                    queue!(stdout, style::Print("\n"))?;
                    stdout.flush()?;

                    match parse_expr(input.value()) {
                        Ok(expr) => {
                            // leave input
                            queue!(stdout, Show)?;
                            stdout.flush()?;
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
                            queue!(stdout, Show)?;
                            stdout.flush()?;
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
