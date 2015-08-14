use std::f64::consts::PI;

use wavetable::{self, Wavetable};
use patch;

pub struct Oscillator<'a> {
    is_enabled: bool,

    wavetable: Wavetable<'a>,
    freq_to_table_incr: f64,
    table_incr: f64,
    initial_phase: f64,
    table_offset: f64,

    amplitude: f64,         // Oscillator signal amplitude
    amp_mod: f64,           // Amplitude modulation

    is_fixed_freq: bool,    // When it is true the baseFrequency doesn't change with noteOn
    base_frequency: f64,    // Oscillator base frequency
    octaves: i32,           // Number of octaves to shift from the base_frequency
    semitones: i32,         // Number of semitones to shift from the base_frequency
    detune: f64,            // Fine shift from the base_frequency
    frequency: f64,         // Calculated from base_frequency, octaves, semitones and detune
    phase_mod: f64,         // Phase modulation calculated from frequency and freq_mod
}

impl<'a> Default for Oscillator<'a> {
    fn default() -> Self {
        Oscillator {
            is_enabled: true,
            wavetable: Wavetable::default(),
            freq_to_table_incr: 0.0,
            table_incr: 0.0,
            initial_phase: 0.0,
            table_offset: 0.0,
            amplitude: 1.0,
            amp_mod: 1.0,
            is_fixed_freq: false,
            base_frequency: 440.0,
            octaves: 0,
            semitones: 0,
            detune: 0.0,
            frequency: 0.0,
            phase_mod: 0.0
        }
    }
}

impl<'a> Oscillator<'a> {
    pub fn new(sample_rate: f64, wavetable: Wavetable<'a>, freq: f64) -> Oscillator<'a> {
        let wt_size = wavetable.size() as f64;
        let mut o = Oscillator {
            wavetable: wavetable,
            freq_to_table_incr: wt_size / sample_rate,
            base_frequency: freq,
            ..Default::default()
        };
        o.init();
        o
    }

    pub fn from_sample_rate(sample_rate: f64) -> Oscillator<'a> {
        let mut o = Oscillator::default();
        let wt_size = o.wavetable.size() as f64;
        o.freq_to_table_incr = wt_size / sample_rate;
        o.init();
        o
    }

    pub fn from_wavetable(sample_rate: f64, wavetable: Wavetable<'a>) -> Oscillator<'a> {
        let wt_size = wavetable.size() as f64;
        let mut o = Oscillator {
            wavetable: wavetable,
            freq_to_table_incr: wt_size / sample_rate,
            ..Default::default()
        };
        o.init();
        o
    }

    pub fn from_patch_def(sample_rate: f64, osc_desc: &patch::Osc) -> Oscillator<'a> {
        let wt_stock = match wavetable::Stock::from_name(&osc_desc.wavetable) {
            Some(stock) => stock,
            None => wavetable::Stock::Sin,
        };
        let wavetable = Wavetable::from_stock(wt_stock);
        let wt_size = wavetable.size() as f64;
        let mut o = Oscillator {
            is_enabled: osc_desc.is_enabled,
            wavetable: wavetable,
            freq_to_table_incr: wt_size / sample_rate,
            amplitude: osc_desc.amplitude,
            is_fixed_freq: osc_desc.is_fixed_freq,
            base_frequency: osc_desc.base_frequency,
            octaves: osc_desc.octaves,
            semitones: osc_desc.semitones,
            detune: osc_desc.detune,
            ..Default::default()
        };
        o.init();
        o
    }

    fn init(&mut self) {
        self.reset_phase();
        self.update_frequency();
    }

    pub fn reset(&mut self) {
        self.reset_phase();
    }

    fn reset_phase(&mut self) {
        self.table_offset = (self.initial_phase / (2.0 * PI)) * (self.wavetable.size() as f64);
    }

    fn update_frequency(&mut self) {
        let pitch_scale = (2.0f64).powf(((((self.octaves * 1200 + self.semitones) * 100) as f64) + self.detune) / 1200.0);
        self.frequency = self.base_frequency * pitch_scale;
        if self.frequency < 0.0 {
            self.frequency = 0.0;
        }
        self.table_incr = self.frequency * self.freq_to_table_incr;
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
    }

    pub fn set_amplitude_modulation(&mut self, value: f64) {
        self.amp_mod = value;
    }

    pub fn get_base_frequency(&self) -> f64 {
        self.base_frequency
    }
    
    pub fn set_base_frequency(&mut self, freq: f64) {
        self.base_frequency = freq;
        self.update_frequency();
    }

    pub fn set_freq_modulation(&mut self, value: f64) {
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
            value = self.amplitude * self.amp_mod * self.wavetable.value(self.table_offset);
        }

        self.table_offset += self.table_incr + self.phase_mod;

        value
    }
}
