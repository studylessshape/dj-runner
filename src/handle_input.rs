use anyhow::Result;
use crossterm::{
    cursor::{self, Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use dj::{parse, Expr, comptime::{Error as CompileError, lexer::Error as LexError, parser::Error as ParseError}, };
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
    is_cut: bool,
}

impl ExprInput {
    pub fn new(is_cut: bool) -> Self {
        Self {
            is_cut,
            ..Default::default()
        }
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
    pub fn write(&self, stdout: &mut StdoutLock) -> Result<()> {
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
    pub fn print(&self, stdout: &mut StdoutLock) -> Result<()> {
        let width = self.width as usize;
        let (column, row) = self.position;

        let mut print_str = if self.is_cut {
            self.input.value()[..self.input.cursor()].to_string()
        } else {
            self.input.value().to_string()
        };
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
        let push_expr = if self.is_cut {
            self.input.value()[..self.input.cursor()]
                .trim_end()
                .to_string()
        } else {
            self.input.value().trim_end().to_string()
        };
        // record current input
        if !self.input.value().is_empty() {
            self.history.push(push_expr.clone()); 
            self.his_index = None;
        }
        // push in total
        if !self.total.is_empty() {
            self.total.push('\n');
        }
        self.total.push_str(&push_expr);
    }

    /// Clear input
    pub fn clear(&mut self) {
        if self.is_cut {
            let after_expr = self.input.value()[self.input.cursor()..].to_string();
            self.input = Input::new(after_expr).with_cursor(0);
        } else {
            self.input.reset();
        }
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

pub fn get_input(expr_input: &mut ExprInput) -> Result<Option<Expr>> {
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
                return Ok(Some(parse("(exit 101)")?));
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
                    expr_input.record();
                    expr_input.print(&mut stdout)?;
                    expr_input.clear();

                    match parse(expr_input.expr()) {
                        Ok(expr) => {
                            // leave input
                            leave_input_mode(&mut stdout)?;
                            return Ok(Some(expr));
                        }
                        Err(CompileError::Lex(LexError::UnclosedString)) | Err(CompileError::Parse(ParseError::UnclosedGroup)) => {
                            queue!(stdout, style::Print("\n\r"))?;
                            stdout.flush()?;
                            expr_input.position = cursor::position()?;
                            expr_input.write(&mut stdout)?;
                        }
                        Err(err) => {
                            leave_input_mode(&mut stdout)?;
                            return Err(err.into());
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

fn enter_input_mode(stdout: &mut StdoutLock) -> Result<()> {
    enable_raw_mode()?;
    queue!(stdout, style::Print("> "), Hide)?;
    stdout.flush()?;
    Ok(())
}

fn leave_input_mode(stdout: &mut StdoutLock) -> Result<()> {
    disable_raw_mode()?;
    queue!(stdout, Show, style::Print("\n"))?;
    stdout.flush()?;
    Ok(())
}
