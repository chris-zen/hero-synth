#![allow(dead_code)]

extern crate portaudio;
extern crate portmidi;
// extern crate coremidi;

extern crate hero_core;
extern crate hero_synth;

mod audio;
mod midi;
mod engine;
mod control;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use audio::{SAMPLE_RATE, audio_start, audio_close};
use midi::Midi;
use engine::Engine;
use control::Control;

fn main() {

    // let (ctrl_notes_tx, ctrl_notes_rx): (Sender<DeviceNoteEvents>, Receiver<DeviceNoteEvents>) = channel();
    let (engine_notes_tx, engine_notes_rx): (Sender<engine::DeviceEvents>, Receiver<engine::DeviceEvents>) = channel();

    let mut engine = Engine::new(SAMPLE_RATE);
    engine.start(engine_notes_rx);

    let engine_mutex = Arc::new(Mutex::new(engine));

    let (midi_input_tx, midi_input_rx): (Sender<midi::DeviceEvents>, Receiver<midi::DeviceEvents>) = channel();

    let mut midi = Midi::new();
    midi.start(midi_input_tx);

    let mut control = Control::new();
    control.start(midi_input_rx, engine_notes_tx);

    let pa_ctx = portaudio::PortAudio::new().unwrap();
    let mut stream = audio_start(&pa_ctx, engine_mutex.clone()).unwrap();

    // Loop while the non-blocking stream is active.
    while let Ok(true) = stream.is_active() {
        pa_ctx.sleep(1000);
    }

    audio_close(&mut stream).unwrap();

    control.stop();

    midi.stop();

    engine_mutex.lock().unwrap().stop();

    println!("");
}
