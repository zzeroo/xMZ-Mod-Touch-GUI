use error::*;
use gdk::enums::key;
use gtk;
use glib;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};
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
fn update_server(server: &Arc<Mutex<XMZModTouchServer>>) {
    use std::io::Read;
    use hyper;
    use serde_json;

    let client = hyper::Client::new();
    if let Ok(mut response) = client.get("http://localhost:3000/api/v1").send() {
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

fn create_treestore(builder: &gtk::Builder, server: Arc<Mutex<XMZModTouchServer>>) {
    let treeview_kombisensors: gtk::TreeView   = build!(builder, "treeview_kombisensors");
    let treestore_kombisensors: gtk::TreeStore = build!(builder, "treestore_kombisensors");

    if let Ok(server) = server.lock() {
        for (zone_id, zone) in server.get_zones().iter().enumerate() {
            let iter = treestore_kombisensors.append(None);
            treestore_kombisensors.set_value(&iter, 0, &glib::Value::from( &( (zone_id as i32) + 1 )) );

            for kombisensor in zone.get_kombisensors() {
                let iter = treestore_kombisensors.append(Some(&iter));
                treestore_kombisensors.set_value(&iter, 1, &glib::Value::from( &format!("{}", kombisensor.get_modbus_address()) ));
                treestore_kombisensors.set_value(&iter, 2, &glib::Value::from( &format!("{}", kombisensor.get_kombisensor_type()) ));
                treestore_kombisensors.set_value(&iter, 6, &glib::Value::from( &format!("{}", kombisensor.get_error_count()) ));

                for sensor in kombisensor.get_sensors() {
                    let iter = treestore_kombisensors.append(Some(&iter));
                    treestore_kombisensors.set_value(&iter, 2, &glib::Value::from( &format!("{}", sensor.get_sensor_type() )));
                    treestore_kombisensors.set_value(&iter, 3, &glib::Value::from( &format!("{:.02}", sensor.get_concentration() )));
                    treestore_kombisensors.set_value(&iter, 4, &glib::Value::from( &format!("{:.02}", sensor.get_concentration_average_15min() )));
                    treestore_kombisensors.set_value(&iter, 5, &glib::Value::from( &format!("{}", sensor.get_si() )));
                }
            }
        }
        treeview_kombisensors.expand_all();
    }
}

pub fn launch() -> Result<()> {
    // Create a XMZModTouchServer in a Arc Mutex
    let server = Arc::new(Mutex::new(XMZModTouchServer::new()));
    update_server(&server.clone());

    if gtk::init().is_err() {
        error!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }

    let glade_str = include_str!("main.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str)?;

    let window_main: gtk::Window = build!(builder, "window_main");
    window_main_setup(&window_main)?;

    // #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    create_treestore(&builder, server.clone());

    // Show 2nd notebook tab
    let notebook_main: gtk::Notebook = build!(builder, "notebook_main");
    notebook_main.set_current_page(Some(1));

    // Timer


    window_main.show_all();

    gtk::main();

    Ok(())
}
