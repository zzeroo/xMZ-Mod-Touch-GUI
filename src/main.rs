#[macro_use] extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate serde_json;
extern crate xmz_mod_touch_gui;
extern crate xmz_server;

use hyper::Client;
use std::io::Read;
use std::sync::{Arc, Mutex};
use xmz_mod_touch_gui::error::*;
use xmz_server::server::{Server};
use std::thread;
use std::time::Duration;


fn poll_server_web_interface(server: Arc<Mutex<xmz_server::Server>>) -> Result<()> {
    let thread_poll = thread::spawn(move || {
        let client = Client::new(); // hyper::Client;
        let mut res = match client.get("http://localhost:3000/").send() {
            Err(err) => { debug!("{:?}", err) }
            Ok(mut res) => {
                assert_eq!(res.status, hyper::Ok);

                let mut s = String::new();
                match res.read_to_string(&mut s) {
                    Err(err) => { debug!("{:?}", err) }
                    Ok(_) => {
                        {
                            let mut server = server.lock().unwrap();
                            *server = serde_json::from_str(&s).unwrap();
                        }
                    }
                }
            }
        };
    }).join();

    Ok(())
}

fn print_some(server: Arc<Mutex<Server>>) -> Result<()> {
    let _ = thread::spawn(move || {
        let server = server.lock().unwrap();

        for kombisensor in server.get_kombisensors().iter() {
            println!("{}", kombisensor.get_modbus_slave_id());
            for sensor in kombisensor.get_sensors().iter() {
                println!("\t{}\t{}", sensor.get_sensor_type(), sensor.get_adc_value());
            }
        }
    }).join();

    Ok(())
}

fn run() -> Result<()> {
    let server = Arc::new(Mutex::new(xmz_server::Server::new()));

    loop {
        // Update Server struct via http
        poll_server_web_interface(server.clone())?;

        // // print some
        print_some(server.clone())?;

        // 1Sekunden Takt
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}


fn main() {
    // Initialisiere den Logger (erst nach diesem sind `trace!()`, `debug!()` usw funktional)
    env_logger::init().unwrap();

    if let Err(ref e) = run() {
        println!("error: {}", e);

        // ::std::process::exit(1);
    }
}
