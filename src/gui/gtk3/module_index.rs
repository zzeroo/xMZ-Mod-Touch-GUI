use gtk;
use gtk::prelude::*;
use error::*;
use xmz_server::server::{Server};
use std::sync::{Arc, Mutex};

pub fn setup(builder: &gtk::Builder, window: &gtk::Window, server: Arc<Mutex<Server>>) -> Result<()> {

    Ok(())
}
