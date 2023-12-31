use std::{convert::Infallible, ops::Mul, str::FromStr, thread, time::Duration};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{
    sound::{ExecuteCue, ExecuteCueError},
    util::defaults,
};

#[serde_as]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FadeCue {
    target: String,

    #[serde(default)]
    volume: u8,

    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    #[serde(default = "defaults::default_fade_duration")]
    duration: Duration,
}

impl FadeCue {
    pub fn target(&self) -> &str {
        self.target.as_ref()
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }
}

#[allow(dead_code)]
impl FadeCue {
    pub fn new(target: impl ToString) -> Self {
        Self {
            target: target.to_string(),
            volume: Default::default(),
            duration: defaults::default_fade_duration(),
        }
    }

    pub fn to_volume(mut self, volume: u8) -> Self {
        self.volume = volume;
        self
    }

    pub fn for_duration(mut self, duration: impl Into<Duration>) -> Self {
        self.duration = duration.into();
        self
    }
}

impl FromStr for FadeCue {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl ExecuteCue for FadeCue {
    fn execute(self, engine: &mut crate::sound::AudioEngine) -> Result<(), ExecuteCueError> {
        let Some(sink) = engine.get_sink(self.target()) else {
            return Err(ExecuteCueError::MissingTarget(self.target))
        };

        let initial_vol = sink.volume().mul(100_f32).round() as i32;

        let target_vol = self.volume() as i32;

        let steps = initial_vol
            .checked_sub(target_vol)
            .ok_or(ExecuteCueError::General("overflow"))?;

        let fade_rate = self
            .duration()
            .checked_div(steps.unsigned_abs())
            .unwrap_or(self.duration().clone());

        log::trace!(
            "fade from {initial_vol} to {}; {steps} steps with {}ms between",
            self.volume(),
            fade_rate.as_millis()
        );

        let vols = if target_vol > initial_vol {
            itertools::Either::Left(initial_vol..=target_vol)
        } else {
            itertools::Either::Right((target_vol..=initial_vol).rev())
        };

        thread::spawn(move || {
            for current_vol in vols {
                thread::sleep(fade_rate);
                log::trace!("fading {} to {current_vol}", self.target());

                let current_vol = (current_vol as f32) / 100.0;
                sink.set_volume(current_vol)
            }

            if self.volume() == 0 {
                sink.stop();
            }
        });

        Ok(())
    }
}
