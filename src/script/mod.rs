
mod types;
mod validation;
mod helpers;

use std::{io, fs};

pub use types::*;

use crate::prelude::*;

pub fn load() -> Result<Script> {
    let f = fs::File::open("script.yaml")?;
    let yaml = io::read_to_string(f)?;

    Ok(serde_yaml::from_str(&yaml)?)
}
