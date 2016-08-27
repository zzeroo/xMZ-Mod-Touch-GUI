
use gtk;
use gtk::prelude::*;
use gdk::enums::key;
use xmz_client::client::Client;

// Die einzelnen Unterfenster sind in seperaten Modulen organisiert
mod module_index;
mod sysinfo_index;
mod settings_index;


// Basic Setup des Fensters
fn window_setup(window: &gtk::Window) {
    let window_title = format!("{}-v{}",
                               env!("CARGO_PKG_DESCRIPTION"),
                               env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);
    // CSS
    let css_style_provider = gtk::CssProvider::new();
    let css_interface = include_str!("interface.css");
    css_style_provider.load_from_data(css_interface).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    match ::std::env::var("XMZ_HARDWARE") {
        Ok(_) => {
            window.fullscreen();
        }
        Err(_) => {}
    }
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
                       env!("CARGO_PKG_NAME")))
    });

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_string(include_str!("interface.glade"));
    let window: gtk::Window = builder.get_object("main_window").unwrap();
    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();

    {   // Hide info_bar
        let info_bar = info_bar.clone();
        info_bar.connect_response(move |info_bar, _| info_bar.hide());
    }

    // Client erzeugen.
    // Der Client ist das Gegenstück um mit dem Server zu kommunizieren
    let mut client = Client::new();



    // Module Index aufbauen
    module_index::setup(&builder, &window, &mut client);

    // System Information bauen
    sysinfo_index::setup(&builder);

    // Einstellungen Fenster (Settings) bauen
    settings_index::setup(&builder);

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.show_all();
    info_bar.hide();

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    // Registriert die Esc Taste mit main_quit() (schliesst also das Fenster mit der Esc Taste),
    // nur in DEBUG Builds. Wird das Programm mit `--release` übersetzt, funktioniert dies nicht.
    #[cfg(debug_assertions)]
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });


    gtk::main();
}
