pub struct Sensor {
    pub name: String,
    pub adc_value: u32,
}

impl Sensor {
    pub fn new(name: String) -> Self {
        Sensor { name: name, adc_value: 0 }
    }
}
