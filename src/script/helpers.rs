
use super::{Script, Cue};

impl Script {
    pub fn cue_names(&self) -> Vec<&str> {
        self.cuelist()
            .iter()
            .map(Cue::label)
            .collect()
    }
}
