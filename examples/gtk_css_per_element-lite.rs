extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gdk::enums::key;

fn main() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
                       env!("CARGO_PKG_NAME")))
    });

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let button = gtk::Button::new_with_label("Click me!");
    let label = gtk::Label::new(Some("Set CSS properties per element!"));
    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    window.set_title("CSS per Element");
    window.set_default_size(600, 400);
    label.set_vexpand(true);

    vertical_layout.add(&label);
    vertical_layout.add(&button);

    // CSS for the button
    let css_style_provider = gtk::CssProvider::new();
    let style = "* { color: blue; font: Monospace 22; }";
    css_style_provider.load_from_data(style).unwrap();
    // get the 'style context' of the button
    let style_context = button.get_style_context().unwrap();
    style_context.add_provider(&css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    // add elements
    window.add(&vertical_layout);
    window.show_all();
    // Close on exit
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    // close via Esc key
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        gtk::Inhibit(false)
    });

    gtk::main();
}
