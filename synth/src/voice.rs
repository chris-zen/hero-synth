use std::rc::Rc;
use std::cell::RefCell;

use hero_core::types::SampleRate;
use hero_core::freq::KEY_FREQ;
use hero_core::wavetable::{self, Wavetable};
use hero_core::oscillator::Oscillator;
use hero_core::panning::Panning;
use hero_core::filter::Filter;
use hero_core::filter::iir::IIR;

use patch::Patch;

pub const MAX_OSCILLATORS: usize = 8;
pub const MAX_FILTERS: usize = 2;

/// Modulation index for Frequency Modulation
const MOD_INDEX: f64 = 6.0;

#[derive(Debug)]
struct VoiceOsc {
    oscillator: Oscillator,
    panning: Panning,
}

#[derive(Debug)]
struct VoiceFilter {
    iir: IIR,
    panning: Panning
}

#[derive(Debug)]
pub struct Voice {
    patch: Rc<RefCell<Patch>>,
    patch_version: usize,
    oscillators: Vec<VoiceOsc>,
    filters: Vec<VoiceFilter>,
    key: usize,
    velocity: f64,
}

/// This synth has one voice per allowed key, so every voice has a fixed freq.
impl Voice {
    pub fn new(sample_rate: SampleRate, patch: Rc<RefCell<Patch>>) -> Voice {
        let mut oscillators = Vec::<VoiceOsc>::with_capacity(MAX_OSCILLATORS);
        for patch_osc in patch.borrow().oscillators.iter().take(MAX_OSCILLATORS) {
            let osc = patch_osc.to_oscillator(sample_rate);
            let voice_osc = VoiceOsc {
                oscillator: osc,
                panning: Panning::new(patch_osc.panning),
            };
            oscillators.push(voice_osc);
        }
        while oscillators.len() < MAX_OSCILLATORS {
            let wt = Wavetable::from_stock(wavetable::Stock::Sin);
            let osc = Oscillator::new(sample_rate, wt, 0.0);
            let voice_osc = VoiceOsc {
                oscillator: osc,
                panning: Panning::new(0.0),
            };
            oscillators.push(voice_osc);
        }

        let mut filters = Vec::<VoiceFilter>::with_capacity(MAX_FILTERS);
        for filt_patch in patch.borrow().filters.iter().take(MAX_FILTERS) {
            // TODO filter from patch
            let iir = IIR::bypass(sample_rate);
            let pan = Panning::new(filt_patch.panning);
            let voice_filter = VoiceFilter {
                iir: iir,
                panning: pan
            };
            filters.push(voice_filter);
        }

        Voice {
            patch: patch,
            patch_version: 0,
            oscillators: oscillators,
            filters: filters,
            key: 0,
            velocity: 0.0
        }
    }

    pub fn patch_version(&self) -> usize {
        self.patch_version
    }

    pub fn update_patch(&mut self, patch: &Patch, patch_version: usize) {
        self.patch_version = patch_version;
        for index in 0..patch.oscillators.len() {
            let patch_osc = &patch.oscillators[index];

            let voice_osc = &mut self.oscillators[index];
            voice_osc.panning.set_value(patch_osc.panning);

            let osc = &mut voice_osc.oscillator;
            osc.set_enabled(patch_osc.is_enabled);
            osc.set_amplitude(patch_osc.amplitude);
            // TODO wavetable
            osc.set_free_phase(patch_osc.is_free_phase);
            osc.set_initial_phase(patch_osc.initial_phase);
            osc.set_octaves(patch_osc.octaves);
            osc.set_semitones(patch_osc.semitones);
            osc.set_detune(patch_osc.detune);
        }
        let remaining_osc = patch.oscillators.len() .. MAX_OSCILLATORS;
        for voice_osc in self.oscillators[remaining_osc].iter_mut() {
            voice_osc.oscillator.set_enabled(false);
            voice_osc.oscillator.set_amplitude(0.0);
        }
    }

    pub fn reset(&mut self) {
        // Oscillators
        for voice_osc in self.oscillators.iter_mut() {
            voice_osc.oscillator.reset();
        }

        // Filters
        for voice_filter in self.filters.iter_mut() {
            voice_filter.iir.reset();
        }
    }

    pub fn note_on(&mut self, key: usize, vel: f64) {
        if self.key != key {
            let freq = KEY_FREQ[key & 0x7f];
            let patch = self.patch.borrow();
            for (index, patch_osc) in patch.oscillators.iter().enumerate() {
                if !patch_osc.is_fixed_freq {
                    let mut voice_osc = &mut self.oscillators[index];
                    voice_osc.oscillator.set_base_frequency(freq);
                }
            }
        }
        self.velocity = vel;
    }

    pub fn note_off(&mut self, _key: usize, _vel: f64) {
        self.velocity = 0.0;
        // TODO envelopes !!!
    }

    pub fn process(&mut self) -> (f64, f64) {
        let mut osc_signals = [0.0f64; MAX_OSCILLATORS];
        let mut osc_amp_mod = [1.0f64; MAX_OSCILLATORS];
        let mut osc_freq_mod = [0.0f64; MAX_OSCILLATORS];

        let num_osc = self.oscillators.len();

        let patch = &self.patch.borrow();

        // Calculate oscillators' signals and send AM and FM modulation

        for i in 0..patch.oscillators.len() {
            let voice_osc = &mut self.oscillators[i];
            let ref mut osc = voice_osc.oscillator;
            let sig = osc.process();
            osc_signals[i] = sig;

            if i < patch.oscillators.len() {
                let patch_osc = &patch.oscillators[i];
                for (index, level) in patch_osc.amp_mod.iter() {
                    osc_amp_mod[index.clone()] += sig * level;
                }

                let fm = sig * MOD_INDEX * osc.get_base_frequency();
                for (index, level) in patch_osc.freq_mod.iter() {
                    osc_freq_mod[index.clone()] += fm * level;
                }
            }
        }

        let mut left = 0.0f64;
        let mut right = 0.0f64;

        // Update oscillators' modulation, and apply panning and mix their signals

        for i in 0..num_osc {
            let voice_osc = &mut self.oscillators[i];
            let ref mut osc = voice_osc.oscillator;
            osc.set_amplitude_modulation(osc_amp_mod[i]);
            osc.set_freq_modulation(osc_freq_mod[i]);

            if i < patch.oscillators.len() {
                let patch_osc = &patch.oscillators[i];
                if patch_osc.level > 0.0 {
                    let (osc_left, osc_right) = voice_osc.panning.process(osc_signals[i]);
                    left += osc_left * patch_osc.level;
                    right += osc_right * patch_osc.level;
                }
            }
        }

        // Normalize output

        let inv_count = 1.0 / num_osc as f64;
        let left = left * inv_count * self.velocity;
        let right = right * inv_count * self.velocity;
        (left, right)
    }
}
