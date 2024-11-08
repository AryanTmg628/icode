pub mod editor {
    use std::fs::File;
    use std::io;
    use std::process;

    use crossterm::style::Attribute;
    use crossterm::style::{Color, Stylize};
    use crossterm::{cursor, event, style, terminal, ExecutableCommand};

    enum Action {
        QUIT,
        UP,
        DOWN,
        LEFT,
        RIGHT,
        CHANGEMODE,
    }

    #[derive(Debug)]
    enum Modes {
        Normal,
        Insert,
    }
    pub struct Editor {
        pub buffer: (File, Vec<String>),
        pub stdout: io::Stdout,
        pub cx: u16,
        pub cy: u16,
        mode: Modes,
    }

    impl Editor {
        pub fn new(stdout: io::Stdout, buffer: (File, Vec<String>)) -> Self {
            Editor {
                stdout,
                buffer,
                cx: 0,
                cy: 0,
                mode: Modes::Normal,
            }
        }

        pub fn move_cursor(&mut self) {
            _ = self.stdout.execute(cursor::MoveTo(self.cx, self.cy));
        }

        pub fn draw_buffer(&mut self) {
            _ = self
                .stdout
                .execute(terminal::Clear(terminal::ClearType::All));
            for (i, line) in self.buffer.1.iter().enumerate() {
                _ = self.stdout.execute(style::Print(line));
                _ = self.stdout.execute(cursor::MoveTo(0, self.cy + i as u16));
            }
            _ = self.stdout.execute(cursor::MoveTo(0, 0));
        }

        pub fn draw_status_line(&mut self) -> Result<(), io::Error> {
            // moving to the buttom of terminal and printing the status line
            let terminal_size = terminal::size();

            match terminal_size {
                Ok((t_column, t_row)) => {
                    let file = " src/main.rs ";

                    let mode = format!(" {:?} ", self.mode).to_uppercase();
                    let file_width = t_column as usize - file.len() as usize - mode.len() as usize;

                    let pos = format!(" {}:{} ", self.cx, self.cy);

                    _ = self
                        .stdout
                        .execute(cursor::MoveTo(t_column - t_column, t_row));

                    // if insert mode printing insert else normal

                    _ = self
                        .stdout
                        .execute(terminal::Clear(terminal::ClearType::CurrentLine)); // clearing
                                                                                     // the line before prining the mode

                    _ = self.stdout.execute(style::PrintStyledContent(
                        mode.with(Color::Black)
                            .on(Color::Blue)
                            .attribute(Attribute::Bold),
                    ))?;
                    _ = self.stdout.execute(style::PrintStyledContent(
                        format!("{:<width$}", file, width = file_width as usize)
                            .with(Color::White)
                            .on(Color::Rgb {
                                r: 67,
                                g: 70,
                                b: 89,
                            })
                            .attribute(Attribute::Bold),
                    ))?;
                    _ = self.stdout.execute(style::PrintStyledContent(
                        pos.with(Color::Black)
                            .on(Color::Blue)
                            .attribute(Attribute::Bold),
                    ))?;
                }
                Err(error) => println!("Could not get the terminal size. {}", error),
            }

            // after printing the status line move cursor to 0,0
            _ = self.stdout.execute(cursor::MoveTo(self.cx, self.cy));

            Ok(())
        }

        pub fn handle_normal_event(&mut self, code: event::KeyCode) {
            match code {
                event::KeyCode::Char('q') => process::exit(0),
                event::KeyCode::Up | event::KeyCode::Char('k') => {
                    self.cy = self.cy.saturating_sub(1);
                    self.move_cursor();
                }
                event::KeyCode::Down | event::KeyCode::Char('j') => {
                    self.cy = self.cy.saturating_add(1);
                    self.move_cursor();
                }
                event::KeyCode::Left | event::KeyCode::Char('h') => {
                    self.cx = self.cx.saturating_sub(1);
                    self.move_cursor();
                }
                event::KeyCode::Right | event::KeyCode::Char('l') => {
                    self.cx = self.cx.saturating_add(1);
                    self.move_cursor();
                }
                event::KeyCode::Char('i') => {
                    self.mode = Modes::Insert; // when user press i in normal mode we enter the
                                               // insert mode

                    self.handle_event();
                }
                _ => {}
            }
        }

        pub fn handle_insert_event(&mut self, code: event::KeyCode) {
            match code {
                event::KeyCode::Char(c) => {
                    println!("{}", c);
                    self.cx += 1;
                    self.move_cursor();
                }
                event::KeyCode::Enter => {
                    self.cx = 0;
                    self.cy += 1;
                    self.move_cursor();
                }
                event::KeyCode::Backspace => {
                    self.cx -= 1;
                    self.move_cursor();
                    print!(" ");
                }
                event::KeyCode::Esc => {
                    self.mode = Modes::Normal; // when user press i in normal mode we enter the
                                               // insert mode

                    self.handle_event();
                }
                _ => {}
            }
        }

        pub fn handle_keyboard_event(&mut self, code: event::KeyCode) {
            // checking the mode
            match self.mode {
                Modes::Normal => self.handle_normal_event(code),
                Modes::Insert => self.handle_insert_event(code),
            }
        }

        pub fn handle_event(&mut self) {
            // first printing the status line

            let _ = self.draw_buffer();
            let _ = self.draw_status_line();

            // reading the event
            match event::read().expect("Couldn't read event") {
                event::Event::Key(event) => self.handle_keyboard_event(event.code),
                _ => println!("Only keyboard events are supported for now!!!!!"),
            }
        }

        pub fn run(&mut self) {
            // Entering into the raw mode

            _ = terminal::enable_raw_mode();
            _ = self.stdout.execute(terminal::EnterAlternateScreen);
            _ = self.stdout.execute(terminal::SetTitle("Icode Editor"));

            loop {
                // handling every events
                self.handle_event();
            }
        }
    }
}
