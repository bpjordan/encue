
use std::io;

use ratatui::{prelude::*, widgets::*};

use crate::cues::{Script, Cue};
use crate::logging::LogWidget;

use super::AppState;

pub struct AppTui<'a> {
    main_layout: Layout,
    top_layout: Layout,
    cuelist: Table<'a>,
}

impl<'a> AppTui<'a> {

    pub fn new(script: &'a Script) -> Self {

        Self {
            cuelist: cue_list(script.cuelist()),
            main_layout: Layout::new()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(60),
                    Constraint::Percentage(15),
                ].as_ref()),
            top_layout: Layout::new()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Length(30),
                    Constraint::Min(15),
                ].as_ref()),
        }
    }
}

impl AppTui<'_> {

    pub fn render<B: Backend> (&self, state: &mut AppState, terminal: &mut Terminal<B>) -> io::Result<()> {

        self.render_cuelist(state, terminal)?;
        self.render_log(state, terminal)?;
        self.render_status(state, terminal)?;

        Ok(())
    }

    pub fn render_cuelist<B: Backend>(&self, state: &mut AppState, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let [_, area, _] = *self.main_layout.split(f.size()) else { unreachable!() };

            f.render_stateful_widget(self.cuelist.clone(), area, state.cuelist_state_mut());
        })?;

        Ok(())
    }

    pub fn render_log<B: Backend>(&self, state: &mut AppState, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let [_, _, area] = *self.main_layout.split(f.size()) else { unreachable!() };

            f.render_stateful_widget(LogWidget::default(), area, &mut state.logger_state().lock().unwrap());
        })?;

        Ok(())
    }

    pub fn render_status<B: Backend>(&self, state: &mut AppState, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.draw(|f| {
            let [top, _, _] = *self.main_layout.split(f.size()) else { unreachable!() };

            let [top_left, top_mid, top_right] = *self.top_layout.split(top) else { unreachable!(); };

            f.render_widget(rta(), top_left);
            f.render_widget(clock(), top_mid);
            f.render_widget(active_list(), top_right);
        })?;

        Ok(())
    }


}

fn cue_list(cuelist: &[Cue]) -> Table {
    let items: Vec<_> = cuelist
        .iter().map(|c| {
            Row::new(vec![
                Cell::from(c.label()),
                Cell::from(c.description()),
            ])
        }).collect();

    Table::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Cues")
        )
        .widths(&[Constraint::Length(10), Constraint::Percentage(100)])
        .column_spacing(1) 
        .header(
            Row::new(vec![
                Cell::from("Label"),
                Cell::from("Description")
            ])
                .style(Style::new().underlined())
        )
        .highlight_symbol(">> ")
        .highlight_style(Style::default().bg(Color::DarkGray))
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

