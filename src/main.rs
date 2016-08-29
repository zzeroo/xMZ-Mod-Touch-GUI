#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_mod_touch_gui;

use xmz_mod_touch_gui::gui;

fn main() {
    let application = gui::gtk3::App::new();
    application.launch();
}
