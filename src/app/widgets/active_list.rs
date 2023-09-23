use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, StatefulWidget, Widget};

use crate::sound::AudioEngine;

#[derive(Debug, Default)]
pub struct ActiveCueList<'b> {
    block: Option<Block<'b>>,
}

impl<'b> ActiveCueList<'b> {
    pub fn block(mut self, block: Block<'b>) -> Self {
        self.block = Some(block);
        self
    }
}

impl StatefulWidget for ActiveCueList<'_> {
    type State = AudioEngine;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut list = List::new(
            state
                .metadata()
                .map(|(l, _)| ListItem::new(l))
                .collect::<Vec<_>>(),
        );

        if let Some(b) = self.block {
            list = list.block(b)
        }

        Widget::render(list, area, buf)
    }
}

pub fn active_cues() -> ActiveCueList<'static> {
    ActiveCueList::default().block(
        Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .title("Active Cues"),
    )
}
