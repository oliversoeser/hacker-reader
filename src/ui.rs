use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let title = Line::from(" Hacker Reader ".bold());
    let instructions = Line::from(vec![
        " Down ".into(),
        "<J> ".blue().bold(),
        "Up ".into(),
        "<K> ".blue().bold(),
        "Quit ".into(), 
        "<Q> ".blue().bold()
    ]).centered();

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin dapibus est id libero sollicitudin fringilla. Nulla id lacinia lacus. Fusce quis aliquet eros. Etiam elementum tellus mauris. Praesent nec viverra est, et semper diam. Nunc ac justo urna. Morbi ornare eros ut risus tristique, et tristique mi fermentum. Curabitur tincidunt iaculis laoreet. Vestibulum nisl ligula, posuere vulputate metus sit amet, molestie elementum massa. Sed efficitur, arcu aliquet pretium mattis, mi nulla fringilla massa, ac gravida risus neque vitae est. Nullam rhoncus sagittis neque vitae sodales. Proin scelerisque dolor vel felis iaculis, at hendrerit lectus varius. Vivamus commodo commodo pellentesque. Nulla pretium, dui ac molestie efficitur, augue erat iaculis augue, at venenatis mauris urna eu risus. Integer accumsan posuere odio, eu aliquet velit facilisis id.Nulla eleifend sed nulla sit amet porta. Maecenas placerat in ipsum vel tempor. Vestibulum neque enim, varius vel nunc nec, bibendum tristique enim. Integer ullamcorper ante ac enim laoreet suscipit. Maecenas sit amet arcu dolor. Vestibulum ante tellus, dignissim sed fermentum nec, pretium at elit. Sed vitae velit nibh. Nunc pharetra diam at ante imperdiet luctus. Nunc ipsum dui, mattis nec euismod vel, tincidunt at magna. Nunc tempor feugiat risus ac pulvinar. Vestibulum id facilisis nulla, eget mattis sapien.\n Curabitur pellentesque nunc eu bibendum sodales. Vestibulum hendrerit neque ut ante vulputate posuere id congue metus. Fusce efficitur volutpat mollis. Fusce egestas lobortis nibh, ut scelerisque ligula accumsan nec. Suspendisse quis laoreet ligula. Cras pharetra turpis sit amet leo scelerisque placerat. Nunc eleifend urna non libero sollicitudin posuere. Duis eu varius leo. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus ultrices urna ultricies nisi imperdiet, eu viverra tellus accumsan.\n Maecenas semper hendrerit laoreet. Mauris viverra posuere dui nec imperdiet. In venenatis metus eget augue rhoncus posuere. Nullam ac semper est. Ut nec nisi lacinia arcu efficitur facilisis eget a ligula. Praesent eget nisi posuere lacus porta bibendum. Sed sed fringilla mi. Nullam fringilla turpis massa, quis viverra nisi vestibulum in. Etiam orci turpis, dapibus at tincidunt consequat, vestibulum quis tellus. Etiam convallis nec lorem at cursus. Ut a cursus enim, ut consequat sem. Suspendisse luctus diam nulla, ut volutpat mi aliquet a. Ut id lectus arcu. Integer sit amet libero ullamcorper, tincidunt diam quis, mollis risus.\n Morbi nec magna quis turpis congue laoreet. Vivamus interdum neque nisi, eu posuere ipsum aliquet non. Curabitur efficitur sapien sed sem tempus, non porta dui tincidunt. Quisque iaculis mauris ipsum, eget mollis libero eleifend ac. Vestibulum mollis est sed metus maximus condimentum. Maecenas in volutpat leo. Nulla facilisi.";

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions)
        .border_set(border::THICK);

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((app.pos, 0));

    frame.render_widget(paragraph, frame.area());
}
