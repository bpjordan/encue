use ratatui::{widgets::{Block, Borders, BorderType, Table, Row, Cell}, prelude::Constraint, style::{Style, Color, Stylize}};

use crate::cues::Cue;

pub fn cue_list(cuelist: &[Cue]) -> Table {
    let items: Vec<_> = cuelist
        .iter().map(|c| {
            Row::new(vec![
                Cell::from(c.label()),
                Cell::from(c.description()),
                Cell::from(c.hint()),
            ])
        }).collect();

    Table::new(items)
        .widths(&[Constraint::Length(10), Constraint::Length(25), Constraint::Percentage(100)])
        .header(
            Row::new(vec![
                Cell::from("Label"),
                Cell::from("Description"),
                Cell::from("Cue"),
            ]).style(Style::new().bold().underlined()),
        )
        .highlight_symbol(">> ")
        .highlight_style(Style::new().bg(Color::DarkGray))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Cues")
        )
}
