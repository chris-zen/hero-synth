use std::f64::consts::PI;

use freq;
use wavetable::Wavetable;

#[derive(Clone, Debug)]
pub struct Oscillator {
    is_enabled: bool,

    wavetable: Wavetable,

    is_free_phase: bool,    // When true, the phase is not reset to the initial_phase, but continues from where it was
    initial_phase: f64,     // The initial phase for the wave in radians

    freq_to_table_incr: f64,
    table_incr: f64,
    table_offset: f64,

    amplitude: f64,         // Oscillator signal amplitude
    amp_mod: f64,           // Amplitude modulation

    base_frequency: f64,    // Oscillator base frequency
    octaves: f64,           // Number of octaves to shift from the base_frequency. Aka Ratio
    semitones: f64,         // Number of semitones to shift from the base_frequency. Aka Pitch
    detune: f64,            // Fine shift from the base_frequency

    frequency: f64,         // Calculated from base_frequency, octaves, semitones and detune
    phase_mod: f64,         // Phase modulation calculated from frequency and freq_mod
}

impl Default for Oscillator {
    fn default() -> Self {
        Oscillator {
            is_enabled: true,
            wavetable: Wavetable::default(),
            is_free_phase: false,
            initial_phase: 0.0,
            freq_to_table_incr: 0.0,
            table_incr: 0.0,
            table_offset: 0.0,
            amplitude: 1.0,
            amp_mod: 1.0,
            base_frequency: 440.0,
            octaves: 0.0,
            semitones: 0.0,
            detune: 0.0,
            frequency: 0.0,
            phase_mod: 0.0
        }
    }
}

impl Oscillator {
    pub fn new(sample_rate: f64, wavetable: Wavetable, freq: f64) -> Oscillator {
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

    pub fn from_sample_rate(sample_rate: f64) -> Oscillator {
        let mut o = Oscillator::default();
        let wt_size = o.wavetable.size() as f64;
        o.freq_to_table_incr = wt_size / sample_rate;
        o.init();
        o
    }

    pub fn from_wavetable(sample_rate: f64, wavetable: Wavetable) -> Oscillator {
        let wt_size = wavetable.size() as f64;
        let mut o = Oscillator {
            wavetable: wavetable,
            freq_to_table_incr: wt_size / sample_rate,
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
        if !self.is_free_phase {
            self.reset_phase();
        }
    }

    fn reset_phase(&mut self) {
        self.table_offset = (self.initial_phase / (2.0 * PI)) * (self.wavetable.size() as f64);
    }

    fn update_frequency(&mut self) {
        let pitch_scale = freq::pitch_scale(self.octaves, self.semitones, self.detune);
        self.frequency = self.base_frequency * pitch_scale;
        if self.frequency < 0.0 {
            self.frequency = 0.0;
        }
        self.table_incr = self.frequency * self.freq_to_table_incr;
    }

    pub fn set_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_free_phase(&mut self, is_free_phase: bool) {
        self.is_free_phase = is_free_phase;
    }

    pub fn is_free_phase(&self) -> bool {
        self.is_free_phase
    }

    pub fn set_initial_phase(&mut self, initial_phase: f64) {
        self.initial_phase = initial_phase;
        self.update_frequency();
    }

    pub fn get_initial_phase(&self) -> f64 {
        self.initial_phase
    }

    pub fn set_octaves(&mut self, octaves: f64) {
        self.octaves = octaves;
        self.update_frequency();
    }

    pub fn get_octaves(&self) -> f64 {
        self.octaves
    }

    pub fn set_semitones(&mut self, semitones: f64) {
        self.semitones = semitones;
        self.update_frequency();
    }

    pub fn get_semitones(&self) -> f64 {
        self.semitones
    }

    pub fn set_detune(&mut self, detune: f64) {
        self.detune = detune;
        self.update_frequency();
    }

    pub fn get_detune(&self) -> f64 {
        self.detune
    }

    pub fn set_amplitude(&mut self, value: f64) {
        self.amplitude = value;
    }

    pub fn set_amplitude_modulation(&mut self, value: f64) {
        self.amp_mod = value;
    }

    pub fn set_base_frequency(&mut self, freq: f64) {
        self.base_frequency = freq;
        self.update_frequency();
    }

    pub fn get_base_frequency(&self) -> f64 {
        self.base_frequency
    }

    pub fn set_freq_modulation(&mut self, value: f64) {
        self.phase_mod = value * self.freq_to_table_incr;
    }

    pub fn process(&mut self) -> f64 {
        let wt_size = self.wavetable.size() as f64;
        if self.table_offset < 0.0 {
            self.table_offset = wt_size - (self.table_offset.abs() % wt_size);
            // self.table_offset += wt_size;
        } else if self.table_offset >= wt_size {
            self.table_offset = self.table_offset % wt_size;
            // self.table_offset -= wt_size;
        }

        let mut value = 0.0f64;
        if self.is_enabled && self.amplitude > 0.0 {
            value = self.amplitude * self.amp_mod * self.wavetable.value(self.table_offset);
        }

        self.table_offset += self.table_incr + self.phase_mod;

        value
    }
}
