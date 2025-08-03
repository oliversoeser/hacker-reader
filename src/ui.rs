use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::Block,
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let title = Line::from(" Hello World! ".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()])
        .centered();

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions)
        .border_set(border::THICK);

    frame.render_widget(block, frame.area());
}
