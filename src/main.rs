extern crate gdk;
extern crate gtk;

use gdk::Screen;
use gtk::{Builder, Notebook, Window};
use gtk::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::thread;
use std::time::Duration;


fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initalize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

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


    window.show_all();

    gtk::main();
}
