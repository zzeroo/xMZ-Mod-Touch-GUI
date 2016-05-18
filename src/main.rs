// Diese Includes sind alle nötig
extern crate gdk;
extern crate gtk;
use std::env;
use gdk::enums::*;
use gtk::prelude::*;
mod app;
use app::App;


fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initalize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Window properties
    window.set_title("Stack switcher test");
    window.set_default_size(1200, 600);
    match env::var("XMZ_HARDWARE") {
        Ok(_) => { window.fullscreen() },
        Err(_) => {},
    }

    // Connect delete event to quit the gtk::main thread
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    // Connect ESC key press event, and quit the gui if ESC was pressed
    window.connect_key_press_event(move |_, key| {
        match key.get_keyval() as u32 {
            key::Escape => gtk::main_quit(),
            _ => (),
        }
        Inhibit(false)
    });

    let box_main = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let mut app = App::new();
    box_main.pack_start(&app.stack, true, true, 0);
    window.add(&box_main);

    // Construct the StackSwitcher
    for i in &["Sensor 1", "Sensor 2", "Sensor 3", "Einstellungen"] {
        app.create_windows(&i.to_string());
    }

    // Swipe
    let swipe = gtk::GestureSwipe::new(&app.stack);
    swipe.connect_swipe(move |_swipe, swipe_x, _swipe_y| {
        match swipe_x < 0f64 {
            true  => { app.next_window() },
            false => { app.prev_window() },
        };
    });

    window.show_all();

    gtk::main();
}
