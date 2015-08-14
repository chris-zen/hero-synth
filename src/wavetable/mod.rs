
pub enum Stock {
    Sin = 0,
    Saw,
}

impl Stock {
    pub fn from_name(name: &str) -> Option<Stock> {
        match name {
            "sin" => Some(Stock::Sin),
            "saw" => Some(Stock::Saw),
            _ => None
        }
    }
}

pub struct Wavetable<'a> {
    data: &'a [f64],
}

impl<'a> Default for Wavetable<'a> {
    fn default() -> Self {
        Wavetable {data: sin::LUT}
    }
}

impl<'a> Wavetable<'a> {
    pub fn new(data: &'a [f64]) -> Wavetable<'a> {
        Wavetable {
            data: data
        }
    }

    pub fn from_stock(stock: Stock) -> Wavetable<'a> {
        match stock {
            Stock::Sin => Wavetable {data: sin::LUT},
            Stock::Saw => Wavetable {data: saw::LUT},
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
pub mod saw;
