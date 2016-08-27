extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gdk::enums::key;

// Basic Setup des Fensters
fn window_setup(window: &gtk::Window) {
    window.set_title("Global CSS");
    window.set_default_size(600, 400);

    // CSS wird hier gleich im Fenster Setup dem kompletten Screen zugeordnet.
    //
    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    let css_style_provider = gtk::CssProvider::new();
    // Das CSS File sollte aus dem Dateisystem mit `include_str!` eingebunden werden.
    // let css_file = include_str!("css_file.css");
    let css_file = "* { color: red; }"; // In diesem Testfile wird Anstelle des CSS Files ein String verwendet
    css_style_provider.load_from_data(css_file).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn main() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
                       env!("CARGO_PKG_NAME")))
    });

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let button = gtk::Button::new_with_label("Click me!");
    let label = gtk::Label::new(Some("Globale CSS Eigenschaften für die gesammte Application (Screen im gtk Sprachgebrauch)!"));
    label.set_vexpand(true);

    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vertical_layout.add(&label);
    vertical_layout.add(&button);

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_setup(&window);

    window.add(&vertical_layout);
    window.show_all();

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        gtk::Inhibit(false)
    });


    gtk::main();
}
