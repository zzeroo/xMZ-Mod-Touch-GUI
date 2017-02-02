#[macro_use] extern crate log;
extern crate env_logger;
extern crate gtk;
#[macro_use] extern crate xmz_mod_touch_gui;

use gtk::prelude::*;
use xmz_mod_touch_gui::error::*;
use xmz_mod_touch_gui::gui;


fn run() -> Result<()> {
    xmz_mod_touch_gui::application::launch()?;

    Ok(())
}


fn main() {
    // Initialisiere den Logger (erst nach diesem sind `trace!()`, `debug!()` usw funktional)
    env_logger::init().unwrap();

    println!("xMZ-Mod-Touch-GUI Version: {}\n",
             env!("CARGO_PKG_VERSION"));

    if gtk::init().is_err() {
        error!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }

    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}
