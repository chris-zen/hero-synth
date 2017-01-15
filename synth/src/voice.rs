use hero_core::oscillator::Oscillator;
use hero_core::panning::Panning;
use hero_core::filter::Filter;
use hero_core::filter::iir::IIR;

use patch::{Patch, SendLevel};

const MAX_OSCILLATORS: usize = 8;
const MAX_FILTERS: usize = 2;

/// Modulation index for Frequency Modulation
const MOD_INDEX: f64 = 6.0;

static KEY_FREQ: [f64; 128] = [
   8.176,      8.662,      9.177,      9.723,     10.301,     10.913,     11.562,     12.250,     12.978,     13.750,     14.568,     15.434,
  16.352,     17.324,     18.354,     19.445,     20.602,     21.827,     23.125,     24.500,     25.957,     27.500,     29.135,     30.868,
  32.703,     34.648,     36.708,     38.891,     41.203,     43.654,     46.249,     48.999,     51.913,     55.000,     58.270,     61.735,
  65.406,     69.296,     73.416,     77.782,     82.407,     87.307,     92.499,     97.999,    103.826,    110.000,    116.541,    123.471,
 130.813,    138.591,    146.832,    155.563,    164.814,    174.614,    184.997,    195.998,    207.652,    220.000,    233.082,    246.942,
 261.626,    277.183,    293.665,    311.127,    329.628,    349.228,    369.994,    391.995,    415.305,    440.000,    466.164,    493.883,
 523.251,    554.365,    587.330,    622.254,    659.255,    698.456,    739.989,    783.991,    830.609,    880.000,    932.328,    987.767,
1046.502,   1108.731,   1174.659,   1244.508,   1318.510,   1396.913,   1479.978,   1567.982,   1661.219,   1760.000,   1864.655,   1975.533,
2093.005,   2217.461,   2349.318,   2489.016,   2637.020,   2793.826,   2959.955,   3135.963,   3322.438,   3520.000,   3729.310,   3951.066,
4186.009,   4434.922,   4698.636,   4978.032,   5274.041,   5587.652,   5919.911,   6271.927,   6644.875,   7040.000,   7458.620,   7902.133,
8372.018,   8869.844,   9397.273,   9956.063,  10548.082,  11175.303,  11839.822,  12543.854];

#[derive(Debug)]
struct VoiceOsc {
    oscillator: Oscillator,
    level: f64,
    panning: Panning,
    am_send: Vec<SendLevel>,
    fm_send: Vec<SendLevel>,
    // filt_send: Vec<SendLevel>
}

#[derive(Debug)]
struct VoiceFilter {
    iir: IIR,
    panning: Panning
}

#[derive(Debug)]
pub struct Voice {
    oscillators: Vec<VoiceOsc>,
    filters: Vec<VoiceFilter>,
    amplitude: f64,
    freq: f64
}

/// This synth has one voice per allowed key, so every voice has a fixed freq.
impl Voice {
    pub fn new(sample_rate: f64, patch: &Patch, key: usize) -> Voice {
        let freq = KEY_FREQ[key & 0x7f];
        let mut oscillators = Vec::<VoiceOsc>::with_capacity(MAX_OSCILLATORS);
        for osc_patch in patch.oscillators.iter().take(MAX_OSCILLATORS) {
            let mut osc = osc_patch.to_oscillator(sample_rate);
            if !osc.is_fixed_freq() {
                osc.set_base_frequency(freq);
            }
            let pan = Panning::new(osc_patch.panning);
            let voice_osc = VoiceOsc {
                oscillator: osc,
                level: osc_patch.level,
                panning: pan,
                am_send: osc_patch.am_send.clone(),
                fm_send: osc_patch.fm_send.clone(),
                // filt_send: osc_patch.filt_send.clone(),
            };
            oscillators.push(voice_osc);
        }

        let mut filters = Vec::<VoiceFilter>::with_capacity(MAX_FILTERS);
        for filt_patch in patch.filters.iter().take(MAX_FILTERS) {
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
            oscillators: oscillators,
            filters: filters,
            amplitude: 1.0,
            freq: freq
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

    pub fn note_on(&mut self, vel: f64) {
        self.amplitude = vel;
    }

    pub fn process(&mut self) -> (f64, f64) {
        let mut osc_signals = [0.0f64; MAX_OSCILLATORS];
        let mut osc_amp_mod = [1.0f64; MAX_OSCILLATORS];
        let mut osc_freq_mod = [0.0f64; MAX_OSCILLATORS];

        let num_osc = self.oscillators.len();

        // Calculate oscillators' signals and send AM and FM modulation

        for i in 0..num_osc {
            let voice_osc = &mut self.oscillators[i];
            let ref mut osc = voice_osc.oscillator;
            let sig = osc.process();
            osc_signals[i] = sig;

            for am_send in voice_osc.am_send.iter() {
                osc_amp_mod[am_send.index] += sig * am_send.level;
            }

            let fm = sig * MOD_INDEX * osc.get_base_frequency();
            for fm_send in voice_osc.fm_send.iter() {
                osc_freq_mod[fm_send.index] += fm * fm_send.level;
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

            if voice_osc.level > 0.0 {
                let (osc_left, osc_right) = voice_osc.panning.process(osc_signals[i]);
                left += osc_left * voice_osc.level;
                right += osc_right * voice_osc.level;
            }
        }

        // Normalize output

        let inv_count = 1.0 / num_osc as f64;
        let left = left * inv_count * self.amplitude;
        let right = right * inv_count * self.amplitude;
        (left, right)
    }
}
