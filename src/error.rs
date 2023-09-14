use std::{path::PathBuf, sync::mpsc};

use crate::app::events::Event;

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum FatalError {
    #[error("{0}")]
    Generic(&'static str),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] serde_yaml::Error),

    #[error("Invalid target in {0}: {1}")]
    CueTarget(String, String),

    #[error("Invalid audio file in {0}: {1}")]
    CueFile(String, PathBuf),

    #[error("Problem drawing interface")]
    Render,

    #[error(transparent)]
    SetLogger(#[from] log::SetLoggerError),

    #[error(transparent)]
    RecvEvent(#[from] mpsc::RecvError),

    #[error("Event listener thread couldn't communicate with main thread: {0}")]
    SendEvent(#[from] mpsc::SendError<Event>),

    #[error("A thread panicked")]
    ThreadPanic,

    #[error(transparent)]
    OutputSetup(#[from] rodio::StreamError),
}
