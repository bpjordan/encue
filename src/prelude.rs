
pub use crate::error::FatalError;

pub type Result<T, E = FatalError> = core::result::Result<T, E>;

