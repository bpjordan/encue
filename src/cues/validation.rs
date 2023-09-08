
use crate::prelude::*;
use super::{Script, cue::CueAction};

impl Script {
    pub fn validate(self) -> Result<Self> {

        let mut valid_targets = self.cue_names();
        valid_targets.push("all");

        for cue in self.cuelist() {
            match cue.action() {
                CueAction::Playback(c) => {
                        if !c.file().exists() {
                            return Err(FatalError::CueFile(cue.label().to_string(), c.file().clone()))
                        }
                    },
                CueAction::Fade(c) => {
                        if !valid_targets.contains(&c.target()) {
                            return Err(FatalError::CueTarget(cue.label().to_string(), c.target().to_string()))
                        }
                    },
                CueAction::Stop(c) => {
                    if !valid_targets.contains(&c.target()) {
                        return Err(FatalError::CueTarget(cue.label().to_string(), c.target().to_string()))
                    }
                },
                _ => {}
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_file() {
    }
}
