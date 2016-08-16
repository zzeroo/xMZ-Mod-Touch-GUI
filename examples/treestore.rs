
extern crate gtk;
extern crate gdk;
extern crate xmz_server;

use gtk::prelude::*;
use gtk::{Orientation, TreeStore, TreeView, TreeViewColumn, CellRendererText, Window,
          WindowPosition, WindowType};
use gdk::enums::key;
use xmz_server::sensor::{Sensor, SensorType};
use xmz_server::module::{Module, ModuleType};


// Beispiel Daten 10 Module, 20 Sensoren
//
fn create_module() -> Vec<Module> {
    let mut modules: Vec<Module> = vec![];

    for _ in 1..11 {
        let sensor1 = Sensor::new(SensorType::NemotoNO2);
        let sensor2 = Sensor::new(SensorType::NemotoCO);
        let mut module = Module::new(ModuleType::RAGAS_CO_NO2);
        module.sensors.push(sensor1);
        module.sensors.push(sensor2);
        modules.push(module);
    }
    // Return value
    modules
}

// Helper Funktion zum Anfügen weiterer Spalten (columns)
fn append_column(treeview: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Die Daten und das View werden über `id` Spalte des Models und
    // über die `id` Spalte des Stores verbunden.
    column.add_attribute(&cell, "text", id);
    // Div. Attribute
    // column.set_title(??)
    column.set_resizable(false);
    column.set_clickable(false);
    treeview.append_column(&column);
}

// Erzeuge ein TreeView
// Hier wird festgelegt wie die Daten Dargestellt werden sollen.
// Einzelne Spalten können angezeigt werden, nicht angezeigt werden,
// doppelt angezeigt werden und so weiter. Der Fantasie sind keine Grenzen gesetzt.
fn create_and_setup_treeview() -> TreeView {
    let treeview = TreeView::new();

    // Attribute des TreeViews
    treeview.set_headers_visible(false);

    append_column(&treeview, 0); // Module::Module ID
    append_column(&treeview, 1); // Module::Modbus Slave Id
    append_column(&treeview, 2); // Module::ModuleType
    append_column(&treeview, 3); // Sensor::SensorType
    append_column(&treeview, 4); // Sensor::Konzentration
    append_column(&treeview, 5); // Sensor::SI Einheit

    treeview
}

fn create_and_fill_treestore(module: &Vec<Module>) -> TreeStore {
    let treestore = TreeStore::new(&[u32::static_type(), // Module::Module ID
                                     String::static_type(), // Module::Modbus Slave ID
                                     String::static_type(), // Module::ModuleType
                                     String::static_type(), // Sensor::SensorType
                                     String::static_type(), // Sensor::Konzentration
                                     String::static_type() /* Sensor::SI Einheit */]);

    for (i, module) in module.iter().enumerate() {
        let module_iter = treestore.insert_with_values(None,
                                                       None,
                                                       &[0, 1, 2],
                                                       &[&(i as u32 + 1),
                                                         &module.modbus_slave_id(),
                                                         &module.module_type()]);
        for (i, sensor) in module.sensors.iter().enumerate() {
            treestore.insert_with_values(Some(&module_iter),
                                         None,
                                         &[0, 2, 3, 4],
                                         &[&(i as u32 + 1),
                                           &sensor.sensor_type(),
                                           &format!("{:.02}",
                                                    sensor.concentration().unwrap_or(0.0)),
                                           &sensor.si()]);
        }
    }

    treestore
}

// Basis Fenster erstellen und einige Attribute und Funktionen einrichten.
fn create_and_setup_window() -> Window {
    let window = Window::new(WindowType::Toplevel);

    window.set_title("TreeStore Test");
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
    // Gtk Zeug
    if gtk::init().is_err() {
        println!("Gtk3 konnte nicht initalisisert werden.");
        return;
    }
    // Basis Fenster erstellen
    let window = create_and_setup_window();
    // Verticales Layout für den TreeView, am unteren Ende soll evtl.
    // ein Button zum editieren angelegt werden.
    let vertical_layout = gtk::Box::new(Orientation::Vertical, 0);
    // Erzeuge ein neues TreeView
    let treeview = create_and_setup_treeview();
    // Beispieldaten
    let module = create_module();
    // TreeStore
    let treestore = create_and_fill_treestore(&module);

    // Verbinde View und Model (TreeStore)
    treeview.set_model(Some(&treestore));

    // TreeView in die Box packen
    vertical_layout.add(&treeview);
    // Box in Fenster packen
    window.add(&vertical_layout);

    // Alles anzeigen
    window.show_all();
    // Gtk Main Loop starten
    gtk::main();
}
