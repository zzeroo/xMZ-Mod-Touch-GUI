#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]

#[macro_use] extern crate log;
extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate hyper;
extern crate serde_json;


// pub mod gui {
//     pub mod gtk3;
// }

// pub mod sysinfo;
// /// Fehlerhandling mit dem error-chain crate
pub mod error;

pub use self::error::*;
