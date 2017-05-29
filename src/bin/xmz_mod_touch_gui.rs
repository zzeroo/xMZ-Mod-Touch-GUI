extern crate log;
extern crate xmz_mod_touch_gui;
extern crate env_logger;

use xmz_mod_touch_gui::error::*;
use xmz_mod_touch_gui::application;


fn run() -> Result<()> {
    application::launch()?;

    Ok(())
}

fn main() {
    // Initalisiere Logger (erst nach diesem Aufruf sind `trace!()`, `debug!()` usw. functional)
    env_logger::init().unwrap();

    if let Err(ref e) = run() {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}
