
use ratatui::{prelude::*, widgets::{Widget, Paragraph, Block, Borders}};
use crate::{prelude::*, logging::LogWidget};

use super::AppState;

pub fn render<B: Backend>(term: &mut Terminal<B>, app: &mut AppState) -> Result<()> {

    term.draw(|f| {
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

        f.render_stateful_widget(app.widget().clone(), main, app.list_state_mut());
        f.render_widget(rta(), top_left);
        f.render_widget(clock(), top_mid);
        f.render_widget(active_list(), top_right);
        if let Ok(mut state) = app.logger_state().lock() {
            f.render_stateful_widget(logger(), bottom, &mut state)
        }

    })?;

    Ok(())
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

