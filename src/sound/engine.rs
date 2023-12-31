use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::prelude::*;

use super::metadata::PlaybackMeta;

struct ActiveCue {
    sink: Arc<Sink>,
    meta: Arc<Mutex<PlaybackMeta>>,
}

pub struct AudioEngine {
    _output_stream: OutputStream,
    output_handle: OutputStreamHandle,
    sinks: HashMap<String, ActiveCue>,
}

#[allow(dead_code)]
impl AudioEngine {
    pub fn try_init_default() -> Result<Self> {
        let (_output_stream, output_handle) = OutputStream::try_default()?;

        Ok(Self {
            _output_stream,
            output_handle,
            sinks: HashMap::new(),
        })
    }

    pub fn output_handle(&self) -> &OutputStreamHandle {
        &self.output_handle
    }

    pub fn get_sink(&self, k: &str) -> Option<Arc<Sink>> {
        self.sinks.get(k).and_then(|f| Some(f.sink.clone()))
    }

    pub fn add_sink(&mut self, k: impl ToString, sink: Sink, meta: Arc<Mutex<PlaybackMeta>>) {
        let sink = Arc::new(sink);

        self.sinks.insert(k.to_string(), ActiveCue { sink, meta });
    }

    pub fn take_sink(&mut self, k: &str) -> Option<Arc<Sink>> {
        self.sinks.remove(k).and_then(|s| Some(s.sink))
    }

    pub fn stop_all(&mut self) {
        for (_, s) in self.sinks.drain() {
            s.sink.stop()
        }
    }

    pub fn metadata(&self) -> impl Iterator<Item = (&str, &Arc<Mutex<PlaybackMeta>>)> + '_ {
        self.sinks.iter().map(|(k, v)| (k.as_str(), &v.meta))
    }

    pub fn gc(&mut self) {
        self.sinks.retain(|_, s| !s.sink.empty())
    }
}
