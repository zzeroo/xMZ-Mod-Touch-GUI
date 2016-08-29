// Zur Zeit total nutzloses File, evtl. kicken...
#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_mod_touch_gui;

fn main() {
    trace!("Initialisiere den Logger");
    env_logger::init().unwrap();

    println!("{}", xmz_mod_touch_gui::sysinfo::ifconfig());
}
