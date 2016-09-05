use common::*;
use errors::*;
use gtk::{Builder, Button, ComboBox, TreeView, TreeStore, Window};
use gtk::prelude::*;
use rustc_serialize::json;
use xmz_client::client::Client;
use std::sync::Arc;
use gui::gtk3::module_index;
use xmz_server::module::Module;


/// Füllt den TreeStore mit den Daten der Module
///
/// # Params
///
/// `treestore` - TreeStore der die Daten hält
/// `modules`   - Vector mit Modulen
///
///
///
pub fn fill_treestore(treestore: &TreeStore, modules: &Vec<Module>) {
    for (id, module) in modules.iter().enumerate() {
        fill_treestore_column(&treestore, &module, id);
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

pub fn save_server_settings_interface(builder: &Builder) {
    // Dieses Array wird am Ende der Funktion serialisiert und an den Server übertragen,
    // dort wird es dann deserialisiert und in die Server Structur eingebunden.
    let mut server_settings_interface: Vec<String> = vec![];

    let combobox_text_modbus_device: ComboBox = builder.get_object("combobox_text_modbus_device").unwrap();
    let combobox_text_baud: ComboBox = builder.get_object("combobox_text_baud").unwrap();
    let combobox_text_data_bits: ComboBox = builder.get_object("combobox_text_data_bits").unwrap();
    let combobox_text_parity: ComboBox = builder.get_object("combobox_text_parity").unwrap();
    let combobox_text_stop_bits: ComboBox = builder.get_object("combobox_text_stop_bits").unwrap();

    let modbus_device = combobox_text_modbus_device.get_active_id().unwrap();
    let baud = combobox_text_baud.get_active_id().unwrap();
    let data_bits = combobox_text_data_bits.get_active_id().unwrap();
    let parity = combobox_text_parity.get_active_id().unwrap();
    let stop_bits = combobox_text_stop_bits.get_active_id().unwrap();

    server_settings_interface.push(modbus_device);
    server_settings_interface.push(baud);
    server_settings_interface.push(data_bits);
    server_settings_interface.push(parity);
    server_settings_interface.push(stop_bits);

    let mut client = Client::new();
    // TODO: Eigene Fehlercodes einführen
    let _ = client.execute(format!("server set interface_config {}", json::encode(&server_settings_interface).unwrap()));

}

pub fn setup(builder: &Builder, window: &Window, client: &mut Client) -> Result<()> {
    let button_server_settings_interface_save: Button = builder.get_object("button_server_settings_interface_save").unwrap();
    let button_reset_client_error_communication: Button = builder.get_object("button_reset_client_error_communication").unwrap();
    let treeview_settings_module_left: TreeView = builder.get_object("treeview_settings_module_left").unwrap();
    let treestore_modules: TreeStore = builder.get_object("treestore_modules").unwrap();
    treeview_settings_module_left.set_model(Some(&treestore_modules));

    let mut client = client.clone();
    if client.error_communication < 5 {
        match module_index::get_modules(&mut client) {
            Ok(modules) => {
                module_index::fill_treestore(&treestore_modules, &modules);
            }
            Err(err) => {
                if client.error_communication < u32::max_value() {
                    client.error_communication += 1;
                }
                report_error(&err);
            }
        }
    }


    let builder1 = builder.clone();
    button_server_settings_interface_save.connect_clicked(move |_| {
        save_server_settings_interface(&builder1);
    });

    button_reset_client_error_communication.connect_clicked(move |_| {
    });

    let treeview_settings_module_left = treeview_settings_module_left.clone();
    treeview_settings_module_left.connect_row_activated(move |_, _, _| {
        println!("Row activated");
    });

    Ok(())
}
