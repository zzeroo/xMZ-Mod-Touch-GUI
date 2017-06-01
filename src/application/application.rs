use error::*;
use gdk::DisplayExt;
use gdk::enums::key;
use gdk::ScreenExt;
use glib;
use glib::translate::ToGlibPtr;
use gobject_sys;
use gtk_sys;
use gtk;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
use xmz_mod_touch_client::XMZModTouchClient;
use xmz_mod_touch_server::XMZModTouchServer;


fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));

    window.set_title(&window_title);
    window.set_default_size(1024, 600);

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
            Err(e) => { error!("Error: css_style_provider.load_from_data() failed: {}", e) }
        }
    }

    #[cfg(not(feature = "development"))]
    window.fullscreen();

    Ok(())
}


/// Holt via http/ json request aktuelle Serverdaten
///
///
fn update_server(server: &Arc<Mutex<XMZModTouchServer>>, hostname: Arc<String>) {
    use std::io::Read;
    use hyper;
    use serde_json;

    let client = hyper::Client::new();
    let url = format!("http://{}:3000", &*hostname);
    if let Ok(mut response) = client.get(&url).send() {
        let mut s = String::new();
        match response.read_to_string(&mut s) {
            Err(e) => println!("Error: {}", e),
            Ok(_size) => {
                if let Ok(mut server) = server.lock() {
                    if let Ok(s) = serde_json::from_str::<XMZModTouchServer>(&s) {
                        *server = s;
                    }
                }
            }
        }
    }
}

fn create_treestore(builder: &gtk::Builder, server: Arc<Mutex<XMZModTouchServer>>) -> gtk::TreeStore {
    let treeview_kombisensors: gtk::TreeView   = build!(builder, "treeview_kombisensors");
    let treestore_kombisensors: gtk::TreeStore = build!(builder, "treestore_kombisensors");

    if let Ok(server) = server.lock() {
        // Zones
        for (zone_id, zone) in server.get_zones().iter().enumerate() {
            let iter = treestore_kombisensors.append(None);
            treestore_kombisensors.set_value(&iter, 0, &glib::Value::from( &( (zone_id as i32) + 1 )) );
            treestore_kombisensors.set_value(&iter, 7, &glib::Value::from( &(zone_id as i32) ));

            // Kombisensors
            for (kombisensor_id, kombisensor) in zone.get_kombisensors().iter().enumerate() {
                let iter = treestore_kombisensors.append(Some(&iter));
                treestore_kombisensors.set_value(&iter, 1, &glib::Value::from( &format!("{}", kombisensor.get_modbus_address()) ));
                treestore_kombisensors.set_value(&iter, 2, &glib::Value::from( &format!("{}", kombisensor.get_kombisensor_type()) ));
                treestore_kombisensors.set_value(&iter, 6, &glib::Value::from( &format!("{}", kombisensor.get_error_count()) ));
                treestore_kombisensors.set_value(&iter, 7, &glib::Value::from( &(kombisensor_id as i32) ));

                // Sensors
                for (sensor_id, sensor) in kombisensor.get_sensors().iter().enumerate() {
                    if sensor.is_enabled() {
                        let iter = treestore_kombisensors.append(Some(&iter));
                        treestore_kombisensors.set_value(&iter, 2, &glib::Value::from( &format!("{}", sensor.get_sensor_type() )));
                        treestore_kombisensors.set_value(&iter, 3, &glib::Value::from( &format!("{:.02}", sensor.get_concentration() )));
                        treestore_kombisensors.set_value(&iter, 4, &glib::Value::from( &format!("{:.02}", sensor.get_concentration_average_15min() )));
                        treestore_kombisensors.set_value(&iter, 5, &glib::Value::from( &format!("{}", sensor.get_si() )));
                        treestore_kombisensors.set_value(&iter, 7, &glib::Value::from( &(sensor_id as i32) ));
                    }
                }
            }
        }
        treeview_kombisensors.expand_all();
    }

    treestore_kombisensors
}

fn update_treestore(builder: &gtk::Builder, server: &Arc<Mutex<XMZModTouchServer>>, treestore: &gtk::TreeStore) {
    if let Ok(server) = server.lock() {
        // Zone
        if let Some(mut iter) = treestore.get_iter_first() {
            let mut valid_zones = true;

            while valid_zones {
                // Get Zone Id
                let zone_id = treestore.get_value(&iter, 7).get::<u32>();

                // Kombisensor
                if let Some(mut iter) = treestore.iter_children(Some(&iter)) {
                    let mut valid_kombisensors = true;

                    while valid_kombisensors {
                        // Get Kombisensor Id
                        let kombisensor_id = treestore.get_value(&iter, 7).get::<u32>();

                        zone_id.map(|zone_id| {
                            kombisensor_id.map(|kombisensor_id| {
                                server.get_zone(zone_id as usize).map(|zone| {
                                    zone.get_kombisensor(kombisensor_id as usize).map(|kombisensor| {
                                        treestore.set_value(&iter, 1, &glib::Value::from( &format!("{}", kombisensor.get_modbus_address()) ));
                                        treestore.set_value(&iter, 2, &glib::Value::from( &format!("{}", kombisensor.get_kombisensor_type()) ));
                                        treestore.set_value(&iter, 6, &glib::Value::from( &format!("{}", kombisensor.get_error_count()) ));
                                    });
                                });
                            });
                        });

                        // Sensor
                        if let Some(mut iter) = treestore.iter_children(Some(&iter)) {
                            let mut valid_sensors = true;

                            while valid_sensors {
                                // Get Sensor ID
                                let sensor_id = treestore.get_value(&iter, 7).get::<u32>();

                                zone_id.map(|zone_id| {
                                    kombisensor_id.map(|kombisensor_id| {
                                        sensor_id.map(|sensor_id| {
                                            server.get_zone(zone_id as usize).map(|zone| {
                                                zone.get_kombisensor(kombisensor_id as usize).map(|kombisensor| {
                                                    kombisensor.get_sensor(sensor_id as usize).map(|sensor| {
                                                        treestore.set_value(&iter, 2, &glib::Value::from( &format!("{}", sensor.get_sensor_type() )));
                                                        treestore.set_value(&iter, 3, &glib::Value::from( &format!("{:.02}", sensor.get_concentration() )));
                                                        treestore.set_value(&iter, 4, &glib::Value::from( &format!("{:.02}", sensor.get_concentration_average_15min() )));
                                                        treestore.set_value(&iter, 5, &glib::Value::from( &format!("{}", sensor.get_si() )));
                                                    });
                                                });
                                            });
                                        });
                                    });
                                });

                                valid_sensors = treestore.iter_next(&mut iter);
                            }
                        }

                        valid_kombisensors = treestore.iter_next(&mut iter);
                    }
                }

                valid_zones = treestore.iter_next(&mut iter);
            }
        }
    }
}

pub fn launch(client: &XMZModTouchClient) -> Result<()> {
    let hostname = Arc::new(client.get_hostname().to_string());
    // Create a XMZModTouchServer in a Arc Mutex
    let server = Arc::new(Mutex::new(XMZModTouchServer::new()));
    // Einmal den Server "von Hand" aktualisieren
    update_server(&server.clone(), hostname.clone());

    if gtk::init().is_err() {
        error!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }

    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/info_bar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let glade_str = include_str!("main.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str)?;

    let window_main: gtk::Window = build!(builder, "window_main");
    window_main_setup(&window_main)?;

    let info_bar: gtk::InfoBar = build!(builder, "info_bar");
    let label_info_bar_msg: gtk::Label = build!(builder, "label_info_bar_msg");
    label_info_bar_msg.set_text("Anwendung erfolgreich gestartet!");

    // #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    // Close Action der InfoBar
    info_bar.connect_response(clone!(info_bar => move |info_bar, _| {
        info_bar.hide();
    }));

    // Gtk main thread
    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    let treestore_kombisensors = create_treestore(&builder, server.clone());
    gtk::timeout_add(5000, clone!(builder, server, hostname, treestore_kombisensors => move || {
        update_server(&server, hostname.clone());
        update_treestore(&builder, &server, &treestore_kombisensors);

        ::glib::Continue(true)
    }));


    // Show 2nd notebook tab
    let notebook_main: gtk::Notebook = build!(builder, "notebook_main");
    notebook_main.set_current_page(Some(1));

    // Timer


    window_main.show_all();
    info_bar.hide();


    gtk::main();

    Ok(())
}
