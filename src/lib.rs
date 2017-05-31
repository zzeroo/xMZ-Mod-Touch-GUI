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
extern crate gobject_sys;
extern crate gtk_sys;
extern crate gtk;
extern crate hyper;
extern crate serde_json;
extern crate xmz_mod_touch_server;

// Die Reihenfolge bei #[macro_use] ist wichtig!!! Immer über die anderen Includes stellen
#[macro_use] mod macros;
pub mod application;
pub mod error;
pub mod xmz_mod_touch_client;

pub use self::xmz_mod_touch_client::XMZModTouchClient;
