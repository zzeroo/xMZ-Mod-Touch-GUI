extern crate xmz_server;
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{CellRendererText, Orientation, TreeView, TreeStore, TreeViewColumn, Window, WindowType,
          WindowPosition};
use gdk::enums::key;
use xmz_server::module::{Module, ModuleType};


fn test_treestore(list: &TreeStore) -> i32 {
    let mut num: i32 = 0;

    if let Some(_iter) = list.get_iter_first() {
        num = list.iter_n_children(None);
        println!("Nummer Children: {:?}", num);
    }
    num
}

fn iter_treestore(treestore: &TreeStore) {
    // Liefert den ersten Iter des TreeStores (store),
    if let Some(mut _iter) = treestore.get_iter_first() {
    }
}

fn update_window(treestore: &TreeStore) {
    let module = make_module(6);

    // fn get_iter_first(&self) -> Option<TreeIter>
    if let Some(mut iter) = treestore.get_iter_first() {
        let mut _valid = true;
        while _valid {
            let id = treestore.get_value(&iter, 0).get::<i64>().unwrap_or(0) as usize;
            println!("Id: {}", id);
            if let Some(m) = module.get(id) {
                treestore.set(&iter, &[1, 2], &[&m.modbus_slave_id(), &m.module_type()]);
                _valid = treestore.iter_next(&mut iter);
            } else {
                _valid = treestore.remove(&mut iter);
            }
            _valid = false;
        }
    }
}

// Füll ein TreeStore mit den übergebenen Modulen
fn populate_model(modules: &Vec<Module>) -> TreeStore {
    // ModuleID, Modbus Slave Id, ModuleType,
    let model = TreeStore::new(&[u32::static_type(),
                                 String::static_type(),
                                 String::static_type(),
                                 // SensorType, Konzentration, SI Einheit
                                 String::static_type(),
                                 String::static_type(),
                                 String::static_type()]);

    for (i, module) in modules.iter().enumerate() {
        let module_iter = model.insert_with_values(None,
                                                   None,
                                                   &[0, 1, 2],
                                                   &[&(i as u32 + 1),
                                                     &module.modbus_slave_id(),
                                                     &module.module_type()]);
        for (i, sensor) in module.sensors.iter().enumerate() {
            model.insert_with_values(Some(&module_iter),
                                     None,
                                     &[0, 2, 3, 4],
                                     &[&(i as u32 + 1),
                                       &sensor.sensor_type(),
                                       &format!("{:.02}", sensor.concentration().unwrap_or(0.0)),
                                       &sensor.si()]);
        }
    }
    model
}


// Erzeugt ein TreeView mit 6 Text Spalten, mit nicht sichtbaren Spaltenköpfen
fn create_and_setup_treeview() -> TreeView {
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

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Verbinde die `id` Spalte des Views mit der `id` Spalte des Models
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

// Erstellt 10 Module
fn default_module() -> Vec<Module> {
    let mut ret_val = vec![];

    for _ in 1..11 {
        ret_val.push(Module::new(ModuleType::RAGAS_CO_NO2));
    }

    ret_val
}

// Erstellt eine beliebige Nummer an Modulen
//
// # Parameters
// `num`    - Anzahl der zu erstellenden Module
//
fn make_module(num: i32) -> Vec<Module> {
    let mut ret_val = vec![];
    for _ in 1..num {
        ret_val.push(Module::new(ModuleType::RAGAS_CO_NO2));
    }
    ret_val
}

// Basis Fenster erstellen und einigen Eigenschaften und Attribute einrichten.
fn create_and_setup_window() -> Window {
    let window = Window::new(WindowType::Toplevel);

    window.set_title("TreeView Tests");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    window
}


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialise Gtk3.");
        return;
    }
    // Basis Fenster Erstellen. Hier wird das Fenster und einige Attribute und Signale erzeugt und
    // miteinander verbunden.
    let window = create_and_setup_window();
    // Verticales Layout für den TreeView und evtl. weitere Widgets
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);
    // Erzeugt ein TreeView mit 6 Text Spalten, mit nicht sichtbaren Spaltenköpfen
    let treeview = create_and_setup_treeview();
    // 10 Module erzeugen
    let modules = default_module();
    // Füllt die Module in ein TreeStore der in der Funktion erzeugt wird
    let model = populate_model(&modules);

    update_window(&model);
    iter_treestore(&model);
    test_treestore(&model);

    // Set Model im View
    treeview.set_model(Some(&model));

    // Tree in gtk::Box packen
    vertical_layout.add(&treeview);
    // gtk::Box in Window Container packen
    window.add(&vertical_layout);

    // Alles anzeigen
    window.show_all();
    // und main loop starten.
    gtk::main();
}
