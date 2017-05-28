#[macro_use] extern crate log;
#[macro_use] extern crate xmz_mod_touch_gui;
extern crate env_logger;
extern crate gtk;

use gtk::prelude::*;
use xmz_mod_touch_gui::application;
use xmz_mod_touch_gui::error::*;


fn run() -> Result<()> {
    // application::launch()?;

    Ok(())
}


fn main() {
    // Initialisiere den Logger (erst nach diesem sind `trace!()`, `debug!()` usw funktional)
    env_logger::init().unwrap();

    println!("xMZ-Mod-Touch-GUI Version: {}\n", env!("CARGO_PKG_VERSION"));

    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}
