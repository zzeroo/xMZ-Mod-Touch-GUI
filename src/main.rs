extern crate gdk;
extern crate gtk;
use std::env;
use gdk::enums::*;
use gtk::prelude::*;
use app::App;
use controllers::modules_controller::ModulesController;
use models::modules::Modules;

mod app;
mod controllers;
mod models;

#[allow(unused_variables)]
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

    let mut app = App::new();

    let box_main = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    box_main.pack_start(&app.stack, true, true, 0);
    window.add(&box_main);

    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_min_content_width(1200);
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let modules = ModulesController::get_modules();
    for module in modules {
        let url = format!("{}", module.name);
        let module_and_url = gtk::LinkButton::new_with_label(&url, Some(&module.name));
        module_and_url.set_halign(gtk::Align::Start);

        container.add(&module_and_url);
    }
    app.create_windows("Übersicht Sensoren", container);

    // Construct the StackSwitcher
    for title in &["Sensor 1", "Sensor 2", "Sensor 3", "Einstellungen"] {
        let label = gtk::Label::new(Some(title));
        app.create_windows(&title.to_string(), label);
    }

    // Swipe
    let swipe = gtk::GestureSwipe::new(&app.stack);
    swipe.connect_swipe(move |swipe, swipe_x, swipe_y| {
        // println!("swipe: {:?}", swipe);
        // println!("swipe_x: {:?}", swipe_x);
        // println!("swipe_y: {:?}", swipe_y);
        match swipe_x < 0f64 {
            true  => {
                if swipe_x < -100.0 && swipe_y > - 80.0 {
                    app.next_window();
                }
            },
            false => {
                if swipe_x > 100.0 && swipe_y < 80.0{
                    app.prev_window();
                }
            },
        };
    });

    window.show_all();

    gtk::main();
}
