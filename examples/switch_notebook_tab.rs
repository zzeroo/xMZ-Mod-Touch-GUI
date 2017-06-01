//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file
extern crate gdk;
extern crate gtk;


mod example {
    use gdk::DisplayExt;
    use gdk::ScreenExt;
    use gtk;
    use gtk::{Builder, Button, MessageDialog, Window};
    use gtk::prelude::*;


    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("switch_notebook_tab.glade");
        let builder = Builder::new();
        builder.add_from_string(&glade_src);

        let window: Window = builder.get_object("window_main").unwrap();
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
        let css_gui = include_str!("switch_notebook_tab.css");
        match css_style_provider.load_from_data(css_gui) {
            Ok(_) => {
                gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
                Err(e) => { println!("Error: css_style_provider.load_from_data() failed: {}", e) }
            }
        }

        let notebook: gtk::Notebook = builder.get_object("notebook").unwrap();
        let mut seiten_nummer = 0;

        gtk::timeout_add(3000, move || {
            if seiten_nummer < 3 {
                seiten_nummer += 1;
            } else {
                seiten_nummer = 0;
            }
            notebook.set_current_page(Some(seiten_nummer));

            gtk::Continue(true)
        });



        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        gtk::main();
    }
}

fn main() {
    example::main()
}
