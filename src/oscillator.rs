use ::std::f64::consts::PI;

use wavetable::Wavetable;

pub struct Oscillator<'a> {
    is_enabled: bool,

    wavetable: &'a Wavetable<'a>,
    freq_to_table_incr: f64,
    table_incr: f64,
    initial_phase: f64,
    table_offset: f64,

    //env_index: usize,

    //mix_amp: f64,
    //pan_left: f64,
    //pan_right: f64,

    amplitude: f64,         // Oscillator signal amplitude

    is_fixed_freq: bool,    // When it is true the baseFrequency doesn't change with noteOn
    base_frequency: f64,    // Oscillator base frequency
    octaves: i32,           // Number of octaves to shift from the base_frequency
    semitones: i32,         // Number of semitones to shift from the base_frequency
    detune: f64,            // Fine shift from the base_frequency
    frequency: f64,         // Calculated from base_frequency, octaves, semitones and detune

    //freq_mod: f64,          // Amount of modulation to the carriers
    phase_mod: f64,         // Phase modulation calculated from frequency and freq_mod
}

impl<'a> Oscillator<'a> {
    pub fn new(sample_rate: f64, wavetable: &'a Wavetable<'a>) -> Oscillator<'a> {
        let mut o = Oscillator {
            is_enabled: true,
            wavetable: wavetable,
            freq_to_table_incr: wavetable.size() as f64 / sample_rate,
            table_incr: 0.0,
            initial_phase: 0.0,
            table_offset: 0.0,
            //env_index: 0,
            //mix_amp: 1.0,
            //pan_left: 1.0,
            //pan_right: 1.0,
            amplitude: 1.0,
            is_fixed_freq: false,
            base_frequency: 440.0,
            octaves: 0,
            semitones: 0,
            detune: 0.0,
            frequency: 0.0,
            //freq_mod: 0.0,
            phase_mod: 0.0
        };
        o.reset_phase();
        o.update_frequency();
        o
    }

    fn reset_phase(&mut self) {
        self.table_offset = (self.initial_phase / (2.0 * PI)) * (self.wavetable.size() as f64);
    }

    //fn modulate_phase(&mut self, phase_incr: f64) {
    //    self.table_offset += phase_incr;
    //}

    fn update_frequency(&mut self) {
        let pitch_scale = (2.0f64).powf(((((self.octaves * 1200 + self.semitones) * 100) as f64) + self.detune) / 1200.0);
        self.frequency = self.base_frequency * pitch_scale;
        if self.frequency < 0.0 {
            self.frequency = 0.0;
        }
        self.table_incr = self.frequency * self.freq_to_table_incr;
        //TODO osc.phaseMod = osc.freqMod * osc.tableIncr
    }

    pub fn set_base_frequency(&mut self, freq: f64) {
        self.base_frequency = freq;
        self.update_frequency();
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
    }

    pub fn set_phase_modulation(&mut self, value: f64) {
        self.phase_mod = value * self.freq_to_table_incr;
    }

    pub fn process(&mut self) -> f64 {
        let wt_size = self.wavetable.size() as f64;
        if self.table_offset < 0.0 {
            self.table_offset += wt_size;
        } else if self.table_offset >= wt_size {
            self.table_offset -= wt_size;
        }

        let mut value = 0.0f64;
        if self.is_enabled && self.amplitude > 0.0 {
            value = self.amplitude * self.wavetable.value(self.table_offset);
        }

        /*
        let table_incr = if self.phase_mod < self.table_incr {
            self.table_incr + self.phase_mod
        }
        else if self.phase_mod > self.table_incr {
            self.table_incr * 2.0
        }
        else {
            0.0
        };*/

        self.table_offset += self.table_incr + self.phase_mod;

        value
    }
}

use wavetable;

#[test]
fn test_osc() {
    let o = Oscillator::new(44100, &wavetable::SIN);
    assert_eq!(o.amplitude, 1.0f64);
}
