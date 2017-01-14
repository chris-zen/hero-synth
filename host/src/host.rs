use hero_core::oscillator::Oscillator;
use hero_core::wavetable::{self, Wavetable};
use hero_core::types::{SampleRate, Tempo};
use hero_core::processing::{AudioOutputBuffer, ProcessingArgs, Processor};

#[derive(Clone)]
pub struct AlienSynth {
    oc1: Oscillator,
    om1: Oscillator,
    om2: Oscillator
}

impl AlienSynth {
    pub fn new(sample_rate: SampleRate) -> AlienSynth {
        let wt1 = Wavetable::from_stock(wavetable::Stock::Sin);
        let wt2 = Wavetable::from_stock(wavetable::Stock::Sin);
        let wt3 = Wavetable::from_stock(wavetable::Stock::Sin);

        let fc = 440.0f64;
        let fm1 = 0.5f64;
        let fm2 = 2.0f64;
        let mi = 16.0;

        let mut oc1 = Oscillator::new(sample_rate, wt1, fc);
        oc1.set_amplitude(1.0);

        let mut om1 = Oscillator::new(sample_rate, wt2, fm1);
        om1.set_amplitude(mi * fm1);

        let mut om2 = Oscillator::new(sample_rate, wt3, fm2);
        om2.set_amplitude(mi * fm2);

        AlienSynth { oc1: oc1, om1: om1, om2: om2 }
    }

    pub fn process(&mut self) -> f64 {
        let ms1 = self.om1.process();
        self.om2.set_freq_modulation(ms1);
        let ms2 = self.om2.process();
        self.oc1.set_freq_modulation(ms2);
        let cs = self.oc1.process();
        cs
    }
}

#[derive(Clone)]
pub struct Host {
    sample_rate: f64,
    tempo: f64,
    alien_synth: AlienSynth
}

const DEFAULT_TEMPO: f64 = 120 as Tempo;

unsafe impl Send for Host {}

impl Host {
    pub fn new(sample_rate: SampleRate) -> Result<Host, String> {
        Ok(Host {
            sample_rate: sample_rate,
            tempo: DEFAULT_TEMPO,
            alien_synth: AlienSynth::new(sample_rate)
        })
    }

    // pub fn sample_rate(&self) -> SampleRate {
    //     self.sample_rate
    // }
}

impl<'a, O> Processor<'a, f32, O> for Host where O: AudioOutputBuffer<f32> {
    fn process(&mut self, args: ProcessingArgs<'a, f32, O>) {
        let mut left = args.audio_left;
        let mut right = args.audio_right;
        for i in 0..args.num_frames {
            let cs = self.alien_synth.process();
            left[i] = cs as f32;
            right[i] = cs as f32;
        }
    }
}
