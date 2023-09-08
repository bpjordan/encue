
use std::{sync::{Arc, Mutex}, time::Instant};

use log::{Log, LevelFilter, Level};
use ratatui::widgets::{StatefulWidget, Widget, Block};

use crate::prelude::*;
use ratatui::prelude::*;

#[derive(Clone)]
pub struct TuiLogger{
    level: LevelFilter,
    state: Arc<Mutex<TuiLoggerState>>,
}

#[derive(Default, Clone)]
pub struct TuiLoggerState(Vec<(Level, Instant, String)>);

impl TuiLoggerState {
    fn push(&mut self, args: (Level, Instant, String)) {
        self.0.push(args)
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

impl TuiLogger {
    pub fn init(log_level: LevelFilter) -> Result<Arc<Mutex<TuiLoggerState>>> {
        log::set_max_level(log_level);

        let logger = Self::new(log_level);

        let history = logger.state.clone();

        log::set_boxed_logger(Box::new(logger))?;

        Ok(history)
    }

    pub fn new(log_level: LevelFilter) -> Self {
        Self {
            level: log_level,
            state: Default::default()
        }
    }
}

#[derive(Default, Clone)]
pub struct LogWidget<'a>{
    block: Option<Block<'a>>
}

impl<'a> LogWidget<'a> {
    pub fn block(mut self, block: Block<'a>) -> LogWidget<'a> {
        self.block = Some(block);
        self
    }
}

impl StatefulWidget for LogWidget<'_> {
    type State = TuiLoggerState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);

                b.render(area, buf);

                inner_area
            },

            None => area
        };

        let history_to_show = state.0
            .iter()
            .rev()
            .take(text_area.height as usize)
            .rev();

        for (y, (level, _time, msg)) in history_to_show.enumerate() {

            let level_color = match level {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Blue,
                Level::Trace => Color::Cyan,
            };

            buf.set_line(text_area.left(), text_area.top() + y as u16,
                &Line::from(vec![
                    Span::from(format!("{level:<5}: ")).bold().fg(level_color),
                    Span::from(msg.to_string())
                ]),
                text_area.width
            );
        }
    }
}

impl Widget for LogWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(
            self,
            area,
            buf,
            &mut TuiLoggerState::default()
        )
    }
}

impl Log for TuiLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target().starts_with("encue") &&
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            let Ok(mut history) = self.state.lock() else {
                return;
            };

            history.push((
                record.level(),
                Instant::now(),
                record.args().to_string(),
            ));
        }
    }

    fn flush(&self) {
        if let Ok(mut history) = self.state.lock() {
            history.clear()
        }
    }
}
