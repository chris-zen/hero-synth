const PI_2: f64 = 6.28318530717958647692528676655900576f64;

pub struct SinGen {
    count: usize,
    phase: f64,
    step: f64,
}

impl SinGen {
    pub fn new(size: usize) -> SinGen {
        SinGen {
            count: size,
            phase: 0.0,
            step: PI_2 / (size as f64),
        }
    }
}

impl Iterator for SinGen {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.count == 0 {
            None
        }
        else {
            let value = self.phase.sin();
            self.phase += self.step;
            self.count -= 1;
            Some(value)
        }
    }
}
