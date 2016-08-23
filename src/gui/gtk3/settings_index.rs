use gtk::prelude::*;
use gtk::{Builder, Button, ComboBox};
use rustc_serialize::json;

pub fn save_server_settings(builder: &Builder) {
    // Dieses Array wird am Ende der Funktion serialisiert und an den Server übertragen,
    // dort wird es dann deserialisiert und in die Server Structur eingebunden.
    let mut server_settings: Vec<String> = vec![];

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

    server_settings.push(modbus_device);
    server_settings.push(baud);
    server_settings.push(data_bits);
    server_settings.push(parity);
    server_settings.push(stop_bits);

    println!("{:#?}", json::encode(&server_settings));
    println!("{:#?}", json::decode::<Vec<String>>(&json::encode(&server_settings).unwrap()));

}

pub fn setup(builder: &Builder) {
    let button_server_settings_save: Button = builder.get_object("button_server_settings_save").unwrap();

    let builder1 = builder.clone();
    button_server_settings_save.connect_clicked(move |_| {
        save_server_settings(&builder1);
    });
}
