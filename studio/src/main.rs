#![allow(dead_code)]

extern crate portaudio;
extern crate portmidi;
extern crate rosc;
extern crate tokio_core;
extern crate futures;

extern crate hero_core;
extern crate hero_synth;

mod audio;
mod midi;
mod osc;
mod engine;
mod control;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use audio::{SAMPLE_RATE, audio_start, audio_close};
use midi::Midi;
use osc::Osc;
use engine::Engine;
use control::Control;

fn main() {

    let (engine_input_tx, engine_input_rx): (Sender<engine::PortEvents>, Receiver<engine::PortEvents>) = channel();
    let (engine_output_tx, engine_output_rx): (Sender<engine::PortEvents>, Receiver<engine::PortEvents>) = channel();

    let mut engine = Engine::new(SAMPLE_RATE);
    engine.start(engine_input_rx, engine_output_tx.clone());

    let engine_mutex = Arc::new(Mutex::new(engine));

    let (midi_input_tx, midi_input_rx): (Sender<midi::PortEvents>, Receiver<midi::PortEvents>) = channel();

    let mut midi = Midi::new();
    midi.start(midi_input_tx);

    let (osc_input_tx, osc_input_rx): (Sender<rosc::OscPacket>, Receiver<rosc::OscPacket>) = channel();
    let (osc_output_tx, osc_output_rx): (Sender<rosc::OscPacket>, Receiver<rosc::OscPacket>) = channel();
    let mut osc = Osc::new("0.0.0.0:7400");
    osc.start(osc_input_tx, osc_output_rx);

    let mut control = Control::new();
    control.start(
        midi_input_rx, osc_input_rx,
        engine_input_tx, engine_output_rx);

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
