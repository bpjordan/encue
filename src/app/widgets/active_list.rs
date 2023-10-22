use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Row, StatefulWidget, Table, Widget};

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
        let mut list = Table::new(
            state
                .metadata()
                .map(|(l, m)| {
                    let meta = m.lock().unwrap();

                    let elapsed = meta.start.elapsed();
                    let total = meta.duration;

                    let timestamp = format!(
                        "{:02}:{:02}/{:02}:{:02}",
                        elapsed.as_secs().saturating_div(60),
                        elapsed.as_secs().wrapping_rem(60),
                        total.as_secs().saturating_div(60),
                        total.as_secs().wrapping_rem(60)
                    );

                    Row::new(vec![l.to_string(), timestamp])
                })
                .collect::<Vec<_>>(),
        )
        .widths(&[Constraint::Min(10), Constraint::Length(11)]);

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
