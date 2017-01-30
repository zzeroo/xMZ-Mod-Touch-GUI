use sensor;

pub struct CoNo2KombisensorMod {
    version: u32,
    modbus_address: u8,
    sensors: Vec<Sensor>,
}

impl CoNo2KombisensorMod {
    pub fn new() -> CoNo2KombisensorMod {
        CoNo2KombisensorMod {
            version: 0,
            modbus_address: 0,
            sensors: vec![],
        }
    }
}
