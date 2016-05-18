extern crate xmz_mod_touch_gui;
extern crate gtk;

use xmz_mod_touch_gui::app::App;


fn setup() {
    gtk::init();
}

fn tear_down() {
}

#[test]
fn test_get_window_index() {
    setup();
    let mut app = App::new();
    for w in &["Fenster 1", "Fenster 2", "Einstellungen"] {
        app.create_windows(&w.to_string());
    }
    assert_eq!(&app.get_window_index("Einstellungen"), &Some(2));
    tear_down();
}

#[test]
#[ignore]
fn test_get_next_window() {
    setup();
    let mut app = App::new();
    for w in &["Fenster 1", "Fenster 2", "Einstellungen"] {
        app.create_windows(&w.to_string());
    }
    assert_eq!(&app.get_window_index("Einstellungen"), &Some(2));
    tear_down();
}

#[test]
#[ignore]
fn test_get_prev_window() {
    unimplemented!();
}
