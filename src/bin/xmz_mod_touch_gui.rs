extern crate clap;
extern crate log;
extern crate xmz_mod_touch_gui;
extern crate env_logger;

use clap::{App, Arg, ArgMatches};
use xmz_mod_touch_gui::XMZModTouchClient;
use xmz_mod_touch_gui::error::*;
use xmz_mod_touch_gui::application;


fn run(matches: &ArgMatches) -> Result<()> {
    let hostname = matches.value_of("hostname").unwrap();

    let client = XMZModTouchClient::new(hostname);

    application::launch(&client)?;

    Ok(())
}

fn main() {
    // Initalisiere Logger (erst nach diesem Aufruf sind `trace!()`, `debug!()` usw. functional)
    env_logger::init().unwrap();

    let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about("Grafische Oberfläche der 'xMZ-Mod-Touch'-Platform")
    .author("Stefan Müller (zzeroo) <s.mueller@it.kls-glt.de>")
    .arg(Arg::with_name("hostname")
        .help("Hostname/ IP Adresse der Server Instanz")
        .long("server")
        .short("s")
        .takes_value(true)
        .required(true)
        .default_value("localhost"))
    .get_matches();

    if let Err(ref e) = run(&matches) {
        println!("error: {}", e);

        ::std::process::exit(1);
    }
}
