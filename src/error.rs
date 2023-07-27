use std::path::PathBuf;

#[allow(unused)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error {0}")]
    Generic(String),

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
    SetLogger(#[from] log::SetLoggerError)
}
