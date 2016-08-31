#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_mod_touch_gui;

use xmz_mod_touch_gui::common::*;
use xmz_mod_touch_gui::gui;



fn main() {
    trace!("Initialisiere den Logger");
    env_logger::init().unwrap();

    let application = gui::gtk3::App::new();
    match application.launch() {
        Ok(_) => {}
        Err(err) => { report_error(&err); }
    }
}
