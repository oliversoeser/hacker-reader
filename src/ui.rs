use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from(" Hacker Reader ".bold());
    let instructions = Line::from(vec![
        " Down ".into(),
        "<J> ".blue().bold(),
        "Up ".into(),
        "<K> ".blue().bold(),
        "Quit ".into(),
        "<Q> ".blue().bold()
    ]).centered();

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions)
        .border_set(border::THICK);

    let paragraph = Paragraph::new(app.body)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((app.pos, 0));

    frame.render_widget(paragraph, frame.area());
}
