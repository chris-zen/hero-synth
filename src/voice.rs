use std::rc::Rc;

use oscillator::Oscillator;
use panning::Panning;
use filter::Filter;
use filter::iir::IIR;
use patch::Patch;

const MAX_OSCILLATORS: usize = 8;
const MAX_FILTERS: usize = 2;

/// Modulation index for Frequency Modulation
const MOD_INDEX: f64 = 6.0;

struct Voice<'a> {
    patch: Rc<Patch>,
    sample_rate: f64,
    oscillators: Vec<Oscillator<'a>>,
    osc_panning: Vec<Panning<'a>>,
    filters: Vec<Box<Filter>>,
    filt_panning: Vec<Panning<'a>>,
    freq: f64,
    amplitude: f64,
}

impl<'a> Voice<'a> {
    pub fn new(sample_rate: f64, patch: Rc<Patch>) -> Voice<'a> {
        let mut oscillators = Vec::with_capacity(MAX_OSCILLATORS);
        let mut osc_panning = Vec::with_capacity(MAX_OSCILLATORS);
        for osc_def in patch.oscillators.iter().take(MAX_OSCILLATORS) {
            let osc = Oscillator::from_patch_def(sample_rate, osc_def);
            oscillators.push(osc);
            let pan = Panning::new(osc_def.panning);
            osc_panning.push(pan);
        }
        /*for _i in patch.oscillators.len() .. MAX_OSCILLATORS {
            let osc = Oscillator::from_sample_rate(sample_rate);
            oscillators.push(osc);
        }*/

        let mut filters: Vec<Box<Filter>> = Vec::with_capacity(MAX_FILTERS);
        let mut filt_panning = Vec::with_capacity(MAX_FILTERS);
        for filt_def in patch.filters.iter().take(MAX_FILTERS) {
            // TODO filter from patch
            let filt = Box::new(IIR::bypass(sample_rate));
            filters.push(filt);
            let pan = Panning::new(filt_def.panning);
            filt_panning.push(pan);
        }

        Voice {
            patch: patch.clone(),
            sample_rate: sample_rate,
            oscillators: oscillators,
            osc_panning: osc_panning,
            filters: filters,
            filt_panning: filt_panning,
            freq: 440.0,
            amplitude: 1.0,
        }
    }

    fn reset(&mut self) {
        // Oscillators
        for osc in self.oscillators.iter_mut() {
            osc.reset();
        }

        // Filters
        for filt in self.filters.iter_mut() {
            filt.reset();
        }
    }

    pub fn set_patch(&mut self, patch: Rc<Patch>) {
        self.patch = patch.clone();

        unimplemented!();

        self.reset();
    }

    pub fn process(&mut self) -> (f64, f64) {
        let mut osc_signals = [0.0; MAX_OSCILLATORS];
        let mut osc_amp_mod = [0.0; MAX_OSCILLATORS];
        let mut osc_freq_mod = [0.0; MAX_OSCILLATORS];

        let num_osc = self.oscillators.len();

        // Calculate oscillators' signals and send modulation

        for i in 0..num_osc {
            let osc = &mut self.oscillators[i];
            let sig = osc.process();
            osc_signals[i] = sig;

            let patch_osc = &self.patch.oscillators[i];

            for am_send in patch_osc.am_send.iter() {
                osc_amp_mod[am_send.index] += sig * am_send.level;
            }

            let fm = sig * MOD_INDEX * osc.get_base_frequency();
            for fm_send in patch_osc.fm_send.iter() {
                osc_freq_mod[fm_send.index] += fm * fm_send.level;
            }
        }

        let mut left = 0.0f64;
        let mut right = 0.0f64;
        let mut count: usize = 0;

        // Update oscillators' modulation, and apply panning and mix their signals

        for i in 0..num_osc {
            let osc = &mut self.oscillators[i];
            osc.set_amplitude_modulation(osc_amp_mod[i]);
            osc.set_freq_modulation(osc_freq_mod[i]);

            let patch_osc = &self.patch.oscillators[i];

            let (osc_left, osc_right) = self.osc_panning[i].process(osc_signals[i]);
            let osc_level = patch_osc.level;
            left += osc_left * osc_level;
            right += osc_right * osc_level;
            count += 1;
        }

        // Normalize output

        let count = 1.0 / count as f64;

        (left * count, right * count)
    }
}
