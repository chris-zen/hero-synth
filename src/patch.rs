
//use std::io;
//use std::fs::File;
//use std::path::Path;

pub struct Osc {
    pub is_enabled: bool,
    pub wavetable: String,
    pub initial_phase: f64,
    pub amplitude: f64,         // Oscillator signal amplitude
    pub is_fixed_freq: bool,    // When it is true the baseFrequency doesn't change with noteOn
    pub base_frequency: f64,    // Oscillator base frequency
    pub octaves: i32,           // Number of octaves to shift from the base_frequency
    pub semitones: i32,         // Number of semitones to shift from the base_frequency
    pub detune: f64,            // Fine shift from the base_frequency
}

pub struct OscFm {
    pub src: usize,
    pub dst: usize,
    pub amplitude: f64
}

pub struct Patch {
    pub oscs: Vec<Osc>,
    pub osc_fm: Vec<OscFm>,
}

/*
pub fn load(path: &Path) -> io::Result<Patch> {
    let mut file = try!(File::open(path));
    let mut data = String::new();
    try!(file.read_to_string(&mut data))
    //try!(Json::from_str(&data))
}
*/
