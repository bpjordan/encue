use std::time::Duration;

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

    let show = Script::new(vec![
        Cue::new(
            "SQ1",
            cue::CueAction::Playback(
                actions::PlaybackCue::new("sound.wav")
                    .for_duration(Duration::from_secs(45))
                    .fade_in_for(Duration::from_secs(5))
                    .fade_out_for(Duration::from_secs(10))
            )
        ),
        Cue::new(
            "SQ2",
            cue::CueAction::Fade(
                actions::FadeCue::new("SQ1")
                    .to_volume(40)
                    .for_duration(Duration::from_secs(10))
            )
        ),
        Cue::new(
            "SQ3",
            cue::CueAction::Stop(
                actions::StopCue::new("all")
            )
        ),
    ]).with_master(80);

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
- label: SQ3
  stop:
    target: all
";

    let show = Script::new(vec![
        Cue::new(
            "SQ1",
            cue::CueAction::Playback(
                actions::PlaybackCue::new("sound.wav")
            )
        ),
        Cue::new(
            "SQ2",
            cue::CueAction::Fade(
                actions::FadeCue::new("SQ1")
            )
        ),
        Cue::new(
            "SQ3",
            cue::CueAction::Stop(
                actions::StopCue::new("all")
            )
        ),
    ]).with_master(100);

    let de = serde_yaml::from_str::<Script>(yaml).expect("Failed to deserialize");
    
    assert_eq!(de, show);
}

#[test]
fn deserialize_strings() {
    let yaml = "
cuelist:
- label: SQ1
  playback: sound.wav
- label: SQ2
  fade: SQ1
- label: SQ3
  stop: all
";

    let show = Script::new(vec![
        Cue::new(
            "SQ1",
            cue::CueAction::Playback(
                actions::PlaybackCue::new("sound.wav")
            )
        ),
        Cue::new(
            "SQ2",
            cue::CueAction::Fade(
                actions::FadeCue::new("SQ1")
            )
        ),
        Cue::new(
            "SQ3",
            cue::CueAction::Stop(
                actions::StopCue::new("all")
            )
        ),
    ]).with_master(100);

    let de = serde_yaml::from_str::<Script>(yaml).expect("Failed to deserialize");
    
    assert_eq!(de, show);
}

