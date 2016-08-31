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


/// Update den TreeStores mit den aktuellen Modul Daten des Servers
///
/// Der Module Vector wird im Timer Loop aktualisiert und dieser Funktion übergeben.
/// Sind mehr Module im Vector als Zeilen im aktullen TreeStore, dann werden Spalten mit der
/// Funktion `fill_treestore_column` angefügt.
/// Sind weniger Module im Vector (wurden zur Laufzeit gelöscht) dann wird der TreeStore
/// entsprechend verkleinert.
///
/// # Parameters
///
/// `treestore`     - TreeStore der die Daten hält
/// `modules`       - Vector mit Module Structs welches die Daten enthält
///
fn update_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
    // Das HashSet dient zur Erkennung ob mehr Module als TreeStore Zeilen vorhanden sind.
    let mut seen: HashSet<usize> = HashSet::new();
    // Auswahl der ersten Zeile des TreeStores
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
            // Existieren mehr Zeilen im TreeStore als Module im Vector vorhanden sind,
            // wird solange eine Zeile aus dem TreeStore entfernt
            // bis Anzahl Module und TreeStore Zeilen wieder gleich sind.
            } else {
                valid = treestore.remove(&mut iter);
            }
        }
    }
    // Alle Module die jetzt noch existieren, noch nicht gesehen also verarbeitet wurden,
    // wird eine neue Zeile im TreeStore angelegt und die Daten des Modules eingefügt.
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
/// FIXME: Auslagern in Glade!
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
fn get_modules(client: &mut Client) -> Result<Vec<Module>> {
    let mut modules: Vec<Module> = vec![];
    let _ = client.execute("module list").map(|data| {
        modules = json::decode(&data).unwrap_or(vec![]);
    });
    Ok(modules)
}

/// Module Index
/// Diese Funktion wird vom Module (mod.rs) als Erste Funktion aufgerufen. Hier werden die
/// Komponenten des Fensters aus dem Builder File eingebunden, der TreeStore für die Module
/// und Sensoren gebildet.
/// **Hier ist auch der Timer definiert der (im Sekundentakt) die Module aktualisiert.**
pub fn setup(builder: &Builder, window: &Window, client: &mut Client) -> Result<()> {
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

    let modules = try!(get_modules(client).chain_err(|| "Module konnten nicht vom Server abgefragt werden"));

    fill_treestore(&treestore_modules, &modules);
    setup_treeview(&treeview_modules);


    // Timer gesteuertes Update des TreeStore
    let treestore1 = treestore_modules.clone();
    let window1 = window.clone();
    let treeview1 = treeview_modules.clone();
    ::gtk::timeout_add(1000, move || {
        let mut client = Client::new();
        match get_modules(&mut client) {
            Ok(..) => {
                update_treestore(&treestore1, &modules);
                treeview1.expand_all();
            }
            Err(_) => {}
        }

        window1.queue_draw();

        ::glib::Continue(true)
    });

    Ok(())
}
