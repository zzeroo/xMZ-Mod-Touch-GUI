extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{
    Window, WindowType, WindowPosition,
};
use gdk::enums::key;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialise Gtk3.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("TreeView Tests");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() { gtk::main_quit() }
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
