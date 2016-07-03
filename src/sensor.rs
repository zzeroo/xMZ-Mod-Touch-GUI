use libmodbus_rs;
use libmodbus_rs::modbus::Modbus;
use std::env;
use std::fs;
use std::result;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum SensorError {
    InvalidValue,
    NoADCValue,
    NoADCValueAtNullgas,
    NoADCValueAtMessgas,
    NoConcentrationNullgas,
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
    pub adc_value: Option<u16>,
    /// SI Einheit
    pub si: String,
    /// Adresse des Modbus Registers für den ADC Wert
    pub modbus_register_address: u32,
    /// Kalibrationspunkt Nullgas
    adc_value_at_nullgas: Option<u32>,
    /// Kalibrationspunkt Messgas
    adc_value_at_messgas: Option<u32>,
    /// Kalibrationspunkt Nullgas
    concentration_nullgas: Option<u32>,
    /// Kalibrationspunkt Messgas
    concentration_messgas: Option<u32>,
}

impl Sensor {
    /// Erzeugt eine neue Sensor Instanz
    ///
    /// # Attributes
    /// * `sensor_type`     - `SensorType` Type der Messzelle
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::sensor::{Sensor, SensorError, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoNO2);
    /// ```
    pub fn new(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::NemotoNO2 => {
                Sensor {
                    sensor_type: SensorType::NemotoNO2,
                    adc_value: None,
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
                    adc_value: None,
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

    /// `concentration` - Liefert den aktuell, berechneten Wert zurück
    ///
    /// Der Wert wird mit einer linearen Funktion aus den Calibrationsdaten `adc_value_at_nullgas`,
    /// `adc_value_at_messgas`, `concentration_nullgas`, `concentration_messgas` und dem akuellen
    /// Analog/ Digital Wert `adc_value` berechnet.
    ///
    /// Die Funktion liefert ein `Result<u32, SensorError>` zurück.
    /// Im Erfolgsfall ein u32 ansonnsten ein SensorError.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::sensor::{Sensor, SensorError, SensorType};
    ///
    /// let mut sensor = Sensor::new(SensorType::NemotoNO2);
    /// assert_eq!(sensor.concentration(), Err(SensorError::NoADCValue));
    /// ```
    pub fn concentration(&self) -> Result<f32> {
        let x = match self.adc_value {
            None => {return Err(SensorError::NoADCValue); }
            Some(value) => {value}
        };
        let y2 = match self.concentration_messgas {
            None => { return Err(SensorError::NoConcentrationMessgas); }
            Some(value) => { value }
        };
        let y1 = match self.concentration_nullgas {
            None => { return Err(SensorError::NoConcentrationNullgas); }
            Some(value) => { value }
        };
        let x2 = match self.adc_value_at_messgas {
            None => { return Err(SensorError::NoADCValueAtMessgas); }
            Some(value) => { value }
        };
        let x1 = match self.adc_value_at_nullgas {
            None => { return Err(SensorError::NoADCValueAtNullgas); }
            Some(value) => { value }
        };

        let result: f32 = (y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32) * (x as f32 - x1 as f32) + y1 as f32;

        Ok(result)
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
                //TODO: Reenable this
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
                tab_reg.get(0).map(|var| self.adc_value = Some(*var));
                modbus.close();
                modbus.free();
            }
            Err(err) => { println!("Modbus Device: {} ist nicht erreichbar: {}", device, err); }
        }
    }
}


#[cfg(test)]
mod tests {
    use sensor::*;

    // Helper Funktion die ein NO2 Sensor mit default Werten zurück gibt
    fn default_no2_sensor() -> Sensor {
        let mut sensor = Sensor::new(SensorType::NemotoNO2);
        sensor.adc_value = Some(772);
        sensor.adc_value_at_nullgas = Some(922);
        sensor.concentration_nullgas = Some(0);
        sensor.adc_value_at_messgas = Some(622);
        sensor.concentration_messgas = Some(20);
        sensor
    }

    #[test]
    fn modbus_register_adresse_nemoto_no2() {
        let sensor = Sensor::new(SensorType::NemotoNO2);
        assert_eq!(sensor.modbus_register_address, 1);
    }

    #[test]
    fn modbus_register_address_nemoto_co() {
        let sensor = Sensor::new(SensorType::NemotoCO);
        assert_eq!(sensor.modbus_register_address, 11);
    }


    #[test]
    fn concentration() {
        let sensor = Sensor::new(SensorType::NemotoNO2);
    }

    // ADC
    #[test]
    fn concentration_should_fail_with_no_adc_value() {
        let mut sensor = default_no2_sensor();
        sensor.adc_value = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoADCValue));
    }

    #[test]
    fn concentration_should_fail_with_no_adc_value_at_nullgas() {
        let mut sensor = default_no2_sensor();
        sensor.adc_value_at_nullgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoADCValueAtNullgas));
    }

    #[test]
    fn concentration_should_fail_with_no_concentration_nullgas() {
        let mut sensor = default_no2_sensor();
        sensor.concentration_nullgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoConcentrationNullgas));
    }

    #[test]
    fn concentration_should_fail_with_no_concentration_messgas() {
        let mut sensor = default_no2_sensor();
        sensor.concentration_messgas = None;
        assert_eq!(sensor.concentration(), Err(SensorError::NoConcentrationMessgas));
    }

    #[test]
    fn concentration_no2() {
        let mut sensor = default_no2_sensor();
        assert_eq!(sensor.concentration().unwrap(), 10.000001);
    }

    #[test]
    fn concentration_co() {
        let mut sensor = Sensor::new(SensorType::NemotoCO);
        sensor.adc_value = Some(333);
        sensor.adc_value_at_nullgas = Some(114);
        sensor.concentration_nullgas = Some(0);
        sensor.adc_value_at_messgas = Some(875);
        sensor.concentration_messgas = Some(280);
        assert_eq!(sensor.concentration().unwrap(), 80.578186);
    }
}
