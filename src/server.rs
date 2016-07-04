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
        let mut module1 = Module::new();
        let mut module2 = Module::new();
        let mut module3 = Module::new();
        let mut module4 = Module::new();
        let mut module5 = Module::new();
        let mut module6 = Module::new();
        let mut sensor1 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor2 = Sensor::new(SensorType::NemotoCO);
        let mut sensor3 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor4 = Sensor::new(SensorType::NemotoCO);
        let mut sensor5 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor6 = Sensor::new(SensorType::NemotoCO);
        let mut sensor7 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor8 = Sensor::new(SensorType::NemotoCO);
        let mut sensor9 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor10 = Sensor::new(SensorType::NemotoCO);
        let mut sensor11 = Sensor::new(SensorType::NemotoNO2);
        let mut sensor12 = Sensor::new(SensorType::NemotoCO);
        sensor1.id = 1;
        sensor2.id = 2;
        sensor3.id = 3;
        sensor4.id = 4;
        sensor5.id = 5;
        sensor6.id = 6;
        sensor7.id = 7;
        sensor8.id = 8;
        sensor9.id = 9;
        sensor10.id = 10;
        sensor11.id = 11;
        sensor12.id = 12;
        module1.sensors.push(sensor1);
        module1.sensors.push(sensor2);
        module2.sensors.push(sensor3);
        module2.sensors.push(sensor4);
        module3.sensors.push(sensor5);
        module3.sensors.push(sensor6);
        module4.sensors.push(sensor7);
        module4.sensors.push(sensor8);
        module5.sensors.push(sensor9);
        module5.sensors.push(sensor10);
        module6.sensors.push(sensor11);
        module6.sensors.push(sensor12);
        module1.modbus_slave_id = 23;
        module2.modbus_slave_id = 24;
        module3.modbus_slave_id = 25;
        module4.modbus_slave_id = 26;
        module5.modbus_slave_id = 27;
        module6.modbus_slave_id = 28;
        self.modules.push(module1);
        self.modules.push(module2);
        self.modules.push(module3);
        self.modules.push(module4);
        self.modules.push(module5);
        self.modules.push(module6);
    }

    pub fn refresh_all(&mut self) {
        self.refresh_sensors();
    }

    pub fn refresh_sensors(&mut self) {
        for module in self.modules.iter_mut() {
            for sensor in module.sensors.iter_mut() {
                println!("\nModbus Adresse: {} ADC: {:?} Konzentration: {:?}", module.modbus_slave_id, sensor.adc_value, sensor.concentration().unwrap_or(0.0));
                sensor.update_adc(module.modbus_slave_id);
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
