#[allow(dead_code)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Script {
    cuelist: Vec<Cue>,

    #[serde(default = "default_vol")]
    master: u8,
}

impl Script {
    pub fn cuelist(&self) -> &[Cue] {
        self.cuelist.as_ref()
    }

    pub fn master(&self) -> u8 {
        self.master
    }
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Cue {
    label: String,

    #[serde(default)]
    description: String,

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
}

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CueAction {
    Playback {
        file: PathBuf,

        duration: Option<u32>,
        fade_in: Option<u32>,
        fade_out: Option<u32>,
    },
    Fade {
        target: String,

        #[serde(default)]
        volume: u8,
        duration: u32
    },
    Stop(String),
}

fn default_vol() -> u8 {
    100
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn deserialize_full() {

        let yaml = "
master: 80
cuelist:
- label: SQ1
  playback:
    file: sound.wav
    duration: 45
    fade_in: 5
    fade_out: 10
- label: SQ2
  fade:
    target: SQ1
    volume: 40
    duration: 10
- label: SQ3
  stop: all
";

        let show = Script {
            cuelist: vec![
                Cue{
                    label: "SQ1".to_string(),
                    description: "".to_string(),
                    action: CueAction::Playback {
                        file: PathBuf::from("sound.wav"),
                        duration: Some(45),
                        fade_in: Some(5),
                        fade_out: Some(10),
                    }
                },
                Cue{
                    label: "SQ2".to_string(),
                    description: "".to_string(),
                    action: CueAction::Fade{
                        target: "SQ1".to_string(),
                        volume: 40,
                        duration: 10
                    }
                },
                Cue {
                    label: "SQ3".to_string(),
                    description: "".to_string(),
                    action: CueAction::Stop("all".to_string())
                },
            ],
            master: 80
        };

        let de = serde_yaml::from_str::<Script>(yaml).expect("Failed to deserialize");
        
        assert_eq!(de, show);
    }

    #[test]
    fn deserialize_with_defaults() {
        let yaml = "
cuelist:
- label: SQ1
  playback:
    file: sound.wav
- label: SQ2
  fade:
    target: SQ1
    duration: 10
- label: SQ3
  stop: all
";

        let show = Script {
            cuelist: vec![
                Cue{
                    label: "SQ1".to_string(),
                    description: "".to_string(),
                    action: CueAction::Playback {
                        file: PathBuf::from("sound.wav"),
                        duration: None,
                        fade_in: None,
                        fade_out: None,
                    }
                },
                Cue{
                    label: "SQ2".to_string(),
                    description: "".to_string(),
                    action: CueAction::Fade{
                        target: "SQ1".to_string(),
                        volume: 0,
                        duration: 10
                    }
                },
                Cue {
                    label: "SQ3".to_string(),
                    description: "".to_string(),
                    action: CueAction::Stop("all".to_string())
                },
            ],
            master: 100
        };

        let de = serde_yaml::from_str::<Script>(yaml).expect("Failed to deserialize");
        
        assert_eq!(de, show);
    }
}
