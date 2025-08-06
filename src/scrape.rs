use scraper::{
    Html,
    Node,
};

pub fn extract_main_text(html: String) -> String {
    let document = Html::parse_document(&html);
    let values = document.tree.values();
    let mut res = "".to_string();
    for node in values {
        match node {
            Node::Text(t) => {
                let text = t.to_string();
                res.push_str(&text);
                res.push_str("\n");
            },
            _ => ()
        }
    }
    res
}