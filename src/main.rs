extern crate gdk;
extern crate glib;
extern crate gtk;

use gdk::Screen;
use gtk::{Builder, Window};
use gtk::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};

fn update_window(list: &gtk::ListStore) {
    // for (id, sensor) in sensors.iter() {
    //     if !seen.contains(id) {
    //         create_and_fill_model(list, sensor.id, sensor.sensor_type.to_string(), sensor.concentration().unwrap_or(0.0).to_string(), sensor.adc_value.unwrap_or(0) as u32);
    //     }
    // }
}

fn window_setup(window: &gtk::Window) {
    // Window basic properties
    let window_title = format!("{} {}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(150.0);

    // env! Macro funktioniert hier nicht wenn XMZ_HARDWARE nicht existiert.
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
    window_setup(&window);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // v_box.pack_start(&notebook.notebook, true, true, 0);

    window.add(&v_box);
    window.show_all();

    gtk::timeout_add(2000, move || {
        // update_window(&list_store);

        glib::Continue(true)
    });

    gtk::main();
}
