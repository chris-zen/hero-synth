use std::collections::HashSet;

use hero_core::types::{SampleRate, DEFAULT_SAMPLE_RATE};
// use hero_core::processing::{AudioOutputBuffer, ProcessingArgs, Processor};

use patch::Patch;
use voice::Voice;

const MAX_KEYS: usize = 128;

pub struct Synth {
    sample_rate: SampleRate,
    patch: Patch,
    voices: Vec<Voice>,
    active_voices: HashSet<usize>
}

impl Default for Synth {
    fn default() -> Self {
        Synth {
            sample_rate: DEFAULT_SAMPLE_RATE,
            patch: Patch::default(),
            voices: Vec::new(),
            active_voices: HashSet::new()
        }
    }
}

impl Synth {
    pub fn new(sample_rate: SampleRate) -> Synth {
        let patch = Patch::default();
        let mut voices = Vec::<Voice>::with_capacity(MAX_KEYS);
        for key in 0..MAX_KEYS {
            let voice = Voice::new(sample_rate, &patch, key);
            voices.push(voice);
        }

        Synth {
            sample_rate: sample_rate,
            patch: patch,
            voices: voices,

            ..Synth::default()
        }
    }

    pub fn get_sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn note_on(&mut self, key: u8, vel: f64) {
        let voice_index = (key & 0x7f) as usize;
        let ref mut voice = self.voices[voice_index];
        voice.reset();
        // TODO update patch
        voice.note_on(vel);
        self.active_voices.insert(voice_index);
    }

    pub fn note_off(&mut self, key: u8, vel: f64) {
        let voice_index = (key & 0x7f) as usize;
        let ref mut voice = self.voices[voice_index];
        voice.note_off(vel);
        self.active_voices.remove(&voice_index);
    }

    pub fn process(&mut self) -> (f64, f64) {
        let mut left = 0f64;
        let mut right = 0f64;
        for voice_index in self.active_voices.iter() {
            let ref mut voice = self.voices[*voice_index];
            let (voice_left, voice_right) = voice.process();
            left += voice_left;
            right += voice_right;
        }
        // Not really sure of having to normalize according to the number of voices.
        // let num_voices = self.active_voices.len() as f64;
        // let left = left / num_voices;
        // let right = right / num_voices;
        (left, right)
    }
}

// impl<'a, O> Processor<'a, f32, O> for Synth
//     where O: AudioOutputBuffer<Output=f32> {
//
//     fn process(&mut self, args: ProcessingArgs<'a, f32, O>) {
//         let mut left_out = args.audio_out_left;
//         let mut right_out = args.audio_out_right;
//         for i in 0..args.num_frames {
//             let mut left = 0f64;
//             let mut right = 0f64;
//             for voice_index in self.active_voices.iter() {
//                 let ref mut voice = self.voices[*voice_index];
//                 let (voice_left, voice_right) = voice.process();
//                 left += voice_left;
//                 right += voice_right;
//             }
//             // Not really sure of having to normalize according to the number of voices.
//             let num_voices = self.active_voices.len() as f64;
//             left_out[i] = (left / num_voices) as f32;
//             right_out[i] = (right / num_voices) as f32;
//         }
//     }
// }
