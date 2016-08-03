extern crate xmz_server;
extern crate xmz_client;
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{
    CellRendererText, Orientation, TreeView, TreeStore, TreeViewColumn,
    Window, WindowType, WindowPosition,
};
use gdk::enums::key;
use xmz_server::module::{Module, ModuleType};
use xmz_server::sensor::{Sensor, SensorType};
use xmz_client::client::Client;


fn create_and_fill_model(modules: &Vec<Module>) -> TreeStore {
    // Modbus Slave Id, ModuleType, SensorType, Konzentration, SI Einheit
    let model = TreeStore::new(&[u32::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), ]);

    for (i, module) in modules.iter().enumerate() {
        model.insert_with_values(None, None, &[0, 1, 2], &[&(i as u32 + 1), &module.modbus_slave_id(), &module.module_type()]);
    }
    model
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Verbinde die `id` Spalte des Views mit der `id` Spalte des Models
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_setup_view() -> TreeView {
    let tree = TreeView::new();

    tree.set_headers_visible(false);
    append_column(&tree, 0);
    append_column(&tree, 1);
    append_column(&tree, 2);
    tree
}

fn create_module_default() -> Vec<Module> {
    let mut ret_val = vec![];

    for _ in (1..11) {
        let mut module = Module::new(ModuleType::RAGAS_CO_NO2);
        let sensor1 = Sensor::new(SensorType::NemotoNO2);
        let sensor2 = Sensor::new(SensorType::NemotoCO);
        module.sensors.push(sensor1); module.sensors.push(sensor2);
        ret_val.push(module);
    }

    ret_val
}


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialise Gtk3.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("TreeView Tests");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() { gtk::main_quit() }
        Inhibit(false)
    });

    // Verticales Layout für den TreeView und evtl. weitere Widgets
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);

    let tree = create_and_setup_view();

    let modules = create_module_default();

    let model = create_and_fill_model(&modules);

    // Set Model im View
    tree.set_model(Some(&model));

    // Tree in gtk::Box packen
    vertical_layout.add(&tree);
    // gtk::Box in Window Container packen
    window.add(&vertical_layout);

    window.show_all();
    gtk::main();
}
