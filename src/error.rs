#[allow(unused)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] serde_yaml::Error)
}
