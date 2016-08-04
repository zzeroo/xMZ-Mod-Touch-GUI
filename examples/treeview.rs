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


fn update_window(list: &TreeStore) {
    let module = default_module();

    if let Some(mut iter) = list.get_iter_first() {
        let mut valid = true;
        while valid {
            let id = list.get_value(&iter, 0).get::<i64>().unwrap_or(0) as usize;
            println!("{}", id);
            if let Some(m) = module.get(id) {
                list.set(   &iter,
                            &[1, 2],
                            &[&m.modbus_slave_id(), &m.module_type()]);
                valid = list.iter_next(&mut iter);
            } else {
                valid = list.remove(&mut iter);
            }
            valid = false;
        }
    }
}


fn create_and_fill_model(modules: &Vec<Module>) -> TreeStore {
    //                            ModuleID,           Modbus Slave Id,        ModuleType,
    let model = TreeStore::new(&[ u32::static_type(), String::static_type(), String::static_type(),
    //                            SensorType,            Konzentration,          SI Einheit
                                  String::static_type(), String::static_type(), String::static_type(), ]);

    for (i, module) in modules.iter().enumerate() {
        let module_iter = model.insert_with_values(None, None, &[0, 1, 2], &[&(i as u32 + 1), &module.modbus_slave_id(), &module.module_type()]);
        for (i, sensor) in module.sensors.iter().enumerate() {
            model.insert_with_values(Some(&module_iter), None, &[0, 2, 3, 4], &[ &(i as u32 + 1),
                                                                                 &sensor.sensor_type(),
                                                                                 &format!("{:.02}", sensor.concentration().unwrap_or(0.0)),
                                                                                 &sensor.si(),
                                                                              ]);
        }
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

    append_column(&tree, 3);
    append_column(&tree, 4);
    append_column(&tree, 5);
    tree
}

fn default_module() -> Vec<Module> {
    let mut ret_val = vec![];

    for _ in 1..11 {
        ret_val.push(Module::new(ModuleType::RAGAS_CO_NO2));
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

    let modules = default_module();

    let model = create_and_fill_model(&modules);

    update_window(&model);

    // Set Model im View
    tree.set_model(Some(&model));

    // Tree in gtk::Box packen
    vertical_layout.add(&tree);
    // gtk::Box in Window Container packen
    window.add(&vertical_layout);

    window.show_all();
    gtk::main();
}
