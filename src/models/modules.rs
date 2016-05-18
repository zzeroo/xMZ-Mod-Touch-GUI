use super::sensors::Sensor;

pub struct Modules {
    pub name: String,
    // sensors: Vec<Sensors>
}

impl Modules {
    pub fn new(name: String) -> Self {
        Modules { name: name }
    }

    pub fn get_sensors(&self) -> Vec<Sensor> {
        (1..3).map(|id| Sensor::new(format!("Sensor: {}", id))).collect()
    }
}
