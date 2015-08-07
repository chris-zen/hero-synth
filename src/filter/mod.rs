pub enum Design {
    LowPass = 0,
    HighPass,
    BandPass,
    BandStop
}

pub enum Slope {
    Slope12 = 0,
    Slope24
}

pub trait Filter {
    fn set_enabled(&mut self, enabled: bool);
    fn set_type(&mut self, design: Design);
    fn set_slope(&mut self, slope: Slope);
    fn set_cutoff(&mut self, cutoff: f64);
    fn set_resonance(&mut self, res: f64);
    fn process(&mut self, signal: f64) -> f64;
}

pub mod iir;
