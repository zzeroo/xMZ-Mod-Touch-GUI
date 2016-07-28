extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;

use gdk::Screen;
use gtk::{ Builder, CellRendererText, TreeStore, TreeView, TreeViewColumn, Window };
use gtk::prelude::*;
use rustc_serialize::json;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::rc::Rc;
use xmz_client::client::Client;
use xmz_server::module::{Module, ModuleType};
use xmz_server::sensor::Sensor;

fn update_window(store: &gtk::TreeStore, client: &mut Client) {
    // Ugly hack, but agile like I
    store.clear();

    &client.execute("module list").map(|data| {
        json::decode(&data).map(|modules: Vec<Module>| {
            for module in modules {
                let module_iter = store.insert_with_values(None, None, &[1],
                                                &[&format!("{:?} - {}", module.modbus_slave_id(), module.module_type())]);

                for sensor in module.sensors {
                    store.insert_with_values(Some(&module_iter), None, &[1],
                                                &[&format!("{:.02} {}", sensor.concentration().unwrap_or(0.0), sensor.si())]);
                }
            }
        });
    });
}

fn window_setup(window: &gtk::Window) {
    // Window basic properties
    let window_title = format!("{} {}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

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

    let module_tree = TreeView::new();
    let module_column_types = [String::static_type(), String::static_type()];
    let module_store = TreeStore::new(&module_column_types);
    let col = TreeViewColumn::new();
    let text_renderer = CellRendererText::new();

    // TODO: Module Namen oder Beschreibung ist hier besser
    col.set_title("Modbus Adresse");
    col.pack_start(&text_renderer, true);
    col.add_attribute(&text_renderer, "text", 1);
    module_tree.append_column(&col);

    module_tree.set_model(Some(&module_store));
    module_tree.set_headers_visible(true);

    let mut client = Client::new();
    update_window(&module_store, &mut client);


    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    v_box.add(&module_tree);
    window.add(&v_box);

    window.show_all();
    module_tree.expand_all();

    // Timer zum Update alle Sekunden
    gtk::timeout_add(2000, move || {
        update_window(&module_store, &mut client);
        module_tree.expand_all();

        glib::Continue(true)
    });

    gtk::main();
}
