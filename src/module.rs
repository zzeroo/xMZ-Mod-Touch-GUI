use sensor::*;

pub struct Module {
    pub sensors: Vec<Sensor>,
    pub modbus_slave_id: i32,
}

impl Module {
    pub fn new() -> Self {
        Module {
            sensors: vec![],
            modbus_slave_id: 1,
        }
    }
}


#[cfg(test)]
mod tests {
    use module::Module;
    use sensor::*;

    #[test]
    fn add_sensors() {
        let mut module = Module::new();
        let sensor1 = Sensor::new(SensorType::NemotoNO2, 1);
        let sensor2 = Sensor::new(SensorType::NemotoCO, 1);

        assert_eq!(module.sensors.len(), 0);
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        assert_eq!(module.sensors.len(), 2);
    }
}
