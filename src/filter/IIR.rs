//!
//! Infinite Impulse Response (IIR) filters
//!
//! Based on https://code.google.com/p/amsynth/source/browse/src/VoiceBoard/LowPassFilter.cc
//!

use std::f64::consts::PI;
use std::fmt::Display;
use std::fmt;
use filter::{Design, Slope, Filter};

const CUTOFF_DELTA: f64 = 0.01;
const CUTOFF_MIN: f64 = 10.0;

fn limit_cutoff(cutoff: f64, sample_rate: f64) -> f64 {
    cutoff.max(CUTOFF_MIN).min((sample_rate - 1.0) / 2.0)
}

#[derive(Debug)]
pub struct Coeffs {
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64
}

impl Display for Coeffs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a0={}, a1={}, a2={}, b1={}, b2={}",
                    self.a0, self.a1, self.a2, self.b1, self.b2)
    }
}

impl Default for Coeffs {
     fn default() -> Self {
         Coeffs {
             a0: 0.0,
             a1: 0.0,
             a2: 0.0,
             b1: 0.0,
             b2: 0.0
         }
     }
}

impl Coeffs {
    pub fn new(a0: f64, a1: f64, a2: f64, b1: f64, b2: f64) -> Coeffs {
        Coeffs {
            a0: a0,
            a1: a1,
            a2: a2,
            b1: b1,
            b2: b2
        }
    }

    fn common(sample_rate: f64, cutoff: f64, res: f64) -> (f64, f64, f64) {
        let w: f64 = cutoff / sample_rate; // cutoff freq [ 0 <= w <= 0.5 ]
        let r: f64 = (0.001f64).max(2.0 * (1.0 - res)); // r is 1/Q (sqrt(2) for a butterworth response)

        let k = (w * PI).tan();
        let k2 = k * k;
        let rk = r * k;
        let bh = 1.0 + rk + k2;

        (k2, rk, bh)
    }

    fn lowpass(sample_rate: f64, cutoff: f64, res: f64) -> Coeffs {

        let (k2, rk, bh) = Self::common(sample_rate, cutoff, res);

        let a0: f64 = k2 / bh;

        Coeffs {
            a0: a0,
            a1: a0 * 2.0,
            a2: a0,
            b1: (2.0 * (k2 - 1.0)) / bh,
            b2: (1.0 - rk + k2) / bh,
        }
    }

    fn highpass(sample_rate: f64, cutoff: f64, res: f64) -> Coeffs {

        let (k2, rk, bh) = Self::common(sample_rate, cutoff, res);

        let a0: f64 = 1.0 / bh;

        Coeffs {
            a0:  a0,
            a1: -2.0 / bh,
            a2:  a0,
            b1: (2.0 * (k2 - 1.0)) / bh,
            b2: (1.0 - rk + k2) / bh,
        }
    }

    fn bandpass(sample_rate: f64, cutoff: f64, res: f64) -> Coeffs {

        let (k2, rk, bh) = Self::common(sample_rate, cutoff, res);

        Coeffs {
            a0:  rk / bh,
            a1:  0.0,
            a2: -rk / bh,
            b1: (2.0 * (k2 - 1.0)) / bh,
            b2: (1.0 - rk + k2) / bh,
        }
    }

    fn bandstop(sample_rate: f64, cutoff: f64, res: f64) -> Coeffs {

        let (k2, rk, bh) = Self::common(sample_rate, cutoff, res);

        let a0: f64 = (1.0 + k2) / bh;
        let a1: f64 = (2.0 * (k2 - 1.0)) / bh;

        Coeffs {
            a0:  a0,
            a1:  a1,
            a2:  a0,
            b1:  a1,
            b2: (1.0 - rk + k2) / bh,
        }
    }
}

pub struct IIR {
    design: Design,
    slope: Slope,
    sample_rate: f64,
    cutoff: f64,
    res: f64,
    enabled: bool,
    pub coeff: Coeffs,
    d1: f64, d2: f64, d3: f64, d4: f64,
    invalid_coeffs: bool,
    invalid_delays: bool,
}

impl IIR {
    pub fn new(design: Design, slope: Slope, sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        assert!(sample_rate > 0.0);
        assert!(cutoff >= 0.0);
        assert!(res >= 0.0);

        let mut f = IIR {
            design: design,
            slope: slope,
            sample_rate: sample_rate,
            cutoff: limit_cutoff(cutoff, sample_rate),
            res: res,
            enabled: true,
            coeff: Coeffs::default(),
            d1: 0.0, d2: 0.0, d3: 0.0, d4: 0.0,
            invalid_coeffs: true,
            invalid_delays: true,
        };
        f.update_coeffs();
        f
    }

    pub fn lowpass12(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::LowPass, Slope::Slope12, sample_rate, cutoff, res)
    }

    pub fn highpass12(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::HighPass, Slope::Slope12, sample_rate, cutoff, res)
    }

    pub fn bandpass12(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::BandPass, Slope::Slope12, sample_rate, cutoff, res)
    }

    pub fn bandstop12(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::BandStop, Slope::Slope12, sample_rate, cutoff, res)
    }

    pub fn lowpass24(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::LowPass, Slope::Slope24, sample_rate, cutoff, res)
    }

    pub fn highpass24(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::HighPass, Slope::Slope24, sample_rate, cutoff, res)
    }

    pub fn bandpass24(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::BandPass, Slope::Slope24, sample_rate, cutoff, res)
    }

    pub fn bandstop24(sample_rate: f64, cutoff: f64, res: f64) -> IIR {
        IIR::new(Design::BandStop, Slope::Slope24, sample_rate, cutoff, res)
    }

    fn update_coeffs(&mut self) {
        self.coeff = match self.design {
            Design::LowPass => Coeffs::lowpass(self.sample_rate, self.cutoff, self.res),
            Design::HighPass => Coeffs::highpass(self.sample_rate, self.cutoff, self.res),
            Design::BandPass => Coeffs::bandpass(self.sample_rate, self.cutoff, self.res),
            Design::BandStop => Coeffs::bandstop(self.sample_rate, self.cutoff, self.res),
        }
    }

    fn reset_delays(&mut self) {
        self.d1 = 0.0; self.d2 = 0.0; self.d3 = 0.0; self.d4 = 0.0;
    }
}

impl Filter for IIR {
    fn set_enabled(&mut self, enabled: bool) {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.invalid_delays = true;
        }
    }

    fn set_type(&mut self, design: Design) {
        self.design = design;
        self.invalid_coeffs = true;
        self.invalid_delays = true;
    }

    fn set_slope(&mut self, slope: Slope) {
        self.slope = slope;
        self.invalid_coeffs = true;
        self.invalid_delays = true;
    }

    fn set_cutoff(&mut self, cutoff: f64) {
        if (self.cutoff - cutoff).abs() >= CUTOFF_DELTA {
            self.cutoff = limit_cutoff(cutoff, self.sample_rate);
            self.invalid_coeffs = true;
        }
    }

    fn set_resonance(&mut self, res: f64) {
        if self.res != res {
            self.res = res;
            self.invalid_coeffs = true;
        }
    }

    fn process(&mut self, signal: f64) -> f64 {
        if self.enabled {
            if self.invalid_coeffs {
                if self.invalid_delays {
                    self.reset_delays();
                }

                self.update_coeffs();
            }

            let Coeffs { a0, a1, a2, b1, b2 } = self.coeff;

            match self.slope {
                Slope::Slope12 => {
                    let out =           a0 * signal + self.d1;
                    self.d1 = self.d2 + a1 * signal - b1 * out;
                    self.d2 =           a2 * signal - b2 * out;
                    self.d3 = 0.0;
                    self.d4 = 0.0;
                    out
                },

                Slope::Slope24 => {
                    let out =           a0 * signal + self.d1;
                    self.d1 = self.d2 + a1 * signal - b1 * out;
                    self.d2 =           a2 * signal - b2 * out;
                    let signal = out;
                    let out =           a0 * signal + self.d3;
                    self.d3 = self.d4 + a1 * signal - b1 * out;
                    self.d4 =           a2 * signal - b2 * out;
                    out
                }
            }
        }
        else {
            signal
        }
    }
}
