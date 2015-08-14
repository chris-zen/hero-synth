pub enum Mode {
    ByPass = 0,
    LowPass,
    HighPass,
    BandPass,
    BandStop
}

pub enum Slope {
    Slope12 = 0,
    Slope24
}

pub trait Filter {
    fn reset(&mut self);
    fn set_enabled(&mut self, enabled: bool);
    fn set_mode(&mut self, mode: Mode);
    fn set_slope(&mut self, slope: Slope);
    fn set_cutoff(&mut self, cutoff: f64);
    fn set_resonance(&mut self, res: f64);
    fn process(&mut self, signal: f64) -> f64;
}

pub mod iir;
