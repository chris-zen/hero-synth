//!
//! Infinite Impulse Response (IIR) filters
//!

use std::f64::consts::PI;
use std::cmp;
use std::fmt::Display;
use std::fmt;

const FREQ_DELTA: f64 = 0.01;
const FREQ_MIN: f64 = 10.0;

fn limit_freq(freq: f64, sample_rate: f64) -> f64 {
    freq.max(FREQ_MIN).min(sample_rate / 2.0)
}

#[derive(Debug)]
pub struct Coeffs([f64; 5]);

impl Display for Coeffs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..5 {
            try!(write!(f, "{}, ", self.0[i]));
        }
        write!(f, "\n")
    }
}
impl Default for Coeffs {
     fn default() -> Self {
         Coeffs([0.0; 5])
     }
}

impl Coeffs {
    pub fn new(c1: f64, c2: f64, c3: f64, c4: f64, c5: f64, c6: f64) -> Coeffs {
        let a = 1.0 / c4;
        Coeffs([
            c1 * a,
            c2 * a,
            c3 * a,
            c5 * a,
            c6 * a])
    }

    fn lowpass(sample_rate: f64, freq: f64) -> Coeffs {
        let n: f64 = 1.0 / (PI * freq / sample_rate).tan();
        let n_sq = n * n;
        let c1 = 1.0 / (1.0 + (2.0f64).sqrt() * n + n_sq);

        Coeffs::new(
            c1,
            c1 * 2.0,
            c1,
            1.0,
            c1 * 2.0 * (1.0 - n_sq),
            c1 * (1.0 - (2.0f64).sqrt() * n + n_sq))
    }

    fn highpass(sample_rate: f64, freq: f64) -> Coeffs {
        let n: f64 = 1.0 / (PI * freq / sample_rate).tan();
        let n_sq = n * n;
        let c1 = 1.0 / (1.0 + (2.0f64).sqrt() * n + n_sq);

        Coeffs::new(
            c1,
            c1 * -2.0,
            c1,
            1.0,
            c1 * 2.0 * (n_sq - 1.0),
            c1 * (1.0 - (2.0f64).sqrt() * n + n_sq))
    }


}

pub enum Design {
    LowPass = 0,
    HighPass
}

pub struct IIR {
    design: Design,
    sample_rate: f64,
    freq: f64,
    q: f64,
    gain: f64,
    enabled: bool,
    pub coeff: Coeffs,
    v1: f64,
    v2: f64,
}

impl IIR {
    pub fn new(design: Design, sample_rate: f64, freq: f64, q: f64, gain: f64) -> IIR {
        assert!(sample_rate > 0.0);
        assert!(freq >= 0.0);
        assert!(q >= 0.0);

        let mut f = IIR {
            design: design,
            sample_rate: sample_rate,
            freq: limit_freq(freq, sample_rate),
            q: q,
            gain: gain,
            enabled: true,
            coeff: Coeffs::default(),
            v1: 0.0,
            v2: 0.0
        };
        f.update_coeff();
        f
    }

    pub fn lowpass(sample_rate: f64, freq: f64) -> IIR {
        IIR::new(Design::LowPass, sample_rate, freq, 0.0, 0.0)
    }

    pub fn highpass(sample_rate: f64, freq: f64) -> IIR {
        IIR::new(Design::HighPass, sample_rate, freq, 0.0, 0.0)
    }

    fn update_coeff(&mut self) {
        self.coeff = match self.design {
            Design::LowPass => Coeffs::lowpass(self.sample_rate, self.freq),
            Design::HighPass => Coeffs::highpass(self.sample_rate, self.freq),
        }
    }

    pub fn update_freq(&mut self, freq: f64) {
        if (self.freq - freq).abs() >= FREQ_DELTA {
            self.freq = limit_freq(freq, self.sample_rate);
            self.update_coeff();
        }
    }

    pub fn update_q(&mut self, q: f64) {
        if self.q != q {
            self.q = q;
            self.update_coeff();
        }
    }

    pub fn process(&mut self, signal: f64) -> f64 {
        let c = self.coeff.0;
        let out = c[0] * signal + self.v1;

        self.v1 = c[1] * signal - c[3] * out + self.v2;
        self.v2 = c[2] * signal - c[4] * out;

        out
    }
}
