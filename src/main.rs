extern crate gdk;
extern crate gtk;
extern crate pango;
use app::App;
use controllers::modules_controller::ModulesController;
use gdk::enums::*;
use gtk::prelude::*;
use models::modules::Modules;
use pango::*;
use std::env;
mod app;
mod controllers;
mod models;

macro_rules! color {
    (white) => (gdk::RGBA{red: 1f64, green: 1f64, blue: 1f64, alpha: 1f64});
    (black) => (gdk::RGBA{red: 0f64, green: 0f64, blue: 0f64, alpha: 0f64});
    (green) => (gdk::RGBA{red: 0.2f64, blue: 0.2f64, green: 0.5f64, alpha: 1f64});
}

#[allow(unused_variables)]
fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initalize GTK."));
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    // Window properties
    window.set_title("Stack switcher test");
    window.set_default_size(1024, 600);


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
    window.add(&app.stack);

    // Constuct Module/ Sensor List
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_min_content_width(1024);
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.override_background_color(gtk::StateFlags::empty(), &color!(green));
    let modules = ModulesController::get_modules();
    for module in modules {
        let mut bold = pango::FontDescription::new();
        bold.set_weight(pango::Weight::Heavy);

        let url = format!("{}", module.name);
        let module_and_url = gtk::LinkButton::new_with_label(&url, Some(&module.name));
        // module_and_url.set_size_request(1024, -1);
        module_and_url.set_halign(gtk::Align::Start);
        module_and_url.override_font(&bold);
        module_and_url.override_background_color(gtk::StateFlags::empty(), &color!(green));
        module_and_url.override_color(gtk::StateFlags::empty(), &color!(white));

        container.pack_start(&module_and_url, true, true, 0);

        for sensor in module.get_sensors() {
            let sensor_details = gtk::TextView::new();
            sensor_details.set_sensitive(false);
            sensor_details.set_halign(gtk::Align::Start);
            sensor_details.set_hexpand(true);
            sensor_details.set_left_margin(10);
            sensor_details.set_right_margin(10);
            sensor_details.set_editable(false);
            sensor_details.get_buffer().unwrap().set_text(&format!("{}: {}", &sensor.name, &sensor.adc_value));
            sensor_details.set_size_request(1024, -1);
            sensor_details.set_monospace(true);

            container.pack_start(&sensor_details, true, true, 0);
        }
        container.add(&gtk::Separator::new(gtk::Orientation::Horizontal));

    }
    scrolled_window.add(&container);
    app.create_windows("Übersicht Sensoren", scrolled_window);

    // Construct the Stack
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
