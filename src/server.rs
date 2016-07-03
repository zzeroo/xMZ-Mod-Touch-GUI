use module::Module;
use sensor::*;
use std::collections::HashMap;

pub struct Server {
    pub modules: Vec<Module>,
    sensor_list: HashMap<i32, Sensor>,
}

impl Server {
    pub fn new() -> Self {
        let mut s = Server {
            modules: vec![],
            sensor_list: HashMap::new(),
        };
        s.refresh_all();
        s
    }

    pub fn init(&mut self) {
        let mut module = Module::new();
        let mut sensor1 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor2 = Sensor::new(SensorType::NemotoCO);
        let mut sensor3 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor4 = Sensor::new(SensorType::NemotoCO);
        let mut sensor5 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor6 = Sensor::new(SensorType::NemotoCO);
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        module.sensors.push(sensor3);
        module.sensors.push(sensor4);
        module.sensors.push(sensor5);
        module.sensors.push(sensor6);
        self.modules.push(module);
    }

    pub fn refresh_all(&mut self) {
        self.refresh_sensors();
    }

    pub fn refresh_sensors(&mut self) {
        for module in self.modules.iter_mut() {
            for sensor in module.sensors.iter_mut() {
                println!("\nModbus Adresse: {} ADC: {:?}", module.modbus_slave_id, sensor.adc_value);
                sensor.update_adc();
            }
        }
        self.refresh_sensor_list();
    }

    fn refresh_sensor_list(&mut self) {
        // self.sensor_list.entry(46).or_insert(self.modules[0].sensors[0].clone());
        // self.sensor_list.entry(46).or_insert(Sensor::new(66));
    }

    pub fn get_sensor_list<'a>(&'a self) -> &'a HashMap<i32, Sensor> {
        &self.sensor_list
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
