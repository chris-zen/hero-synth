extern crate portaudio;
extern crate portmidi;
// extern crate coremidi;

extern crate hero_core;
extern crate hero_synth;

mod host;
mod buffers;
mod audio;
mod midi;

use std::sync::{Arc, Mutex};

use audio::{SAMPLE_RATE, audio_start, audio_close};
use midi::midi_start;

fn main() {

    let host = Arc::new(Mutex::new(host::Host::new(SAMPLE_RATE).unwrap()));

    let pa_ctx = portaudio::PortAudio::new().unwrap();

    let pm_ctx = portmidi::PortMidi::new().unwrap();

    let mut stream = audio_start(&pa_ctx, host.clone()).unwrap();

    midi_start(&pm_ctx, host.clone()).unwrap();

    // Loop while the non-blocking stream is active.
    while let Ok(true) = stream.is_active() {
        pa_ctx.sleep(1000);
    }

    audio_close(&mut stream).unwrap();

    println!("");
}
