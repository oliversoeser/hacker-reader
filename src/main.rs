use std::io;

use ratatui::DefaultTerminal;
use tokio;

mod app;
mod ui;
use crate::{
    ui::ui,
    app::App,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    ratatui::restore();
    res
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    app.start_fetch();

    while !app.exit {
        terminal.draw(|frame| ui(frame, app))?;
        app.handle_events()?;
    }

    Ok(())
}
