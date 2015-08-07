
use oscillator::Oscillator;
use filter::Filter;
use patch::Patch;

const NUM_OSC: usize = 8;
const NUM_FILTERS: usize = 2;

struct Voice<'a, 'b> {
    patch: &'b Patch,
    sample_rate: f64,
    oscillators: Vec<&'a Oscillator<'a>>,
    filters: Vec<&'a Filter>,
    freq: f64,
    amplitude: f64,
}

impl<'a, 'b> Voice<'a, 'b> {
    pub fn new(sample_rate: f64, patch: &'b Patch) -> Voice<'a, 'b> {
        let mut v = Voice {
            patch: patch,
            sample_rate: sample_rate,
            oscillators: Vec::with_capacity(NUM_OSC),
            filters: Vec::with_capacity(NUM_FILTERS),
            freq: 440.0,
            amplitude: 1.0,
        };
        v
    }

    fn reset(&mut self) {
        // Oscillators
        for osc in &self.patch.oscs {
            let o = Oscillator::from_patch(self.sample_rate, osc);
            self.oscillators.push(&o);
        }
        // TODO reset filters
    }

    pub fn set_patch(&mut self, patch: &'b Patch) {
        self.patch = patch;

        self.reset();
    }

    pub fn process(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}
