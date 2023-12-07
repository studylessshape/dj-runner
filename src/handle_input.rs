use crate::{parse_expr, RunnerError};
use crossterm::{
    cursor::{self},
    event::{
        read, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    queue, style,
};
use dj::{ast::Expr, InterpretError, Token};
use std::io::{stdout, StdoutLock, Write};

struct Input {
    value: String,
    cursor: i32,
    left_bound: i32,
}

impl Input {
    pub fn new(value: String) -> Self {
        Self {
            value,
            cursor: 0,
            left_bound: 0,
        }
    }

    pub fn with_left_bound(&mut self, left_bound: i32) {
        let left_bound = left_bound.clamp(0, left_bound);
        self.cursor = left_bound;
        self.left_bound = left_bound;
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    /// Use [crossterm::queue] to print and don't execute stdout.flush()
    pub fn write(&self, stdout: &mut StdoutLock) -> Result<(), RunnerError> {
        queue!(
            stdout,
            cursor::MoveToColumn(self.left_bound as u16),
            style::Print(self.value()),
            cursor::MoveToColumn(self.cursor as u16)
        )?;
        Ok(())
    }

    fn move_left(&mut self, stdout: &mut StdoutLock) -> Result<(), RunnerError> {
        self.cursor -= 1;
        if self.cursor < self.left_bound {
            self.cursor = self.left_bound;
        } else {
            queue!(stdout, cursor::MoveLeft(1))?;
        }
        Ok(())
    }

    fn move_right(&mut self, stdout: &mut StdoutLock) -> Result<(), RunnerError> {
        self.cursor += 1;
        if (self.cursor - self.left_bound) as usize > self.value.len() {
            self.cursor = self.value.len() as i32 + self.left_bound;
        } else {
            queue!(stdout, cursor::MoveRight(1))?;
        }
        Ok(())
    }

    fn move_to(&mut self, stdout: &mut StdoutLock, offset: i32) -> Result<(), RunnerError> {
        self.cursor = (self.cursor + offset)
            .clamp(self.left_bound, self.value.len() as i32 + self.left_bound);
        queue!(stdout, cursor::MoveToColumn(self.cursor as u16))?;
        Ok(())
    }

    pub fn handle_event(
        &mut self,
        stdout: &mut StdoutLock,
        event: &Event,
    ) -> Result<(), RunnerError> {
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            let position = (self.cursor - self.left_bound) as usize;
            if *kind != KeyEventKind::Press {
                return Ok(());
            }
            return match *code {
                KeyCode::Backspace => {
                    self.move_left(stdout)?;

                    if position == self.value.len() {
                        self.value.pop();
                    } else if position > 0 {
                        self.value.remove(position - 1);
                    }
                    // after move cursor, reprint the string after cursor
                    let position = (self.cursor - self.left_bound) as usize;
                    queue!(
                        stdout,
                        style::Print(
                            self.value
                                .chars()
                                .skip(position)
                                .chain(std::iter::once(' '))
                                .collect::<String>()
                        )
                    )?;
                    Ok(())
                }
                KeyCode::Left => self.move_left(stdout),
                KeyCode::Right => self.move_right(stdout),
                KeyCode::Home => self.move_to(stdout, -(self.value.len() as i32)),
                KeyCode::End => self.move_to(stdout, self.value.len() as i32),
                KeyCode::Delete => {
                    if position != self.value.len() {
                        self.value.remove(position);
                        queue!(
                            stdout,
                            style::Print(
                                self.value
                                    .chars()
                                    .skip(position)
                                    .chain(std::iter::once(' '))
                                    .collect::<String>()
                            )
                        )?;
                    }
                    Ok(())
                }
                KeyCode::Char(c) => {
                    if position == self.value.len() {
                        self.value.push(c);
                    } else {
                        self.value.insert(position, c);
                    }
                    self.move_right(stdout)
                }
                _ => Ok(()),
            };
        }
        Ok(())
    }
}

fn write_input(stdout: &mut StdoutLock, input: &Input) -> Result<(), RunnerError> {
    input.write(stdout)?;
    stdout.flush().map_err(|e| RunnerError::from(e))
}

/// Input support all char and Home, End, Left, Right, Backspace and Delete key.
pub fn get_input() -> Result<Option<Expr>, RunnerError> {
    let mut stdout = stdout().lock();

    queue!(stdout, style::Print("> "))?;
    stdout.flush()?;

    let mut expr_input = String::new();
    let mut input = Input::new(String::new());

    input.with_left_bound(cursor::position()?.0 as i32);
    write_input(&mut stdout, &input)?;

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
                    }
                    expr_input.push_str(input.value());
                    input.clear_value();
                    input.with_left_bound(0);

                    queue!(stdout, style::Print("\n"))?;
                    stdout.flush()?;

                    match parse_expr(&expr_input) {
                        Ok(expr) => return Ok(Some(expr)),
                        Err(InterpretError::Syntax(dj::SyntaxError::RequrieToken(Token::End(
                            _,
                        )))) => {}
                        Err(err) => return Err(RunnerError::from(err)),
                    }
                }
                _ => {
                    input.handle_event(&mut stdout, &event)?;
                }
            }
            write_input(&mut stdout, &input)?;
        }
    }
}
