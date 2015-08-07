
pub struct Wavetable<'a> {
    data: &'a [f64],
}

impl<'a> Wavetable<'a> {
    pub fn new(data: &'a [f64]) -> Wavetable {
        Wavetable {
            data: data
        }
    }

    pub fn from_name<'b>(name: &'b str) -> Wavetable {
        match name {
            "sin" => Wavetable {data: sin::LUT},
            "saw" => Wavetable {data: saw::LUT},
            _ => panic!(format!("Unknown wavetable: {}", name)),
        }
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn value(&self, offset: f64) -> f64 {
        let data_len = self.data.len();
        let pos: usize = offset.floor() as usize;
        assert!(pos < data_len);
        let value = self.data[pos];

        let next_pos: usize = (pos + 1) % data_len;
        let next_value = self.data[next_pos];

        let diff = next_value - value;
        let fraction = offset - (pos as f64);
        return value + diff * fraction;
    }
}
pub mod sin;

pub static SIN: Wavetable<'static> = Wavetable {data: sin::LUT};

pub mod saw;

pub static SAW: Wavetable<'static> = Wavetable {data: saw::LUT};
