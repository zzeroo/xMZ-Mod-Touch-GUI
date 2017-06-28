use api_client::ApiClient;
use errors::*;
use gdk::{DisplayExt, ScreenExt};
use gdk::enums::key;
use gtk;
use gtk::{
    CellRendererText, Orientation, ScrolledWindow,
    TreeView, TreeStore, TreeViewColumn, TreeIter,
    Window, WindowType, WindowPosition
};
use gtk::prelude::*;
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use xmz_mod_touch_server::{Kombisensor, Sensor, Zone};


// Helper function, to contruct the model>view join
fn append_text_column(treeview: &TreeView, title: Option<&str>, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    if let Some(title) = title { column.set_title(&title); }
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", id);
    treeview.append_column(&column);
}

fn create_and_fill_model_zone(treestore: &TreeStore, zone_id: usize) -> TreeIter {
    treestore.insert_with_values(
        None,
        None,
        &[0],
        &[
            &format!("{}", zone_id),
        ]
    )
}
fn create_and_fill_model_kombisensor(treestore: &TreeStore, iter_zones: &TreeIter, kombisensor: &Kombisensor, kombisensor_id: usize) -> TreeIter {
    treestore.insert_with_values(
        Some(&iter_zones),
        None,
        &[0, 1, 2],
        &[
            &format!("{}", kombisensor_id),
            &format!("{}", kombisensor.get_kombisensor_type()),
            &format!("{}", kombisensor.get_modbus_address()),
        ]
    )
}
fn create_and_fill_model_sensor(treestore: &TreeStore, iter_kombisensors: &TreeIter, sensor: &Sensor, sensor_id: usize) -> TreeIter {
    treestore.insert_with_values(
        Some(&iter_kombisensors),
        None,
        &[0, 1, 3, 4, 5],
        &[
            &format!("{}", sensor_id),
            &format!("{}", sensor.get_sensor_type()),
            &format!("{:.02}", sensor.get_concentration()),
            &format!("{:.02}", sensor.get_concentration_average_15min()),
            &format!("{}", sensor.get_si()),
        ]
    )
}
// Nur wenn Zonen im Server sind, hat diese Funktion ein effekt
fn treestore_fill(client: &Arc<Mutex<ApiClient>>, treestore: &TreeStore, treeview: &TreeView) {
    if let Ok(client) = client.try_lock() {
        for (zone_id, zone) in client.get_server_data().get_zones().iter().enumerate() {
            let iter_zones = create_and_fill_model_zone(treestore, zone_id);

            for (kombisensor_id, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                let iter_kombisensors = create_and_fill_model_kombisensor(treestore, &iter_zones, &kombisensor, kombisensor_id);

                for (sensor_id, sensor) in kombisensor.get_sensors().iter().enumerate() {
                    let iter_sensors = create_and_fill_model_sensor(treestore, &iter_kombisensors, &sensor, sensor_id);
                }
            }

            treeview.set_model(Some(treestore));
            treeview.expand_all();
        }
    }
}

fn update_client(client: &Arc<Mutex<ApiClient>>) -> Result<()> {
    if let Ok(mut client) = client.try_lock() {
        client.update_server_data();
    } else {
        bail!("Server::try_lock() failed...")
    }

    Ok(())
}

fn treestore_update(client: &Arc<Mutex<ApiClient>>, treestore: &TreeStore, treeview: &TreeView) {
    if let Ok(client) = client.try_lock() {
        // HashSet<Zone Vector Index>
        let mut seen_zones: HashSet<usize> = HashSet::new();
        // HashSet<(Zone Idx, Kombisensor Idx)>
        let mut seen_kombisensors: HashSet<(usize, usize)> = HashSet::new();
        // HashSet<(Zone, Kombisensor, Sensor)>
        let mut seen_sensors: HashSet<(usize, usize, usize)> = HashSet::new();

        // ZONE
        if let Some(mut iter_zones) = treestore.get_iter_first() {
            let mut valid_zone = true;
            while valid_zone {
                // Zonen durchlaufen
                if let Some(zone_id) = treestore.get_value(&iter_zones, 0).get::<String>() {
                    if let Ok(zone_id) = zone_id.parse::<usize>() {
                        if let Some(zone) = client.get_server_data().get_zone(zone_id) {
                            treestore.set(&iter_zones,
                                &[0],
                                &[
                                    &format!("{}", zone_id),
                                ],
                            );

                            // KOMBISENSOR
                            if let Some(mut iter_kombisensors) = treestore.iter_children(&iter_zones) {
                                let mut valid_kombisensor = true;
                                while valid_kombisensor {
                                    // Kombisensoren der Zone durchlaufen
                                    if let Some(kombisensor_id) = treestore.get_value(&iter_kombisensors, 0).get::<String>() {
                                        if let Ok(kombisensor_id) = kombisensor_id.parse::<usize>() {
                                            if let Some(kombisensor) = zone.get_kombisensor(kombisensor_id) {
                                                treestore.set(&iter_kombisensors,
                                                    &[0, 1, 2],
                                                    &[
                                                        &format!("{}", kombisensor_id),
                                                        &format!("{}", kombisensor.get_kombisensor_type()),
                                                        &format!("{}", kombisensor.get_modbus_address()),
                                                    ],
                                                );

                                                // SENSOR
                                                if let Some(mut iter_sensors) = treestore.iter_children(&iter_kombisensors) {
                                                    let mut valid_sensor = true;
                                                    while valid_sensor {
                                                        // Sensoren des Kombisensors durchlaufen
                                                        if let Some(sensor_id) = treestore.get_value(&iter_sensors, 0).get::<String>() {
                                                            if let Ok(sensor_id) = sensor_id.parse::<usize>() {
                                                                if let Some(sensor) = kombisensor.get_sensor(sensor_id) {
                                                                    treestore.set(&iter_sensors,
                                                                        &[0, 1, 3, 4, 5],
                                                                        &[
                                                                            &format!("{}", sensor_id),
                                                                            &format!("{}", sensor.get_sensor_type()),
                                                                            &format!("{:.02}", sensor.get_concentration()),
                                                                            &format!("{:.02}", sensor.get_concentration_average_15min()),
                                                                            &format!("{}", sensor.get_si()),
                                                                        ],
                                                                    );

                                                                    valid_sensor = treestore.iter_next(&mut iter_sensors);
                                                                    seen_sensors.insert((zone_id, kombisensor_id, sensor_id));
                                                                } else {
                                                                    valid_sensor = treestore.remove(&mut iter_sensors);
                                                                } // Ende if/ else get Sesor vom Server
                                                            } // Ende `sensor_id.parse()`
                                                        } // Ende get sensor_id vom TreeStore
                                                    } // Ende `while valid_sensor`
                                                } // Ende iter_sensors

                                                valid_kombisensor = treestore.iter_next(&mut iter_kombisensors);
                                                seen_kombisensors.insert((zone_id, kombisensor_id));
                                            } else {
                                                valid_kombisensor = treestore.remove(&mut iter_kombisensors);
                                            } // Ende if/ else get Kombisensor vom Server
                                        } // Ende `kombisensor_id.parse()`
                                    } // Ende get kombisensor_id vom TreeStore
                                } // Ende `while valid_kombisensor`
                            } // Ende iter_kombisensors

                            valid_zone = treestore.iter_next(&mut iter_zones);
                            seen_zones.insert(zone_id);
                        } else {
                            valid_zone = treestore.remove(&mut iter_zones);
                        } // Ende if/ else get zone vom Server
                    } // Ende `zone_id.parse()`
                } // Ende get zone_id vom TreeStore
            } // Ende `while valid_zone`
        } // Ende iter_zones


        // Unbekannte Zone erfassen
        for (zone_id, zone) in client.get_server_data().get_zones().iter().enumerate() {
            if !seen_zones.contains(&zone_id) {
                println!("erfasse Zone: {}", &zone_id);
                let iter_zones = create_and_fill_model_zone(treestore, zone_id);
                for (kombisensor_id, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                    let iter_kombisensors = create_and_fill_model_kombisensor(treestore, &iter_zones, &kombisensor, kombisensor_id);
                    for (sensor_id, sensor) in kombisensor.get_sensors().iter().enumerate() {
                        let _iter_sensors = create_and_fill_model_sensor(treestore, &iter_kombisensors, &sensor, sensor_id);
                    }
                }
            } else {
                // Unbekannte Kombisensor erfassen
                for (kombisensor_id, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                    if !seen_kombisensors.contains(&(zone_id, kombisensor_id)) {
                        println!("erfasse Kombisensor: {}", &kombisensor_id);
                        if let Some(iter_zones) = treestore.get_iter_from_string(&format!("{}", zone_id)) {
                            let iter_kombisensors = create_and_fill_model_kombisensor(treestore, &iter_zones, &kombisensor, kombisensor_id);
                            for (sensor_id, sensor) in kombisensor.get_sensors().iter().enumerate() {
                                let _iter_sensors = create_and_fill_model_sensor(treestore, &iter_kombisensors, &sensor, sensor_id);
                            }
                        }
                    } else {
                        // Unbekannten Sensor erfassen
                        for (sensor_id, sensor) in kombisensor.get_sensors().iter().enumerate() {
                            if !seen_sensors.contains(&(zone_id, kombisensor_id, sensor_id)) {
                                println!("erfasse Sensor: {}", &sensor_id);
                                if let Some(iter_kombisensors) = treestore.get_iter_from_string(&format!("{}:{}", zone_id, kombisensor_id)) {
                                    let _iter_sensors = create_and_fill_model_sensor(treestore, &iter_kombisensors, &sensor, sensor_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        treeview.set_model(Some(treestore));
        treeview.expand_all();
    } // Ende Server::lock();
}

// Model
fn setup_treestore() -> Result<TreeStore> {
    let treestore = TreeStore::new(
        &[
            String::static_type(),  // 0:id zone_id, (zone_id, kombisensor_id), (zone_id, kombisensor_id, sensor_id)
            String::static_type(),  // 1:type
            String::static_type(),  // 2:modbus_address
            String::static_type(),  // 3:value
            String::static_type(),  // 4:average
            String::static_type(),  // 5:si
        ]
    );

    Ok(treestore)
}

// View
fn setup_treeview() -> Result<TreeView> {
    let treeview = TreeView::new();

    treeview.set_headers_visible(true);
    #[cfg(feature = "development")]
    append_text_column(&treeview, None, 0); // "ID"
    append_text_column(&treeview, Some("Typ"), 1);
    append_text_column(&treeview, Some("Modbus"), 2);
    append_text_column(&treeview, Some("DIW"), 3);
    append_text_column(&treeview, Some("Ø 15min"), 4);
    append_text_column(&treeview, None, 5); // "SI"

    Ok(treeview)
}

fn setup_window() -> Result<Window> {
    let window = Window::new(WindowType::Toplevel);

    let window_title = format!("{} {}",
               env!("CARGO_PKG_DESCRIPTION"),
               env!("CARGO_PKG_VERSION"));

   window.set_title(&window_title);
   window.set_default_size(1024, 600);

   #[cfg(not(feature = "development"))]
   window.fullscreen();

    if let Some(display) = window.get_display() {
        let screen = display.get_screen(0);
        screen.set_resolution(150.0);

        // CSS Datei einbinden
        let css_style_provider = gtk::CssProvider::new();
        let css_gui = include_str!("gui.css");
        match css_style_provider.load_from_data(css_gui) {
        Ok(_) => {
            gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
            Err(e) => { bail!("Laden des CSS Style Providers failed: {}", e) }
        }
    }

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Connect `ESC` key event to gtk main_quit
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    Ok(window)
}

pub fn launch(hostname: &str) -> Result<()> {
    if gtk::init().is_err() {
        bail!("Failed to initalize GTK.");
    }

    let client = ApiClient::new("0.0.0.0");
    let client = Arc::new(Mutex::new(client));
    let window = setup_window()?;
    let treeview = setup_treeview()?;
    let treestore = setup_treestore()?;

    update_client(&client.clone());
    treestore_fill(&client.clone(), &treestore, &treeview);

    let scrolled_window = ScrolledWindow::new(None, None);
    let hbox = gtk::Box::new(Orientation::Horizontal, 10);
    hbox.pack_start(&treeview, true, true, 20);
    scrolled_window.add(&hbox);
    window.add(&scrolled_window);


    gtk::timeout_add(100, clone!(client, treestore, treeview => move || {
        update_client(&client);
        treestore_update(&client, &treestore, &treeview);

        ::glib::Continue(true)
    }));


    window.show_all();
    gtk::main();

    Ok(())
}
