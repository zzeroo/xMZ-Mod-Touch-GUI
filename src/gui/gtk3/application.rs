#[macro_use] use gui::gtk3::macros;
use error::*;
use gdk::enums::key;
use gtk;
use gtk::prelude::*;
use hyper::{Client};
use serde_json;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::server::{Server};
use gtk_sys;
use gobject_sys;
use glib;
use glib::Value;


// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


fn poll_server_web_interface(server: Arc<Mutex<Server>>) -> Result<()> {
    let _ = thread::spawn(move || {
        let client = Client::new(); // hyper::Client;
        let mut res = match client.get("http://localhost:3000/").send() {
            Err(err) => { debug!("{:?}", err) }
            Ok(mut res) => {
                assert_eq!(res.status, ::hyper::Ok);

                let mut s = String::new();
                match res.read_to_string(&mut s) {
                    Err(err) => { debug!("{:?}", err) }
                    Ok(_) => {
                        {
                            let mut server = server.lock().unwrap();
                            *server = serde_json::from_str(&s).unwrap();
                        }
                    }
                }
            }
        };
    }).join();

    Ok(())
}

fn window_main_setup(window: &gtk::Window) {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));

    window.set_title(&window_title);
    window.set_default_size(1024, 600);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    // CSS Datei einbinden
    let css_style_provider = gtk::CssProvider::new();
    let css_gui = include_str!("gui.css");
    css_style_provider.load_from_data(css_gui).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    #[cfg(not(feature = "development"))]
    window.fullscreen();
}

fn create_treestore(builder: &gtk::Builder, server: Arc<Mutex<Server>>) {
    let treeview_kombisensors: gtk::TreeView = builder.get_object("treeview_kombisensors").unwrap();
    let treestore_kombisensors: gtk::TreeStore = builder.get_object("treestore_kombisensors").unwrap();

    match server.lock() {
        Err(_) => {}
        Ok(server) => {
            for kombisensor in server.get_kombisensors().iter() {
                let iter = treestore_kombisensors.append(None);
                treestore_kombisensors.set_value(&iter, 0u32, &Value::from(&kombisensor.get_modbus_slave_id()));
                treestore_kombisensors.set_value(&iter, 1u32, &Value::from(&format!("{}", kombisensor.get_kombisensor_type())));
                treestore_kombisensors.set_value(&iter, 4u32, &Value::from(&format!("{}", kombisensor.get_error_count())));

                for sensor in kombisensor.get_sensors().iter() {
                    let iter = treestore_kombisensors.append(Some(&iter));
                    treestore_kombisensors.set_value(&iter, 1u32, &Value::from(&format!("{}", sensor.get_sensor_type())));
                    treestore_kombisensors.set_value(&iter, 2u32, &Value::from(&format!("{:.02}", sensor.get_concentration())));
                    treestore_kombisensors.set_value(&iter, 3u32, &Value::from(&format!("{}", sensor.get_si())));
                }
                treeview_kombisensors.expand_all();
            }
        }
    }
}

fn update_treestore(builder: &gtk::Builder, server: Arc<Mutex<Server>>) {
    let treeview_kombisensors: gtk::TreeView = builder.get_object("treeview_kombisensors").unwrap();
    let treestore_kombisensors: gtk::TreeStore = builder.get_object("treestore_kombisensors").unwrap();

    match server.lock() {
        Err(_) => {}
        Ok(server) => {
            if let Some(iter) = treestore_kombisensors.get_iter_first() {
                let mut valid = true;
                while valid {
                    for kombisensor in server.get_kombisensors().iter() {
                        treestore_kombisensors.set_value(&iter, 0u32, &Value::from(&kombisensor.get_modbus_slave_id()));
                        treestore_kombisensors.set_value(&iter, 1u32, &Value::from(&format!("{}", kombisensor.get_kombisensor_type())));
                        treestore_kombisensors.set_value(&iter, 4u32, &Value::from(&format!("{}", kombisensor.get_error_count())));

                        for (i, sensor) in kombisensor.get_sensors().iter().enumerate() {
                            if let Some(iter) = treestore_kombisensors.iter_nth_child(Some(&iter), i as i32) {
                                treestore_kombisensors.set_value(&iter, 1u32, &Value::from(&format!("{}", sensor.get_sensor_type())));
                                treestore_kombisensors.set_value(&iter, 2u32, &Value::from(&format!("{:.02}", sensor.get_concentration())));
                                treestore_kombisensors.set_value(&iter, 3u32, &Value::from(&format!("{}", sensor.get_si())));
                            }
                        }

                        // treeview_kombisensors.expand_all();
                        valid = treestore_kombisensors.iter_next(&iter);
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn launch() -> Result<()> {
    use glib::translate::ToGlibPtr;

    let server = Arc::new(Mutex::new(Server::new()));
    {
        poll_server_web_interface(server.clone());
    }

    // // Disable Animationen
    // // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    // unsafe{
    //     gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
    //     "gtk-enable-animations".to_glib_none().0, 0, 0);
    // }

    let glade_str = include_str!("gui.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str)?;

    let window_main: gtk::Window = builder.get_object("window_main").unwrap();
    let treeview_kombisensors: gtk::TreeView = builder.get_object("treeview_kombisensors").unwrap();
    let treestore_kombisensors: gtk::TreeStore = builder.get_object("treestore_kombisensors").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_main_setup(&window_main);

    { // Hide info_bar
            let info_bar = info_bar.clone();
            info_bar.connect_response(move |info_bar, _| info_bar.hide());
    }

    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window_main.show_all();
    info_bar.hide();

    create_treestore(&builder, server.clone());

    // Server Update Task
    gtk::idle_add(clone!(server => move || {
        // Update Server struct via http
        match poll_server_web_interface(server.clone()) {
            Err(err) => {}
            Ok(_) => {}
        }
        thread::sleep(Duration::from_millis(100));

        ::glib::Continue(true)
    }));

    // TreeStore Update Task
    gtk::idle_add(clone!(server => move || {
        update_treestore(&builder, server.clone());
        thread::sleep(Duration::from_millis(1000));

        ::glib::Continue(true)
    }));

    #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    gtk::main();

    Ok(())
}
