use error::*;
use gdk::enums::key;
use gtk;
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
        let css_gui = include_str!("gui.css");
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
    
    let glade_str = include_str!("main.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str)?;

    let window_main: gtk::Window = builder.get_object("window_main").unwrap();
    window_main_setup(&window_main)?;
    
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

    window_main.show_all();

    gtk::main();

    Ok(())
}