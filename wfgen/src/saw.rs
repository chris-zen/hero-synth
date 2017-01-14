pub struct SawGen {
    count: usize,
    phase: f64,
    step: f64,
}

impl SawGen {
    pub fn new(size: usize) -> SawGen {
        SawGen {
            count: size,
            phase: 0.0,
            step: 2.0 / (size as f64),
        }
    }
}

impl Iterator for SawGen {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.count == 0 {
            None
        }
        else {
            let value = -1.0 + self.phase;
            self.phase += self.step;
            self.count -= 1;
            Some(value)
        }
    }
}
