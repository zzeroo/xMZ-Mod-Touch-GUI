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


/// Module Index
/// Diese Funktion wird vom Module (mod.rs) als Erste Funktion aufgerufen. Hier werden die
/// Komponenten des Fensters aus dem Builder File eingebunden, der TreeStore für die Module
/// und Sensoren gebildet.
pub fn setup(builder: &Builder, client: &mut Client) {
    let info_bar: InfoBar = builder.get_object("info_bar").unwrap();
    let treeview_modules: TreeView = builder.get_object("treeview_modules").unwrap();
    let treestore_modules: TreeStore = builder.get_object("treestore_modules").expect("TreeStore konnte nicht aus dem Builder File geladen werden.");


    let _ = client.execute("module list").map(|data| {
        let _ = json::decode(&data).map(|modules: Vec<Module>| {

            for module in modules {
                let module_iter = treestore_modules.insert_with_values(None,
                                                              None,
                                                              &[0, 1],
                                                              &[&module.modbus_slave_id(),
                                                                &module.module_type()]);

                for sensor in module.sensors {
                    let _sensor_iter = treestore_modules.insert_with_values(Some(&module_iter),
                                                                  None,
                                                                  &[2, 3, 4],
                                                                  &[&sensor.sensor_type(),
                                                                    &format!("{:.02}",
                                                                    &sensor.concentration().unwrap_or(0.0)),
                                                                    &sensor.si()]);
                }
            }
        });
    });
}
