#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]
#![recursion_limit = "1024"]

#[macro_use]extern crate error_chain;
#[macro_use] extern crate log;
extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;


pub mod gui {
    pub mod gtk3;
}

pub mod sysinfo;
/// Fehlerhandling mit dem error-chain crate
mod errors;
