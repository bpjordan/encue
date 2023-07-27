
pub use crate::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub type TerminalBackend = ratatui::prelude::CrosstermBackend<std::io::Stdout>;

