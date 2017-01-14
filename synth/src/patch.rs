
//use std::io;
//use std::fs::File;
//use std::path::Path;

use hero_core::wavetable::{self, Wavetable};
use hero_core::oscillator::Oscillator;
use hero_core::types::SampleRate;


pub struct SendLevel {
    pub index: usize,
    pub level: f64
}

pub struct OscPatch {
    pub is_enabled: bool,
    pub wavetable: String,
    pub initial_phase: f64,
    pub amplitude: f64,            // Oscillator signal amplitude
    pub is_fixed_freq: bool,       // When it is true the baseFrequency doesn't change with noteOn
    pub base_frequency: f64,       // Oscillator base frequency
    pub octaves: i32,              // Number of octaves to shift from the base_frequency
    pub semitones: i32,            // Number of semitones to shift from the base_frequency
    pub detune: f64,               // Fine shift from the base_frequency

    pub am_send: Vec<SendLevel>,   // Send levels for amplitude modulation
    pub fm_send: Vec<SendLevel>,   // Send levels for frequency modulation
    pub filt_send: Vec<SendLevel>, // Send levels for the filter input

    pub panning: f64,              // Panning [-1, +1]
    pub level: f64,                // Mix level
}

impl Default for OscPatch {
    fn default() -> Self {
        OscPatch {
            is_enabled: true,
            wavetable: "sin".to_string(),
            initial_phase: 0.0,
            amplitude: 1.0,
            is_fixed_freq: false,
            base_frequency: 440.0,
            octaves: 0,
            semitones: 0,
            detune: 0.0,

            am_send: Vec::new(),
            fm_send: Vec::new(),
            filt_send: Vec::new(),

            panning: 0.0,
            level: 1.0
        }
    }
}

impl OscPatch {
    pub fn into_oscillator(&self, sample_rate: SampleRate) -> Oscillator {
        let wt_stock = match wavetable::Stock::from_name(&self.wavetable) {
            Some(stock) => stock,
            None => wavetable::Stock::Sin,
        };
        let wavetable = Wavetable::from_stock(wt_stock);

        let mut o = Oscillator::new(sample_rate, wavetable, self.base_frequency);
        o.set_enabled(self.is_enabled);
        o.set_amplitude(self.amplitude);
        o.set_fixed_freq(self.is_fixed_freq);
        o.set_octaves(self.octaves);
        o.set_semitones(self.semitones);
        o.set_detune(self.detune);
        o
    }
}

pub struct FilterPatch {
    // TODO filter params
    pub mode: String,
    pub slope: String,
    pub freq: f64,
    pub res: f64,

    pub am_send: Vec<SendLevel>,   // Send levels for amplitude modulation
    pub fm_send: Vec<SendLevel>,   // Send levels for frequency modulation
    pub filt_send: Vec<SendLevel>, // Send levels for the filter input

    pub panning: f64,              // Panning [-1, +1]
    pub level: f64,                // Mix level
}

pub struct Patch {
    pub oscillators: Vec<OscPatch>,
    pub filters: Vec<FilterPatch>,
}

impl Default for Patch {
    fn default() -> Self {
        let osc1 = OscPatch::default();

        Patch {
            oscillators: vec![osc1],
            filters: Vec::new()
        }
    }
}

/*
pub fn load(path: &Path) -> io::Result<Patch> {
    let mut file = try!(File::open(path));
    let mut data = String::new();
    try!(file.read_to_string(&mut data))
    //try!(Json::from_str(&data))
}
*/
