
use std::{str::FromStr, convert::Infallible};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StopCue {
    target: String,
}

impl StopCue {
    pub fn target(&self) -> &str {
        self.target.as_ref()
    }
}

impl StopCue {
    pub fn new(target: impl ToString) -> Self {
        Self { target: target.to_string() }
    }
}

impl FromStr for StopCue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
