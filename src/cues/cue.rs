use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::actions::*;

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Cue {
    label: String,

    #[serde(default)]
    description: String,

    #[serde(default)]
    hint: String,

    #[serde(flatten)]
    action: CueAction,
}

impl Cue {
    pub fn action(&self) -> &CueAction {
        &self.action
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn label(&self) -> &str {
        self.label.as_ref()
    }

    pub fn hint(&self) -> &str {
        self.hint.as_ref()
    }
}

impl Cue {
    pub fn new(label: impl ToString, action: impl Into<CueAction>) -> Self {
        Self {
            label: label.to_string(),
            description: "".to_string(),
            hint: "".to_string(),
            action: action.into()
        }
    }

    pub fn with_description(mut self, desc: impl ToString) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_hint(mut self, context: impl ToString) -> Self {
        self.hint = context.to_string();
        self
    }
}

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CueAction {

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Playback(PlaybackCue),

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Fade(FadeCue),

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Stop(StopCue),
}

impl From<StopCue> for CueAction {
    fn from(v: StopCue) -> Self {
        Self::Stop(v)
    }
}

impl From<FadeCue> for CueAction {
    fn from(v: FadeCue) -> Self {
        Self::Fade(v)
    }
}

impl From<PlaybackCue> for CueAction {
    fn from(v: PlaybackCue) -> Self {
        Self::Playback(v)
    }
}
