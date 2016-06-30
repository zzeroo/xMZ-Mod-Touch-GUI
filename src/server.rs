use module::Module;
use sensor::Sensor;

pub struct Server {
    pub modules: Vec<Module>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            modules: vec![],
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
