use gtk::prelude::*;
use gtk::{Builder, Button, ComboBox};
use rustc_serialize::json;
use xmz_client::client::Client;


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

pub fn setup(builder: &Builder) {
    let button_server_settings_interface_save: Button = builder.get_object("button_server_settings_interface_save").unwrap();

    let builder1 = builder.clone();
    button_server_settings_interface_save.connect_clicked(move |_| {
        save_server_settings_interface(&builder1);
    });
}
