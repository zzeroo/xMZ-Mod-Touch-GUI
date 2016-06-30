use module::Module;
use sensor::Sensor;

pub struct Server {
    pub modules: Vec<Module>,
    sensor_index: Vec<Sensor>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            modules: vec![],
            sensor_index: vec![],
        }
    }

    pub fn init(&mut self) {
        let mut module = Module::new();
        let mut sensor1 = Sensor::new();
        let mut sensor2 = Sensor::new();
        sensor1.modbus_slave_id = Some(46);
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        self.modules.push(module);
    }

    pub fn refresh_all_sensors(&mut self) {
        for module in self.modules.iter_mut() {
            for sensor in module.sensors.iter_mut() {
                sensor.update_adc();
                println!("{:?}", sensor.adc_value);
            }
        }
    }

    pub fn get_sensor_index<'a>(&'a self) -> &'a Vec<Sensor> {
        &self.sensor_index
    }
}

#[cfg(test)]
mod test {
    use server::{Server};
    use module::{Module};

    #[test]
    fn add_modules() {
        let mut server = Server::new();
        let module1 = Module::new();

        assert_eq!(server.modules.len(), 0);
        server.modules.push(module1);
        assert_eq!(server.modules.len(), 1);
    }
}
