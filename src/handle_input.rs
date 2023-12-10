use crate::{parse_expr, RunnerError};
use crossterm::{
    cursor::{self, Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use dj::{ast::Expr, InterpretError, Token};
use std::io::{stdout, StdoutLock, Write};
use tui_input::{backend::crossterm as backend, StateChanged};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Default)]
pub struct ExprInput {
    input: Input,
    total: String,
    pub position: (u16, u16),
    pub width: u16,
    history: Vec<String>,
    his_index: Option<usize>,
}

impl ExprInput {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn expr(&self) -> &str {
        &self.total
    }

    pub fn reset(&mut self) {
        self.input.reset();
        self.total.clear();
        self.his_index = None;
    }

    pub fn handle_input_event(&mut self, event: &Event) -> Option<StateChanged> {
        self.input.handle_event(event)
    }
    /// Call [tui_input::backend::crossterm::write]
    pub fn write(&self, stdout: &mut StdoutLock) -> Result<(), RunnerError> {
        backend::write(
            stdout,
            self.input.value(),
            self.input.cursor(),
            self.position,
            self.width,
        )?;
        stdout.flush()?;
        Ok(())
    }

    /// Print [tui_input::Input] without cursor
    pub fn print(&self, stdout: &mut StdoutLock) -> Result<(), RunnerError> {
        let width = self.width as usize;
        let (column, row) = self.position;

        let mut print_str = String::from(self.input.value());
        if print_str.len() <= width {
            print_str.push_str(
                &vec![' '; width - print_str.len() + 1]
                    .into_iter()
                    .collect::<String>(),
            );
        }
        queue!(stdout, MoveTo(column, row), style::Print(print_str))?;
        stdout.flush()?;
        Ok(())
    }

    /// Record current input to history and total string, and reset input for next
    pub fn record(&mut self) {
        // record current input
        if self.input.value().len() != 0 {
            self.history.push(self.input.value().to_string());
            self.his_index = None;
        }
        // push in total
        if self.total.len() != 0 {
            self.total.push('\n');
        }
        self.total.push_str(self.input.value());
        self.input.reset();
    }

    /// Set input to pre history
    pub fn set_pre_history(&mut self) {
        let his_idx = if self.his_index.is_none() {
            let len = self.history.len();
            if len == 0 {
                return;
            }
            len.saturating_sub(1)
        } else {
            self.his_index.unwrap().saturating_sub(1)
        };
        self.his_index = Some(his_idx);
        self.input = Input::new(self.history[his_idx].clone());
    }

    /// Set input to next history
    pub fn set_next_history(&mut self) {
        if self.his_index.is_none() {
            return;
        }
        let his_idx = (self.his_index.unwrap() + 1).min(self.history.len().saturating_sub(1));
        self.his_index = Some(his_idx);
        self.input = Input::new(self.history[his_idx].clone());
    }
}

pub fn get_input(expr_input: &mut ExprInput) -> Result<Option<Expr>, RunnerError> {
    let mut stdout = stdout().lock();
    // enter input and set input
    enter_input_mode(&mut stdout)?;
    // position and width will help to show the input
    let position = cursor::position()?;
    let width = 50.min(terminal::size()?.0) - 2;

    expr_input.position = position;
    expr_input.width = width;

    expr_input.write(&mut stdout)?;

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
            // if input control + c, will exit program
            if let (KeyCode::Char('c'), KeyModifiers::CONTROL) = (code, modifiers) {
                leave_input_mode(&mut stdout)?;
                queue!(
                    stdout,
                    style::PrintStyledContent(
                        "error"
                            .with(style::Color::Red)
                            .attribute(style::Attribute::Bold)
                    ),
                    style::Print(": exit by CONTROL C\n")
                )?;
                return Ok(Some(parse_expr("(exit 101)")?));
            }
            match code {
                KeyCode::Esc => {
                    leave_input_mode(&mut stdout)?;
                    return Ok(None);
                }
                KeyCode::Up => {
                    expr_input.set_pre_history();
                    expr_input.write(&mut stdout)?;
                }
                KeyCode::Down => {
                    expr_input.set_next_history();
                    expr_input.write(&mut stdout)?;
                }
                KeyCode::Enter => {
                    expr_input.print(&mut stdout)?;
                    expr_input.record();

                    match parse_expr(expr_input.expr()) {
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
                            expr_input.position = cursor::position()?;
                            expr_input.write(&mut stdout)?;
                        }
                        Err(err) => {
                            leave_input_mode(&mut stdout)?;
                            return Err(RunnerError::from(err));
                        }
                    }
                }
                _ => {
                    if expr_input.handle_input_event(&event).is_some() {
                        expr_input.write(&mut stdout)?;
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
