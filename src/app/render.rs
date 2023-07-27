
use std::time::Duration;

use crossterm::event::{Event, self, KeyCode};
use log::LevelFilter;
use ratatui::{prelude::*, widgets::{Widget, Paragraph, Block, Borders}};
use crate::{prelude::*, script::Script, logging::{TuiLogger, LogWidget}};

pub fn render_loop<B: Backend>(terminal: &mut Terminal<B>, script: &Script) -> Result<()> {

    let log_widget_state = TuiLogger::init(LevelFilter::Trace)?;

    loop {
        terminal.draw(|f| {
            let [top, main, bottom] = *Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(60),
                    Constraint::Percentage(15),
                ].as_ref())
                .split(f.size())
            else {
                return;
            };

            let [top_left, top_mid, top_right] = *Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Length(30),
                    Constraint::Min(15),
                ].as_ref())
                .split(top)
            else {
                return;
            };

            f.render_widget(cue_list(script), main);
            f.render_widget(rta(), top_left);
            f.render_widget(clock(), top_mid);
            f.render_widget(active_list(), top_right);
            if let Ok(mut state) = log_widget_state.lock() {
                f.render_stateful_widget(logger(), bottom, &mut state)
            }

        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(k) = event::read()? {
                log::trace!("Key pressed: {:?}", k.code);
                if let KeyCode::Char('q') = k.code {
                    return Ok(())
                }
            }
        }
    }
}

fn cue_list(_script: &Script) -> impl Widget {

    Paragraph::new("Placeholder

press q to quit")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cuelist")
        )
}

fn logger<'a>() -> LogWidget<'a> {

    LogWidget::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Log")
        )
}

fn rta() -> impl Widget {

    Paragraph::new("Placeholder")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Output")
        )
}

fn active_list() -> impl Widget {

    Paragraph::new("Placeholder")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Active Cues")
        )
}

fn clock() -> impl Widget {

    Paragraph::new("Placeholder")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Clock")
        )
}
