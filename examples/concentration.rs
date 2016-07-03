extern crate xmz_mod_touch_gui;

use xmz_mod_touch_gui::sensor::*;

fn main() {
    let sensor = Sensor::new(SensorType::NemotoNO2);
    println!("{:?}", sensor.concentration());
}
