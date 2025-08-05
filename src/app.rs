use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default)]
pub struct App {
    pub pos: u16,
    pub body: &'static str,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App { pos: 0, body: "Hello World!".into(), exit: false }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {},
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') | KeyCode::Up => self.up(),
            KeyCode::Char('j') | KeyCode::Down => self.down(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {},
        }
    }
    
    fn up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn down(&mut self) {
        self.pos += 1;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}