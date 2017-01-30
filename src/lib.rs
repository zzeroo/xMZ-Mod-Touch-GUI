#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! xMZ-Mod-Touch-GUI
//!
//! Grafische Oberfläche der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/zzeroo/xMZ-Mod-Touch-GUI.git

#[macro_use] extern crate log;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate hyper;
extern crate serde_json;
extern crate xmz_server;
extern crate gobject_sys;
extern crate gtk_sys;


pub mod gui {
    pub mod gtk3;
}

pub mod error;

pub use self::error::*;
pub use self::gui::gtk3::*;
