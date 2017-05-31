extern crate gtk;
extern crate gdk;

use gdk::DisplayExt;
use gdk::enums::key;
use gtk::prelude::*;


fn main() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: Could not initalise Gtk.",
                       env!("CARGO_PKG_NAME")))
    });

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let button = gtk::Button::new_with_label("Click me!");
    let label = gtk::Label::new(Some("Globale CSS properties, application wide (screen in gtk lingua)!"));
    let vertical_layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    window.set_title("Globale CSS");
    window.set_default_size(600, 400);
    label.set_vexpand(true);

    vertical_layout.add(&label);
    vertical_layout.add(&button);

    // Here the CSS assigned to the complete screen
    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    let css_style_provider = gtk::CssProvider::new();
    // Include the CSS file from filesystem with eht `include_str!` macro
    //let css_file = include_str!("css_file.css");
    // Or use simply a string like this for it
    let css_file = "* { color: red; }";
    css_style_provider.load_from_data(css_file).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.add(&vertical_layout);
    window.show_all();
    // Close context
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::Inhibit(false)
    });
    // Close application with Esc key, too
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        gtk::Inhibit(false)
    });

    gtk::main();
}
