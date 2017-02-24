use std::collections::HashMap;

use hero_core::wavetable::{self, Wavetable};
use hero_core::oscillator::Oscillator;
use hero_core::types::SampleRate;


#[derive(Clone, Debug)]
pub struct OscPatch {
    pub is_enabled: bool,
    pub amplitude: f64,            // Oscillator signal amplitude
    pub wavetable: String,
    pub is_free_phase: bool,
    pub initial_phase: f64,
    pub is_fixed_freq: bool,       // When it is true the baseFrequency doesn't change with noteOn
    pub base_frequency: f64,       // Oscillator base frequency
    pub octaves: f64,              // Number of octaves to shift from the base_frequency
    pub semitones: f64,            // Number of semitones to shift from the base_frequency
    pub detune: f64,               // Fine shift from the base_frequency

    pub amp_mod: HashMap<usize, f64>,   // Send levels for amplitude modulation
    pub freq_mod: HashMap<usize, f64>,   // Send levels for frequency modulation
    pub filt_send: HashMap<usize, f64>, // Send levels for the filter input

    pub level: f64,                // Mix level
    pub panning: f64,              // Panning [-1, +1]
}

impl Default for OscPatch {
    fn default() -> Self {
        OscPatch {
            is_enabled: true,
            amplitude: 1.0,
            wavetable: "sin".to_string(),
            is_free_phase: false,
            initial_phase: 0.0,
            is_fixed_freq: false,
            base_frequency: 440.0,
            octaves: 0.0,
            semitones: 0.0,
            detune: 0.0,

            amp_mod: HashMap::new(),
            freq_mod: HashMap::new(),
            filt_send: HashMap::new(),

            level: 1.0,
            panning: 0.0,
        }
    }
}

impl OscPatch {
    pub fn get_wavetable(&self) -> Wavetable {
        let wt_stock = match wavetable::Stock::from_name(&self.wavetable) {
            Some(stock) => stock,
            None => wavetable::Stock::Sin,
        };
        Wavetable::from_stock(wt_stock)
    }

    pub fn to_oscillator(&self, sample_rate: SampleRate) -> Oscillator {
        let wavetable = self.get_wavetable();
        let mut o = Oscillator::new(sample_rate, wavetable, self.base_frequency);
        o.set_enabled(self.is_enabled);
        o.set_amplitude(self.amplitude);
        o.set_free_phase(self.is_free_phase);
        o.set_initial_phase(self.initial_phase);
        o.set_octaves(self.octaves);
        o.set_semitones(self.semitones);
        o.set_detune(self.detune);
        o
    }
}

#[derive(Clone, Debug)]
pub struct FilterPatch {
    // TODO filter params
    pub mode: String,
    pub slope: String,
    pub freq: f64,
    pub res: f64,

    pub amp_mod: HashMap<usize, f64>,   // Send levels for amplitude modulation
    pub freq_mod: HashMap<usize, f64>,   // Send levels for frequency modulation
    pub filt_send: HashMap<usize, f64>, // Send levels for the filter input

    pub panning: f64,              // Panning [-1, +1]
    pub level: f64,                // Mix level
}

#[derive(Clone, Debug)]
pub struct Patch {
    pub oscillators: Vec<OscPatch>,
    pub filters: Vec<FilterPatch>,
}

impl Default for Patch {
    fn default() -> Self {
        const O1: usize = 0;
        const O2: usize = 0;
        const O3: usize = 0;
        const O4: usize = 0;
        let mut o1 = OscPatch::default();
        let mut o2 = OscPatch::default();
        let mut o3 = OscPatch::default();
        let mut o4 = OscPatch::default();
        o1.level = 1.0;
        // o1.is_fixed_freq = true;
        // o1.base_frequency = 1.0 / 4.0;
        // o1.detune = 0.75;
        // o1.amplitude = 16.0 * o1.base_frequency;

        o2.level = 0.0;
        // o2.is_fixed_freq = true;
        // o2.base_frequency = 2.0;
        // o2.detune = 0.0;
        // o2.amplitude = 16.0 * o2.base_frequency;

        o3.level = 0.0;
        o3.is_fixed_freq = true;
        o3.base_frequency = 4.0;
        // o3.octaves = 4.0;
        // o3.detune = 0.16;
        // o3.amplitude = 16.0 * o2.base_frequency;

        o4.level = -1.0;
        // o3.is_fixed_freq = true;
        // o3.base_frequency = 2.0;
        o4.semitones = 8.0;
        // o3.detune = 0.16;
        // o3.amplitude = 16.0 * o2.base_frequency;

        o3.freq_mod.insert(O1, 0.70);

        Patch {
            oscillators: vec![o1, o2, o3, o4],
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
