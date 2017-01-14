//!
//! Non linear panning using a wavetable
//!

use wavetable::{self, Wavetable};

pub struct Panning {
    left: f64,
    right: f64,
    value: f64,
    wavetable: Wavetable
}

impl Default for Panning {
    fn default() -> Self {
        Panning {
            left: 0.5,
            right: 0.5,
            value: 0.0,
            wavetable: Wavetable::from_stock(wavetable::Stock::Sin)
        }
    }
}

impl Panning {
    pub fn new(value: f64) -> Panning {
        let mut p = Panning::default();
        p.set_value(value);
        p
    }

    pub fn set_value(&mut self, value: f64) {
        if self.value != value {
            let wt_size = self.wavetable.size() as f64;
            self.left = self.wavetable.value(((1.0 - value) / 8.0) * wt_size);
            self.right = self.wavetable.value(((1.0 + value) / 8.0) * wt_size);
            self.value = value;
        }
    }

    pub fn process(&self, signal: f64) -> (f64, f64) {
        (signal * self.left, signal * self.right)
    }
}
