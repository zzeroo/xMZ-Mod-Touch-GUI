use gdk::enums::key;
// FIXME: Entferne `use gtk` und ersetze es durch `use gtk::{Window, Button, usw...}` in diesem File
use gtk;
use gtk::prelude::*;
use gui::gtk3::*;
use errors::*;
use xmz_client::client::*;

extern crate libc;
extern crate glib;
use glib::translate::ToGlibPtr;


pub struct App {
}

impl App {
    /// Erzeugt einen neuen Application Kontext
    pub fn new() -> Self {
        App { }
    }

    /// Konstruiert die Fensterelemente, Signale und so weiter und zeigt am Ende das Fenster an
    pub fn launch(&self) -> Result<()> {
        let mut client = Client::new();
        client.set_socket_receive_timeout(200);
        client.set_socket_send_timeout(200);

        gtk::init().unwrap_or_else(|_| {
            panic!("GTK konnte nicht initalisiert werden.");
        });

        // // Disable Animationen
        // // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
        // // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
        // unsafe{
        //     self::gobject_ffi::g_object_set (gtk_ffi::gtk_settings_get_default () as gpointer,
        //     "gtk-enable-animations".to_glib_none().0, glib_ffi::GFALSE, ::std::ptr::null::<libc::c_void>());
        // }

        // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
        let builder = gtk::Builder::new();
        builder.add_from_string(include_str!("interface.glade")).unwrap();
        let window: gtk::Window = builder.get_object("main_window").unwrap();
        let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();
        let info_bar_error: gtk::InfoBar = builder.get_object("info_bar_error").unwrap();

        {   // Hide info_bar's
            let info_bar = info_bar.clone();
            info_bar.connect_response(move |info_bar, _| info_bar.hide());
            let info_bar = info_bar_error.clone();
            info_bar.connect_response(move |info_bar, _| info_bar.hide());
        }
        // Module Index aufbauen
        try!(module_index::setup(&builder, &window, &mut client).chain_err(|| "Module Index konnte nicht aufgebaut werden"));
        // System Information bauen
        sysinfo_index::setup(&builder);
        // Einstellungen Fenster (Settings) bauen
        try!(settings_index::setup(&builder, &window, &mut client).chain_err(|| "Einstellung Index konnte nicht aufgebaut werden"));
        // Rufe Funktion für die Basis Fenster Konfiguration auf
        window_setup(&window);
        // Fenster anzeigen und Infobar verstecken
        window.show_all();
        info_bar.hide();
        info_bar_error.hide();
        // Beende Programm wenn das Fenster geschlossen wurde
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        // Registriert die Esc Taste mit main_quit() (schliesst also das Fenster mit der Esc Taste),
        // nur in DEBUG Builds. Wird das Programm mit `--release` übersetzt, funktioniert dies nicht.
        #[cfg(debug_assertions)]
        window.connect_key_press_event(move |_, key| {
            debug!("Tastendruck erkannt");
            if let key::Escape = key.get_keyval() {
                debug!("Escape erkannt");
                gtk::main_quit();
            }
            Inhibit(false)
        });

        // Registriert die F5 Taste mit info_bar.show().
        // Wird das Programm mit `--release` übersetzt, funktioniert dies nicht.
        #[cfg(debug_assertions)]
        window.connect_key_press_event(move |_, key| {
            debug!("Tastendruck erkannt");
            if let key::F5 = key.get_keyval() {
                debug!("F5 erkannt");
                debug!("Infobar zeigen");
                info_bar.show()
            }
            Inhibit(false)
        });

        gtk::main();

        Ok(())
    }
}


/// Basic Setup des Fensters
///
/// Diese Funktion stellt den Fenster Titel aus der Beschreibung und der Version die aus dem
/// Cargo.toml File gezogen werden.
/// Anschließend wied die DPI Auflösung des Fensters geändert, so das die Schriften auf der
/// 'xMZ-Mod-Touch'-Hardware besser zu lesen sind. Und eine global gültige CSS Datei eingebunden.
/// Am Ende der Funktion wird das Fenster in den Fullscreen Mode gesetzt wenn die Umgebungsvariable
/// `XMZ_HARDWARE` gesetzt ist.
fn window_setup(window: &gtk::Window) {
    // Window Title Informationen gewinnen und Titel setzen
    let window_title = format!("{}-v{}",
                               env!("CARGO_PKG_DESCRIPTION"),
                               env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    // DPI Auflösung ändern
    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);
    // CSS Datei einbinden
    let css_style_provider = gtk::CssProvider::new();
    let css_interface = include_str!("interface.css");
    css_style_provider.load_from_data(css_interface).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    // Fenster im Fullscreen Mode starten wenn die Umgebungsvariable `XMZ_HARDWARE` gesetzt ist.
    // Der Wert von `XMZ_HARDWARE` ist egal,
    // `XMZ_HARDWARE="0.1.0"` ist genau so gut wie `XMZ_HARDWARE=""`.
    match ::std::env::var("XMZ_HARDWARE") {
        Ok(_) => {
            window.fullscreen();
        }
        Err(_) => {}
    }
}