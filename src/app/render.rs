use crate::{logging::LogWidget, prelude::*};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::{widgets::active_cues, AppState};

pub fn render<B: Backend>(term: &mut Terminal<B>, app: &mut AppState) -> Result<()> {
    term.draw(|f| {
        let [top, main, bottom, keys] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Min(10),
                Constraint::Percentage(25),
                Constraint::Length(3),
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

        *app.list_height_mut() = main.height.checked_sub(2).unwrap_or(0);

        f.render_stateful_widget(app.widget().clone(), main, app.list_state_mut());
        f.render_widget(rta(), top_left);
        f.render_widget(clock(), top_mid);
        f.render_stateful_widget(active_cues(), top_right, app.engine_mut());
        f.render_widget(hotkey_guide(), keys);
        if let Ok(mut state) = app.logger_state().lock() {
            f.render_stateful_widget(logger(), bottom, &mut state)
        }
    })?;

    Ok(())
}

fn logger<'a>() -> LogWidget<'a> {
    LogWidget::default().block(Block::default().borders(Borders::ALL).title("Log"))
}

fn rta() -> impl Widget {
    Paragraph::new("Placeholder")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Output"))
}

fn clock() -> impl Widget {
    Paragraph::new("Placeholder")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Clock"))
}

fn hotkey_guide() -> impl Widget {
    Paragraph::new(
        "[q] Quit | [s]: Stop All | [j]: Select Next | [k]: Select Prev | [<Space>]: Run Selected",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
}
