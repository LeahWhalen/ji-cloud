use futures_signals::signal::{Mutable};
use shared::{domain::audio::AudioId};
use super::recorder::AudioRecorder;
use super::options::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioInputMode {
    Playing(AudioId),
    Stopped(AudioId),
    Empty,
    Recording,
    Uploading,
    // Paused(AudioId, Timecode) we don't have a design for this but might be useful
}

#[derive(Clone, Copy, PartialEq)]
pub enum AudioInputAddMethod {
    Record,
    Upload,
}

pub struct State {
    //on_change is called imperatively for every update
    //for example, to push to history
    pub on_change: Option<Box<dyn Fn(Option<AudioId>)>>,
    //audio_id is a mutable for affecting DOM
    //intermediate updates can be skipped
    pub mode: Mutable<AudioInputMode>,
    pub add_method: Mutable<AudioInputAddMethod>,
    pub recorder: AudioRecorder,
}

impl State {
    pub fn new(opts: AudioInputOptions) -> Self {
        let mode = match opts.audio_id {
            Some(audio_id) => AudioInputMode::Stopped(audio_id),
            None => AudioInputMode::Empty,
        };

        Self {
            on_change: opts.on_change,
            mode: Mutable::new(mode),
            recorder: AudioRecorder::new(),
            add_method: Mutable::new(AudioInputAddMethod::Record),
        }
    }
}