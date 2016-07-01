use libmodbus_rs;
use libmodbus_rs::modbus::Modbus;
use std::env;

pub struct Sensor {
    pub name: String,
    pub adc_value: u16,
    pub modbus_slave_id: i32,
}

impl Sensor {
    pub fn new(slave_id: i32) -> Self {
        Sensor {
            name: "Sensor".to_string(),
            adc_value: 0,
            modbus_slave_id: slave_id,
        }
    }

    pub fn update_adc(&mut self) {
        let device = match env::var("XMZ_HARDWARE") {
            Ok(_) => "/dev/ttyS1",
            Err(_) => "/dev/ttyUSB0",
        };
        let mut modbus = Modbus::new_rtu(device, 9600, 'N', 8, 1);
        modbus.set_slave(self.modbus_slave_id);
        let _ = modbus.set_debug(true);
        let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
        let mut tab_reg: Vec<u16> = vec![];

        match modbus.connect() {
            Err(_) => { modbus.free(); }
            Ok(_) => {
                tab_reg = modbus.read_registers(1, 1);
            }
        }
        tab_reg.get(0).map(|var| self.adc_value = *var);
        modbus.close();
        modbus.free();
    }
}
