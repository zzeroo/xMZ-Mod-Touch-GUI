use gtk::{self};
use gtk::prelude::*;
use gdk::enums::key;


pub fn launch() {
    // TODO: Programm Name aus env!("CARGO_PKG_NAME") nutzen
    gtk::init().unwrap_or_else(|_| panic!("xmz_mod_touch_gui: GTK konnte nicht initalisiert werden."));

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_string(include_str!("interface.glade"));
    let window: gtk::Window                         = builder.get_object("main_window").unwrap();


    window.show_all();

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    #[cfg(debug_assertions)]
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() { gtk::main_quit() }
        Inhibit(false)
    }) ;

    gtk::main();
}
