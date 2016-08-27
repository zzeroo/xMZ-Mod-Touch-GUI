// In diesem Beispiel wird dem Button ein CSS String/ File zugeordnet. Für eine Application Weite
// Lösung siehe: `examples/gtk_css_global.rs`
//
// https://gitlab.com/search?utf8=%E2%9C%93&search=css&group_id=&project_id=1152771&search_code=true&repository_ref=master
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gdk::enums::key;

// Basic Setup des Fensters
fn window_setup(window: &gtk::Window) {
    window.set_title("CSS per Element");
    window.set_default_size(600, 400);
    // mehr Setup hier ...
}

fn main() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
                       env!("CARGO_PKG_NAME")))
    });

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let button = gtk::Button::new_with_label("Click me!");
    let label = gtk::Label::new(Some("CSS Eigenschaften per Element setzen!"));
    label.set_vexpand(true);

    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vertical_layout.add(&label);
    vertical_layout.add(&button);

    // CSS nur für den Button
    let css_style_provider = gtk::CssProvider::new();
    let style = "* { color: blue; font: Monospace 22; }";
    css_style_provider.load_from_data(style).unwrap();
    // Hier wird der Style Context des Buttons erfragt
    let style_context = button.get_style_context().unwrap();
    style_context.add_provider(&css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

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
