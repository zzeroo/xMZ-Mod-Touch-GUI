extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate libmodbus_rs;

mod server;
mod module;
mod sensor;

use gdk::Screen;
use gtk::{Builder, Notebook, Window};
use gtk::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use server::*;


fn update_window(server: &Rc<RefCell<server::Server>>) {
    let mut server = server.borrow_mut();
    for module in server.modules.iter_mut() {
        for sensor in module.sensors.iter_mut() {
            sensor.update_adc();
            println!("{:?}", sensor.adc_value);
        }
    }

}

fn window_setup(window: &gtk::Window) {
    // Window properties
    let window_title = "xMZ-Mod-Touch GUI ".to_string() + env!("CARGO_PKG_VERSION");
    window.set_title(&window_title);
    window.set_default_size(1024, 600);


    match env::var("XMZ_HARDWARE") {
        Ok(_) => {
            window.fullscreen();
        },
        Err(_) => {
            // Connect ESC key press event, and quit the gui if ESC was pressed
            window.connect_key_press_event(move |_, key| {
                match key.get_keyval() as u32 {
                    gdk::enums::key::Escape => gtk::main_quit(),
                    _ => (),
                }
                Inhibit(false)
            });
        },
    }

    // Connect delete event to quit the gtk::main thread
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
}


fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initalize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    let mut server = Server::new();
    server.init();

    let srv = Rc::new(RefCell::new(server));
    let mut notebook = Notebook::new();

    window_setup(&window);

    gtk::timeout_add(1000, move || {
        update_window(&srv);

        glib::Continue(true)
    });

    window.show_all();

    gtk::main();
}
