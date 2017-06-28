#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! xMZ-Mod-Touch-GUI
//!
//! Grafische Oberfläche der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/zzeroo/xMZ-Mod-Touch-GUI.git

// `error_chain!` can recurse deeply(3)
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate hyper;
extern crate serde_json;
extern crate xmz_mod_touch_server;

// Die Reihenfolge bei #[macro_use] ist wichtig!!! Immer über die anderen Includes stellen
#[macro_use] mod macros;
pub mod api_client;
pub mod application;
pub mod errors;

pub use self::api_client::ApiClient;
pub use self::errors::*;
