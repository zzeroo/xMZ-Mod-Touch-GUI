#![feature(stmt_expr_attributes)]


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
