
//use std::io;
//use std::fs::File;
//use std::path::Path;

pub struct SendLevel {
    pub index: usize,
    pub level: f64
}

pub struct Osc {
    pub is_enabled: bool,
    pub wavetable: String,
    pub initial_phase: f64,
    pub amplitude: f64,            // Oscillator signal amplitude
    pub is_fixed_freq: bool,       // When it is true the baseFrequency doesn't change with noteOn
    pub base_frequency: f64,       // Oscillator base frequency
    pub octaves: i32,              // Number of octaves to shift from the base_frequency
    pub semitones: i32,            // Number of semitones to shift from the base_frequency
    pub detune: f64,               // Fine shift from the base_frequency

    pub am_send: Vec<SendLevel>,   // Send levels for amplitude modulation
    pub fm_send: Vec<SendLevel>,   // Send levels for frequency modulation
    pub filt_send: Vec<SendLevel>, // Send levels for the filter input

    pub panning: f64,              // Panning [-1, +1]
    pub level: f64,                // Mix level
}

pub struct Filter {
    // TODO filter params
    pub mode: String,
    pub slope: String,
    pub freq: f64,
    pub res: f64,

    pub am_send: Vec<SendLevel>,   // Send levels for amplitude modulation
    pub fm_send: Vec<SendLevel>,   // Send levels for frequency modulation
    pub filt_send: Vec<SendLevel>, // Send levels for the filter input

    pub panning: f64,              // Panning [-1, +1]
    pub level: f64,                // Mix level
}

pub struct Patch {
    pub oscillators: Vec<Osc>,
    pub filters: Vec<Filter>,
}

/*
pub fn load(path: &Path) -> io::Result<Patch> {
    let mut file = try!(File::open(path));
    let mut data = String::new();
    try!(file.read_to_string(&mut data))
    //try!(Json::from_str(&data))
}
*/
