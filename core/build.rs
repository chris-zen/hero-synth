extern crate hero_wfgen;

use hero_wfgen::{DEFAULT_WAVETABLE_SIZE, gen_lut, sin, saw};

const WT_SRC: &'static str = "src/wavetable";

fn main() {
    let mut sin_gen = sin::SinGen::new(DEFAULT_WAVETABLE_SIZE);
    gen_lut(WT_SRC, "sin", &mut sin_gen).unwrap();

    let mut saw_gen = saw::SawGen::new(DEFAULT_WAVETABLE_SIZE);
    gen_lut(WT_SRC, "saw", &mut saw_gen).unwrap();
}
