
use crate::prelude::*;
use super::Script;

impl Script {
    pub fn validate(self) -> Result<Self> {

        let mut valid_targets = self.cue_names();
        valid_targets.push("all");

        for cue in self.cuelist() {
            match cue.action() {
                super::CueAction::Playback {
                    file,
                    ..
                } => {
                        if !file.exists() {
                            return Err(Error::CueFile(cue.label().to_string(), file.clone()))
                        }
                    },
                super::CueAction::Fade {
                    target,
                    ..
                } => {
                        if !valid_targets.contains(&target.as_str()) {
                            return Err(Error::CueTarget(cue.label().to_string(), target.clone()))
                        }
                    },
                super::CueAction::Stop(target) => {
                    if !valid_targets.contains(&target.as_str()) {
                        return Err(Error::CueTarget(cue.label().to_string(), target.clone()))
                    }
                },
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
