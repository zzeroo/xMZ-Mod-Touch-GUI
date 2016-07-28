use gtk::{self};
use gtk::prelude::*;
use gdk::enums::key;


// Basic Setup des Windows
fn window_setup(window: &gtk::Window) {
    let window_title = format!("{} {}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    // window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    match option_env!("XMZ_HARDWARE") {
        Some(_) => { window.fullscreen(); }
        None    => {}
    }
}


pub fn launch() {
    // TODO: Programm Name aus env!("CARGO_PKG_NAME") nutzen
    gtk::init().unwrap_or_else(|_| panic!(format!("{}: GTK konnte nicht initalisiert werden.", env!("CARGO_PKG_NAME"))) );

    // Initialisiere alle Widgets die das Programm nutzt aus dem Glade File.
    let builder = gtk::Builder::new_from_string(include_str!("interface.glade"));
    let window: gtk::Window                         = builder.get_object("main_window").unwrap();
    let main_window_stack: gtk::Stack               = builder.get_object("main_window_stack").unwrap();
    let info_bar: gtk::InfoBar                      = builder.get_object("info_bar").unwrap();


    // Hide info_bar
    info_bar.connect_response(move |foo, _| {
        println!("{:?}", foo);
        foo.hide()
    });



    window_setup(&window);
    window.show_all();

    // Beende Programm wenn das Fenster geschlossen wurde
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Registriert die Esc Taste mit main_quit() (schliesst also das Fenster mit der Esc Taste),
    // aber nur in DEBUG Builds. Wird das Programm mit `--release` übersetzt, funktioniert dies nicht.
    #[cfg(debug_assertions)]
    window.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() { gtk::main_quit() }
        Inhibit(false)
    }) ;

    gtk::main();
}
