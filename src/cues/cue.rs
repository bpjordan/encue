use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::sound::{ExecutableCue, PrepareCue};

use super::actions::*;

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Cue {
    label: String,

    #[serde(default)]
    description: String,

    #[serde(default)]
    #[serde(alias = "cue")]
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

#[allow(dead_code)]
impl Cue {
    pub fn new(label: impl ToString, action: impl Into<CueAction>) -> Self {
        Self {
            label: label.to_string(),
            description: "".to_string(),
            hint: "".to_string(),
            action: action.into(),
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
    Playlist(PlaylistCue),

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Playback(PlaybackCue),

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Fade(FadeCue),

    #[serde(deserialize_with = "crate::util::serde::string_or_struct::deserialize")]
    Stop(StopCue),
    // Group(CueGroup),
}

impl From<PlaylistCue> for CueAction {
    fn from(v: PlaylistCue) -> Self {
        Self::Playlist(v)
    }
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

impl CueAction {
    pub fn prepare(
        &self,
        label: Option<&str>,
    ) -> Result<ExecutableCue, Box<dyn Error + Send + Sync>> {
        match self {
            CueAction::Playlist(p) => Ok(ExecutableCue::Playback(p.prepare(label)?)),
            CueAction::Playback(p) => Ok(ExecutableCue::Playback(p.prepare(label)?)),
            CueAction::Fade(f) => Ok(ExecutableCue::Fade(f.clone())),
            CueAction::Stop(s) => Ok(ExecutableCue::Stop(s.clone())),
        }
    }
}
