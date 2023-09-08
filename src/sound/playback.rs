use rodio::{Sink, queue::SourcesQueueOutput};

use super::{ExecuteCue, ExecuteCueError};


pub struct PlaybackExecutable {
    label: Option<String>,
    queue: SourcesQueueOutput<f32>,
    sink: Sink
}

impl PlaybackExecutable {
    pub fn new(label: Option<String>, sink: Sink, queue: SourcesQueueOutput<f32>) -> Self {
        Self {
            sink,
            queue,
            label,
        }
    }
}

impl ExecuteCue for PlaybackExecutable {
    fn execute(self, engine: &mut super::AudioEngine) -> Result<(), ExecuteCueError> {
        engine.output_handle().play_raw(self.queue)?;
        self.sink.play();
        if let Some(label) = self.label {
            engine.add_sink(label, self.sink)
        } else {
            self.sink.detach()
        }

        Ok(())
    }
}
