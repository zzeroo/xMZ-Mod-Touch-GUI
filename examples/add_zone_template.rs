#[macro_use] extern crate log;
extern crate gdk;
extern crate glib;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate gtk;
extern crate hyper;
extern crate serde_json;
extern crate xmz_mod_touch_server;

#[macro_use] extern crate xmz_mod_touch_gui;

use xmz_mod_touch_gui::error::*;
use gdk::enums::key;
use gtk::prelude::*;


fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));

    window.set_title(&window_title);
    window.set_default_size(1024, 600);

    if let Some(display) = window.get_display() {
        let screen = display.get_screen(0);
        screen.set_resolution(180.0);

        // CSS Datei einbinden
        let css_style_provider = gtk::CssProvider::new();
        let css_gui = include_str!("add_zone_template.css");
        match css_style_provider.load_from_data(css_gui) {
            Ok(_) => {
                gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
            Err(e) => { error!("Error: css_style_provider.load_from_data() failed: {}", e) }
        }
    }

    #[cfg(not(feature = "development"))]
    window.fullscreen();

    Ok(())
}

pub fn launch() -> Result<()> {
    if gtk::init().is_err() {
        error!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }

    // Glade Files einbinden
    // let glade_main_str = include_str!("add_zone_template.glade");
    let glade_notebook_str = include_str!("add_zone_template_notebook.glade");
    let glade_main_str = include_str!("add_zone_template.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_main_str)?;
    builder.add_from_string(&glade_notebook_str)?;

    let window_main: gtk::Window = build!(builder, "window_main");
    window_main_setup(&window_main)?;

    let mut notebook_main: gtk::Notebook = build!(builder, "notebook_main");
    for i in 0..3 {
        let box_notebook_zone: gtk::Box = build!(builder, "box_notebook_zone");
        let title = format!("Zone {}", &i);
        let label = gtk::Label::new(title.as_ref());
        &notebook_main.insert_page(&box_notebook_zone, Some( &label ), Some(&i + 1 as u32) );
    }


    #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Test timer

    window_main.show_all();

    gtk::main();

    Ok(())
}


fn main() {
    if let Err(err) = launch() {
        println!("Error: {}", err);
    }
}
