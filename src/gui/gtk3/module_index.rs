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
    let modules_tree: TreeView = builder.get_object("modules_treeview").unwrap();
    // TODO: WAS IST DAS? ->let scroll: ScrolledWindow = builder.get_object("scroll_module_index").unwrap();
    let info_bar: InfoBar = builder.get_object("info_bar").unwrap();

    // modbus_slave_id, ModuleType, SensorType, concentration, SI
    let treeview =
        TreeStore::new(&[Type::String, Type::String, Type::String, Type::String, Type::String]);

    // A simple macro for adding a column to the preview tree.
    macro_rules! add_column {
        ($modules_tree:ident, $title:expr, $id:expr) => {{
            let column  = TreeViewColumn::new();
            let renderer = CellRendererText::new();
            column.set_title($title);
            column.set_resizable(false);
            column.pack_start(&renderer, true);
            column.add_attribute(&renderer, "text", $id);
            modules_tree.append_column(&column);
        }}
    }

    add_column!(treeview, "Modbus Slave", 0);
    add_column!(treeview, "Module Typ", 1);
    add_column!(treeview, "Sensor Typ", 2);
    add_column!(treeview, "Konzentration", 3);
    add_column!(treeview, "SI", 4);
    modules_tree.set_model(Some(&treeview));

    let _ = client.execute("module list").map(|data| {
        let _ = json::decode(&data).map(|modules: Vec<Module>| {

            for module in modules {
                let module_iter = treeview.insert_with_values(None,
                                                              None,
                                                              &[0, 1],
                                                              &[&module.modbus_slave_id(),
                                                                &module.module_type()]);

                for sensor in module.sensors {
                    let _sensor_iter = treeview.insert_with_values(Some(&module_iter),
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
