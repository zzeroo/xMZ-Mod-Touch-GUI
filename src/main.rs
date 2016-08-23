#![feature(stmt_expr_attributes)]

extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;


mod gui {
    pub mod gtk3;
}

pub mod sysinfo;

fn main() {
    gui::gtk3::launch();
}
