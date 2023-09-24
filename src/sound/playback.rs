use std::sync::{Arc, Mutex};

use rodio::{queue::SourcesQueueOutput, Sink};

use super::{metadata::PlaybackMeta, ExecuteCue, ExecuteCueError};

pub struct PlaybackExecutable {
    label: Option<String>,
    queue: SourcesQueueOutput<f32>,
    sink: Sink,
    meta: Arc<Mutex<PlaybackMeta>>,
}

impl PlaybackExecutable {
    pub fn new(
        label: Option<String>,
        sink: Sink,
        queue: SourcesQueueOutput<f32>,
        meta: Arc<Mutex<PlaybackMeta>>,
    ) -> Self {
        Self {
            sink,
            queue,
            label,
            meta,
        }
    }
}

impl ExecuteCue for PlaybackExecutable {
    fn execute(self, engine: &mut super::AudioEngine) -> Result<(), ExecuteCueError> {
        engine.output_handle().play_raw(self.queue)?;
        self.sink.play();
        if let Some(label) = self.label {
            engine.add_sink(label, self.sink, self.meta)
        } else {
            self.sink.detach()
        }

        Ok(())
    }
}
