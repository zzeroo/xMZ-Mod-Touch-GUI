extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate libmodbus_rs;

mod server;
mod module;
mod sensor;
mod notebook;
mod sensor_index;

use gdk::Screen;
use gtk::{Builder, Window};
use gtk::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use server::*;
use sensor_index::*;
use notebook::*;
use std::collections::{HashMap, HashSet};
use sensor::*;

fn update_window(list: &gtk::ListStore, server: &Rc<RefCell<server::Server>>) {
    let mut server = server.borrow_mut();
    server.refresh_all();

    let mut sensors: HashMap<i32, &Sensor> = HashMap::new();
    let mut seen: HashSet<i32> = HashSet::new();

    for module in server.modules.iter() {
        for sensor in module.sensors.iter() {
            sensors.entry(sensor.modbus_slave_id).or_insert(sensor);
        }
    }

    if let Some(mut iter) = list.get_iter_first() {
        let mut valid = true;
        while valid {
            let modbus_slave_id = list.get_value(&iter, 0).get::<i32>().unwrap();
            if let Some(sensor) = sensors.get(&(modbus_slave_id)) {
                list.set(&iter,
                        &[0, 1, 2],
                        &[&sensor.modbus_slave_id, &"Sensor", &(sensor.adc_value as i32)]);
                // println!(">>{:?}", sensor.adc_value);
                valid = list.iter_next(&mut iter);
                seen.insert(modbus_slave_id);
            } else {
                valid = list.remove(&mut iter);
            }
        }
    }

    for (modbus_slave_id, sensor) in sensors.iter() {
        if !seen.contains(modbus_slave_id) {
            create_and_fill_model(list, sensor.modbus_slave_id as u32, &sensor.name, sensor.adc_value as u32);
        }
    }
}

fn window_setup(window: &gtk::Window) {
    // Window properties
    let window_title = "xMZ-Mod-Touch GUI ".to_string() + env!("CARGO_PKG_VERSION");
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(180.0);

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
    let mut notebook = notebook::Notebook::new();
    let sensor_index = SensorIndex::new(&srv, &mut notebook);
    let info_button = sensor_index.info_button.clone();

    window_setup(&window);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

    v_box.pack_start(&notebook.notebook, true, true, 0);

    window.add(&v_box);
    window.show_all();

    let list_store = sensor_index.list_store.clone();

    gtk::timeout_add(1000, move || {
        update_window(&list_store, &srv);

        glib::Continue(true)
    });

    gtk::main();
}
