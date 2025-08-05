use std::io;
use std::sync::{Arc, Mutex};
use std::u16::MAX;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default)]
pub struct App {
    pub pos: u16,
    pub body: Arc<Mutex<String>>,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            pos: 0,
            body: Arc::new(Mutex::new("Loading...".into())),
            exit: false
        }
    }

    pub fn start_fetch(&self) {
        let body_clone = Arc::clone(&self.body);

        tokio::spawn(async move {
            match fetch("https://news.ycombinator.com").await {
                Ok(content) => {
                    if let Ok(mut body) = body_clone.lock() {
                        *body = content;
                    }
                },
                Err(e) => {
                    if let Ok(mut body) = body_clone.lock() {
                        *body = format!("Error fetching webpage: {}", e);
                    }
                }
            }
        });
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
        if self.pos < MAX {
            self.pos += 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

async fn fetch(url: &'static str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(response)
}