/// Diese Datei representiert den Module Index View, die Übersichtsseite mit den Modulen und
/// deren Sensoren.
extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;

use gtk::{Builder, CellRendererText, InfoBar, ScrolledWindow, TreeView, TreeViewColumn, TreeStore,
          Type};
use self::rustc_serialize::json;
use self::xmz_client::client::Client;
use self::xmz_server::module::Module;


/// Helper Funktion zum Anfügen weiterer Spalten (columns)
///
/// # Params
/// `treeview`  - Der TreeView mit dem gearbeitet werden soll
/// `id`        - ID der Spalte, Null basiert
///
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


fn setup_treeview(treeview: &TreeView) {
    // Header verstecken
    treeview.set_headers_visible(false);

    append_column(&treeview, 1);
    append_column(&treeview, 2);
    append_column(&treeview, 3);
    append_column(&treeview, 4);
    append_column(&treeview, 5);
}


fn fill_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
    for (id, module) in modules.iter().enumerate() {
        create_and_fill_model(&treestore, &module, id);
    }
}

// Helper Funktion die den TreeStore nachträglich im eine Spalte mit den Daten des `module` füllt
fn create_and_fill_model(treestore: &TreeStore, module: &Module, id: usize) {
    let module_iter = treestore.insert_with_values(None,
                                                   None,
                                                   &[0, 1, 2],
                                                   &[&(id as u32 + 1),
                                                     &module.modbus_slave_id(),
                                                     &module.module_type()]);
    for (i, sensor) in module.sensors.iter().enumerate() {
        treestore.insert_with_values(Some(&module_iter),
                                     None,
                                     &[0, 2, 3, 4],
                                     &[&(i as u32 + 1),
                                       &sensor.sensor_type(),
                                       &format!("{:.02}", sensor.concentration().unwrap_or(0.0)),
                                       &sensor.si()]);
    }
}
/// Module Index
/// Diese Funktion wird vom Module (mod.rs) als Erste Funktion aufgerufen. Hier werden die
/// Komponenten des Fensters aus dem Builder File eingebunden, der TreeStore für die Module
/// und Sensoren gebildet.
pub fn setup(builder: &Builder, client: &mut Client) {
    let info_bar: InfoBar = builder.get_object("info_bar").unwrap();
    let treeview_modules: TreeView = builder.get_object("treeview_modules").unwrap();
    let treestore_modules: TreeStore = builder.get_object("treestore_modules")
        .expect("TreeStore konnte nicht aus dem Builder File geladen werden.");


    // Frage den Server, über den Client, nach den aktuellen Module Daten
    let mut modules: Vec<Module> = vec![];
    let _ = client.execute("module list").map(|data| {
        modules = json::decode(&data).unwrap_or(vec![]);
    });

    fill_treestore(&treestore_modules, &modules);
    setup_treeview(&treeview_modules);
}
