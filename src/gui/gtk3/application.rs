use error::*;
use gdk::enums::key;
use gobject_sys;
use gtk_sys;
use gtk;
use gtk::prelude::*;
use hyper::{Client};
use serde_json;
use std::collections::HashSet;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::server::{Server};

// make moving clones into closures more convenient
#[macro_export]
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
    let thread_poll = thread::spawn(move || {
        let client = Client::new(); // hyper::Client;
        let mut res = match client.get("http://127.0.0.1:3000/").send() {
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

fn update_treestore(server: Arc<Mutex<Server>>, treestore: &gtk::TreeStore) {
    if let Some(mut iter) = treestore.get_iter_first() {
        println!("{:?}", treestore.get_value(&iter, 1));
    }
}


fn window_main_setup(window: &gtk::Window) {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));
    window.set_default_size(1024, 600);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    // CSS Datei einbinden
    let css_style_provider = gtk::CssProvider::new();
    let css_gui = include_str!("gui.css");
    css_style_provider.load_from_data(css_gui).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    #[cfg(not(feature = "dev"))]
    window.fullscreen();
}

fn fill_treestore(server: Arc<Mutex<Server>>, treestore: &gtk::TreeStore, treeview: &gtk::TreeView) {
    match server.lock() {
        Err(_) => { println!("Server konnte nicht gelockt werden"); }
        Ok(server) => {
            for kombisensor in server.get_kombisensors().iter() {
                let iter = &treestore.insert_with_values(
                    None,
                    None,
                    &[0, 1, 4],
                    &[
                        &kombisensor.get_modbus_slave_id(),
                        &format!("{}", kombisensor.get_kombisensor_type()),
                        &kombisensor.get_error_count(),
                    ]
                );

                for sensor in kombisensor.get_sensors().iter() {
                    println!("{:?}", format!("{}", &sensor.get_sensor_type()));
                    println!("{:?}", format!("{:.02}", &sensor.get_concentration()));
                    println!("{:?}", format!("{}", &sensor.get_si()));

                //     &treestore.insert_with_values(
                //         Some(&iter),
                //         None,
                //         &[1, 2, 3],
                //         &[
                //             &"foo",
                //             &"bar",
                //             &"baz",
                //             // &format!("{}", &sensor.get_sensor_type()),
                //             // &format!("{:.02}", &sensor.get_concentration()),
                //             // &format!("{}", &sensor.get_si()),
                //         ]
                //     );
                }
            }
            treeview.expand_all();
        }
    }
}


/// Helper Funktion zum Anfügen weiterer Spalten (columns) in einen TreeView
///
/// # Params
///
/// `treeview`  - Der TreeView mit dem gearbeitet werden soll
/// `id`        - ID der Spalte, Null basiert
///
/// FIXME: Auslagern in Glade!
fn append_column(treeview: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // // Die Daten und das View werden über `id` Spalte des Models und
    // über die `id` Spalte des Stores verbunden.
    column.add_attribute(&cell, "text", id);
    // Diverse Attribute
    column.set_resizable(false);
    column.set_clickable(false);
    treeview.append_column(&column);
}

/// Basis Setup des TreeViews
///
fn setup_treeview(treeview: &gtk::TreeView) {
    // Header verstecken
    treeview.set_headers_visible(true);

    append_column(&treeview, 1);
    append_column(&treeview, 2);
    append_column(&treeview, 3);
    append_column(&treeview, 4);
    append_column(&treeview, 5);
}

pub fn launch() -> Result<()> {
    let server = Arc::new(Mutex::new(Server::new()));
    poll_server_web_interface(server.clone());

    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        use glib::translate::ToGlibPtr;
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let glade_str = include_str!("gui2.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str)?;

    let window_main: gtk::Window = builder.get_object("window_main").unwrap();
    // let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
    let scrolled_window: gtk::ScrolledWindow = builder.get_object("scrolled_window").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_main_setup(&window_main);

    { // Module Index aufbauen
        try!(::module_index::setup(&builder, &window_main, server.clone()));
    }

    // { // Hide info_bar
    //         let info_bar = info_bar.clone();
    //         info_bar.connect_response(move |info_bar, _| info_bar.hide());
    // }

    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let treeview_kombisensors = gtk::TreeView::new();
    let treestore_kombisensors = gtk::TreeStore::new(&[u32::static_type(),      // Modbus Slave Id
                                                        String::static_type(),  // Type
                                                        String::static_type(),  // Value
                                                        String::static_type(),  // Si
                                                        u32::static_type()      // Errors
    ]);
    // Verbinde View und Model (TreeStore)
    treeview_kombisensors.set_model(Some(&treestore_kombisensors));
    // Setup des TreeViews
    setup_treeview(&treeview_kombisensors);
    // Kombisensoren Index
    fill_treestore(server.clone(), &treestore_kombisensors, &treeview_kombisensors);
    scrolled_window.add(&treeview_kombisensors);

    window_main.show_all();
    // info_bar.hide();

    // Server Update Task
    gtk::idle_add(clone!(server => move || {
        // Update Server struct via http
        match poll_server_web_interface(server.clone()) {
            Err(err) => { println!("Webinterface Fehler")}
            Ok(_) => {}
        }
        thread::sleep(Duration::from_millis(100));

        ::glib::Continue(true)
    }));

    #[cfg(feature = "dev")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    gtk::main();

    Ok(())
}
