use std::{collections::HashMap, sync::Arc};

use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::prelude::*;

pub struct AudioEngine {
    _output_stream: OutputStream,
    output_handle: OutputStreamHandle,
    sinks: HashMap<String, Arc<Sink>>,
}

#[allow(dead_code)]
impl AudioEngine {
    pub fn try_init_default() -> Result<Self> {
        let (_output_stream, output_handle) = OutputStream::try_default()?;

        Ok(Self {
            _output_stream,
            output_handle,
            sinks: HashMap::new()
        })
    }

    pub fn output_handle(&self) -> &OutputStreamHandle {
        &self.output_handle
    }

    pub fn get_sink(&self, k: &str) -> Option<Arc<Sink>> {
        self.sinks.get(k).cloned()
    }

    pub fn add_sink(&mut self, k: impl ToString, sink: Sink) {
        let v = Arc::new(sink);

        self.sinks.insert(k.to_string(), v);
    }

    pub fn take_sink(&mut self, k: &str) -> Option<Arc<Sink>> {
        self.sinks.remove(k)
    }

    pub fn stop_all(&mut self) {
        for (_, s) in self.sinks.drain() {
            s.stop()
        }
    }

    pub fn gc(&mut self) {
        self.sinks.retain(|_, s| !s.empty())
    }
}
