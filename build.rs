extern crate wfgen;

const WT_SRC: &'static str = "src/wavetable";

fn main() {
    wfgen::gen_lut(WT_SRC, "sin",
                   &mut wfgen::sin::SinGen::new(wfgen::DEFAULT_WAVETABLE_SIZE)).unwrap();

   wfgen::gen_lut(WT_SRC, "saw",
                  &mut wfgen::saw::SawGen::new(wfgen::DEFAULT_WAVETABLE_SIZE)).unwrap();
}
