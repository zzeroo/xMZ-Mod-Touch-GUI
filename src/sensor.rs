use libmodbus_rs;
use libmodbus_rs::modbus::Modbus;


pub struct Sensor {
    pub adc_value: u16,
}

impl Sensor {
    pub fn new() -> Self {
        Sensor {
            adc_value: 0,
        }
    }

    pub fn update_adc(&mut self) {
        let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
        let _ = modbus.set_slave(46);
        // let _ = modbus.set_debug(true);
        let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
        let mut tab_reg: Vec<u16> = vec![];

        match modbus.connect() {
            Err(_) => { modbus.free(); }
            Ok(_) => {
                tab_reg = modbus.read_registers(1, 1);
            }
        }
        tab_reg.get(0).map(|var| self.adc_value = *var);
    }
}
