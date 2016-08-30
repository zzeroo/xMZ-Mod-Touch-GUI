/// Diese Datei representiert den Module Index View, die Übersichtsseite mit den Modulen und
/// deren Sensoren.
use errors::*;
use gtk::prelude::*;
use gtk::{Builder, CellRendererText, TreeView, TreeViewColumn, TreeStore,
          Window};
use rustc_serialize::json;
use xmz_client::client::Client;
use xmz_server::module::Module;
use std::collections::HashSet;


/// Update des TreeStores mit den aktuellen Modul Daten des Servers
fn update_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
    let mut seen: HashSet<usize> = HashSet::new();

    if let Some(mut iter) = treestore.get_iter_first() {
        let mut valid = true;
        while valid {
            // ID ist Nummer der Spalte, von 1 gezält
            let id = treestore.get_value(&iter, 0).get::<u32>().unwrap() as usize;
            if let Some(module) = modules.get(id - 1) {
                treestore.set(&iter,
                              &[0, 1, 2],
                              &[&(id as u32), &module.modbus_slave_id(), &module.module_type()]);
                  // Durchläuft jeden Sensor der im Modul konfiguriert wurde und setzt die Felder
                  for (i, sensor) in module.sensors.iter().enumerate() {
                      treestore.set(&treestore.iter_nth_child(Some(&iter), i as i32).unwrap(),
                                   &[0, 2, 3, 4],
                                   &[&(i as u32 + 1),
                                     &sensor.sensor_type(),
                                     &format!("{:.02}", sensor.concentration().unwrap_or(0.0)),
                                     &sensor.si()]);
                  }

                valid = treestore.iter_next(&mut iter);
                seen.insert(id);
            } else {
                valid = treestore.remove(&mut iter);
            }
        }
    }

    for (id, module) in modules.iter().enumerate() {
        if !seen.contains(&(id + 1)) {
            fill_treestore_column(treestore, &module, id);
        }
    }
}

/// Helper Funktion die den TreeStore nachträglich um eine weitere Zeile `id` (3. Parameter)
/// mit den Daten des als 2. Parameter übergebenen `module` füllt.
///
/// #Parameters
///
/// `treestore`     - TreeStore der die Daten hält
/// `module`        - Module Struct die die neuen Daten enthält
/// `id`            - ID wo im TreeStore die Daten eingefügt werden sollen
///
fn fill_treestore_column(treestore: &TreeStore, module: &Module, id: usize) {
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

/// Helper Funktion zum Anfügen weiterer Spalten (columns) in einen TreeView
///
/// # Params
///
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

/// Basis Setup des TreeViews
///
/// FIXME: Auslagern in Glade!
fn setup_treeview(treeview: &TreeView) {
    // Header verstecken
    treeview.set_headers_visible(false);

    append_column(&treeview, 1);
    append_column(&treeview, 2);
    append_column(&treeview, 3);
    append_column(&treeview, 4);
    append_column(&treeview, 5);

    // Alle Spalten aufklappen, so das die Sensoren sichtbar sind.
    treeview.expand_all();
}

/// Füllt den TreeStore mit den Daten der Module
///
/// # Params
///
/// `treestore` - TreeStore der die Daten hält
/// `modules`   - Vector mit Modulen
///
///
///
fn fill_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
    for (id, module) in modules.iter().enumerate() {
        fill_treestore_column(&treestore, &module, id);
    }
}

/// Frage den Server, über den Client, nach den aktuellen Module Daten ab.
///
/// Liefert ein Vector mit Modules zurück.
///
fn get_modules(client: &mut Client) -> Vec<Module> {
    let mut modules: Vec<Module> = vec![];
    let _ = client.execute("module list").map(|data| {
        modules = json::decode(&data).unwrap_or(vec![]);
    });
    modules
}

/// Module Index
/// Diese Funktion wird vom Module (mod.rs) als Erste Funktion aufgerufen. Hier werden die
/// Komponenten des Fensters aus dem Builder File eingebunden, der TreeStore für die Module
/// und Sensoren gebildet.
pub fn setup(builder: &Builder, window: &Window, client: &mut Client) {
    let treeview_modules: TreeView = builder.get_object("treeview_modules").unwrap();

    // FIXME: TreeStore aus dem Glade erzeugt Fehler in append_column()
    // `column.add_attribute(&cell, "text", id);`
    // let treestore_modules: TreeStore = builder.get_object("treestore_modules")
    //     .expect("TreeStore konnte nicht aus dem Builder File geladen werden.");
    // FIXME: Manuell erzeugter TreeStore, auch OK oder ><
    let treestore_modules = TreeStore::new(&[u32::static_type(), // Module::Module ID
                                     String::static_type(), // Module::Modbus Slave ID
                                     String::static_type(), // Module::ModuleType
                                     String::static_type(), // Sensor::SensorType
                                     String::static_type(), // Sensor::Konzentration
                                     String::static_type() /* Sensor::SI Einheit */]);

     // Verbinde View und Model (TreeStore)
     // Das ist nur nötig, wenn der TreeStore nicht aus dem Glade File kommt
     treeview_modules.set_model(Some(&treestore_modules));

    let modules = get_modules(client);

    fill_treestore(&treestore_modules, &modules);
    setup_treeview(&treeview_modules);


    // Update TreeStore
    let treestore1 = treestore_modules.clone();
    let window1 = window.clone();
    let treeview1 = treeview_modules.clone();
    ::gtk::timeout_add(1000, move || {
        let mut client = Client::new();
        let modules = get_modules(&mut client);
        update_treestore(&treestore1, &modules);
        treeview1.expand_all();

        window1.queue_draw();

        ::glib::Continue(true)
    });
}

// /// Module Index
// 167:pub fn setup(builder: &Builder, window: &Window, client: &mut Client) {
// - ruft get_modules()
// - ruft fill_treestore()
// - ruft setup_treeview()
// - ruft update_treestore() im Timer auf
//     /// Frage den Server, über den Client, nach den aktuellen Module Daten ab.
//     155:fn get_modules(client: &mut Client) -> Vec<Module> {
//     /// Füllt den TreeStore mit den Daten der Module
//     126:fn fill_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
//     - ruft fill_treestore_column() für jedes Module im modules Vector auf
//         // Helper Funktion die den TreeStore nachträglich im eine Spalte mit den Daten des `module` füllt
//         60:fn fill_treestore_column(treestore: &TreeStore, module: &Module, id: usize) {
//
//     /// Basis Setup des TreeViews
//     103:fn setup_treeview(treeview: &TreeView) {
//     - ruft append_column() für jede Spate des TreeViews auf
//         /// Helper Funktion zum Anfügen weiterer Spalten (columns) in einen TreeView
//         85:fn append_column(treeview: &TreeView, id: i32) {
//
//     /// Update des TreeStores mit den aktuellen Modul Daten des Servers
//     14:fn update_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
//
//
// /// Helper Funktion die den TreeStore nachträglich um eine weitere Zeile `id` (3. Parameter)
// /// mit den Daten des als 2. Parameter übergebenen `module` füllt.
// 60:fn fill_treestore_column(treestore: &TreeStore, module: &Module, id: usize) {
