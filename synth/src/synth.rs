use hero_core::types::SampleRate;
use hero_core::processing::{AudioOutputBuffer, ProcessingArgs, Processor};

use patch::Patch;


pub struct Synth {
    sample_rate: SampleRate,
    patch: Patch
}

impl Default for Synth {
    fn default() -> Self {
        Synth {
            sample_rate: 44100 as SampleRate,
            patch: Patch::default()
        }
    }
}

impl Synth {
    pub fn new(sample_rate: SampleRate) -> Synth {
        Synth {
            sample_rate: sample_rate,
            ..Synth::default()
        }
    }

    pub fn get_sample_rate(&self) -> SampleRate {
        self.sample_rate
    }
}

impl<'a, O> Processor<'a, f32, O> for Synth where O: AudioOutputBuffer<f32> {
    fn process(&mut self, args: ProcessingArgs<'a, f32, O>) {

        let mut d = args.audio_left;
        d[1] + 1.0;
        d[2] = 1.2;
    }
}
