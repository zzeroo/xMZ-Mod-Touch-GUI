use libmodbus_rs;
use libmodbus_rs::modbus::Modbus;
use std::env;
use std::fs;
use std::result;
use std::fmt;

pub enum SensorError {
    InvalidValue,
    NoADCValue,
    NoADCValueAtNullgas,
    NoADCValueAtMessgas,
    NoConcentrationNullas,
    NoConcentrationMessgas,
}

// Rust type alias
pub type Result<T> = result::Result<T, SensorError>;

pub enum SensorType {
    /// Nemoto NO2 Messzelle, EC NAP-550 https://www.nemoto.co.jp/nse/sensor-search/nap-550.html?lang=en
    NemotoNO2,
    /// Nemote CO Messzelle, EC NAP-505 https://www.nemoto.co.jp/nse/sensor-search/use/gas-alarm/nap-505.html?lang=en
    NemotoCO,
}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SensorType::NemotoNO2 => write!(f, "Nemoto NO2"),
            SensorType::NemotoCO => write!(f, "Nemoto CO"),
        }
    }
}

pub struct Sensor {
    /// Sensor Typ
    pub sensor_type: SensorType,
    /// ADC Wert    - wird vom Server Prozess über das Modbus Protokoll ausgelesen und aktualisiert
    pub adc_value: u16,
    /// SI Einheit
    pub si: String,
    adc_value_at_nullgas: Option<u32>,
    adc_value_at_messgas: Option<u32>,
    concentration_nullgas: Option<u32>,
    concentration_messgas: Option<u32>,
    /// Adresse des Modbus Registers für den ADC Wert
    pub modbus_register_address: u32,
}

impl Sensor {
    /// Erzeugt eine neue Sensor Instanz
    ///
    /// # Attributes
    /// * `sensor_type`     - `SensorType` Type der Messzelle
    /// * `modbus_slave_id` - Modbus Slave Adresse
    ///
    pub fn new(sensor_type: SensorType, modbus_slave_id: u16) -> Self {
        match sensor_type {
            SensorType::NemotoNO2 => {
                Sensor {
                    sensor_type: SensorType::NemotoNO2,
                    adc_value: modbus_slave_id,
                    si: "ppm".to_string(),
                    adc_value_at_nullgas: Some(922),  // TODO: Read in sensor calibration data
                    adc_value_at_messgas: Some(622),  // TODO: Read in sensor calibration data
                    concentration_nullgas: Some(0),  // TODO: Read in sensor calibration data
                    concentration_messgas: Some(20),  // TODO: Read in sensor calibration data
                    modbus_register_address: 1,
                }
            },
            SensorType::NemotoCO => {
                Sensor {
                    sensor_type: SensorType::NemotoCO,
                    adc_value: modbus_slave_id,
                    si: "ppm".to_string(),
                    adc_value_at_nullgas: Some(107),  // TODO: Read in sensor calibration data
                    adc_value_at_messgas: Some(888),  // TODO: Read in sensor calibration data
                    concentration_nullgas: Some(0),  // TODO: Read in sensor calibration data
                    concentration_messgas: Some(280),  // TODO: Read in sensor calibration data
                    modbus_register_address: 11,
                }
            },
        }
    }

    pub fn update_adc(&mut self) {
        let device = match env::var("XMZ_HARDWARE") {
            Ok(_) => "/dev/ttyS1",
            Err(_) => "/dev/ttyUSB0",
        };
        // Wenn das Device vorhanden ist wird die Update Logic durchlaufen, ansonnsten nur ein
        // Fehler aus gegeben.
        match fs::metadata(device) {
            Ok(_) => {
                let mut modbus = Modbus::new_rtu(device, 9600, 'N', 8, 1);
                //modbus.set_slave(self.modbus_slave_id);
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
            Err(err) => { println!("Modbus Device: {} ist nicht erreichbar: {}", device, err); }
        }
    }
}
