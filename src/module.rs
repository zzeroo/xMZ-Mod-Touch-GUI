use sensor::Sensor;

pub struct Module {
    pub sensors: Vec<Sensor>,
}

impl Module {
    pub fn new() -> Self {
        Module {
            sensors: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use module::Module;
    use sensor::Sensor;

    #[test]
    fn add_sensors() {
        let mut module = Module::new();
        let sensor1 = Sensor::new();
        let sensor2 = Sensor::new();

        assert_eq!(module.sensors.len(), 0);
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        assert_eq!(module.sensors.len(), 2);
    }
}
