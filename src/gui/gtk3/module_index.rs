extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;

use gtk::{Builder, CellRendererText, InfoBar, ScrolledWindow, TreeView, TreeViewColumn, TreeStore, Type};
use self::rustc_serialize::json;
use self::xmz_client::client::Client;
use self::xmz_server::module::{Module, ModuleType};
use self::xmz_server::sensor::Sensor;

// Module Index
pub fn setup(builder: &Builder, client: &mut Client) {
    let modules_tree: TreeView = builder.get_object("modules_treeview").unwrap();
    let scroll: ScrolledWindow = builder.get_object("scroll_module_index").unwrap();
    let info_bar: InfoBar      = builder.get_object("info_bar").unwrap();

    // modbus_slave_id, ModuleType, SensorType, concentration, SI
    let modules_store = TreeStore::new(&[Type::String, Type::String, Type::String, Type::String, Type::String, ]);

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

    add_column!(modules_store, "Modbus Slave", 0);
    add_column!(modules_store, "Module Typ", 1);
    add_column!(modules_store, "Sensor Typ", 2);
    add_column!(modules_store, "Konzentration", 3);
    add_column!(modules_store, "SI", 4);
    modules_tree.set_model(Some(&modules_store));

    client.execute("module list").map(|data| {
        json::decode(&data).map(|modules: Vec<Module>| {

            for module in modules {
                let module_iter = modules_store.insert_with_values(None, None, &[0, 1], &[&module.modbus_slave_id(), &module.module_type()]);

                for sensor in module.sensors {
                    let sensor_iter = modules_store.insert_with_values(Some(&module_iter), None, &[2, 3, 4],
                                                                        &[  &sensor.sensor_type(),
                                                                            &format!("{:.02}", &sensor.concentration().unwrap_or(0.0)),
                                                                            &sensor.si(),
                                                                        ]);
                }
            }
        });
    });
}
