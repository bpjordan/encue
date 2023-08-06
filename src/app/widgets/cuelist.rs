use ratatui::{
    widgets::{ListItem, List, Block, Borders, BorderType}, text::{Span, Line}};

use crate::cues::Cue;

pub fn cue_list(cuelist: &[Cue]) -> List {
    let items: Vec<_> = cuelist
        .iter().map(|c| {
            ListItem::new(Line::from(vec![
                Span::raw(format!(" {:<10}", c.label())),
                Span::raw(c.description())
            ]))
        }).collect();

    List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Cues")
        )
        .highlight_symbol(">>")
        .to_owned()
}
