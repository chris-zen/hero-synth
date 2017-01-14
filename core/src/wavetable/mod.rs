
mod sin;
mod saw;

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

pub struct Wavetable {
    data: Vec<f64>,
}

impl Clone for Wavetable {
    fn clone(&self) -> Self { Wavetable { data: self.data.clone() } }
}

impl Default for Wavetable {
    fn default() -> Self {
        Wavetable { data: sin::LUT.to_vec() }
    }
}

impl Wavetable {
    pub fn new(data: Vec<f64>) -> Wavetable {
        Wavetable {
            data: data
        }
    }

    pub fn from_stock(stock: Stock) -> Wavetable {
        match stock {
            Stock::Sin => Wavetable { data: sin::LUT.to_vec() },
            Stock::Saw => Wavetable { data: saw::LUT.to_vec() },
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
