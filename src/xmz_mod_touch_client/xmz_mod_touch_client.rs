use error::*;
use hyper::{self, Client};
use serde_json;
use std::io::Read;
use std::sync::Mutex;
use xmz_mod_touch_server::xmz_mod_touch_server::XMZModTouchServer;


#[derive(Debug)]
pub struct XMZModTouchClient<'a> {
    hyperclient: Client,
    server_data: Mutex<XMZModTouchServer>,
    hostname: &'a str,
}

impl<'a> XMZModTouchClient<'a> {
    /// Erzeugt einen neuen Clienten
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::XMZModTouchClient;
    /// let client = XMZModTouchClient::new("localhost");
    ///
    /// assert_eq!(client.get_hostname(), "localhost");
    /// ```
    pub fn new(hostname: &'a str) -> Self {
        XMZModTouchClient {
            hyperclient: Client::new(),
            server_data: Mutex::new(XMZModTouchServer::new()),
            hostname: hostname,
        }
    }

    /// Get hostname
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::XMZModTouchClient;
    /// let client = XMZModTouchClient::new("localhost");
    ///
    /// assert_eq!(client.get_hostname(), "localhost");
    /// ```
    pub fn get_hostname(&self) -> &str {
        self.hostname
    }

    /// Get server_data
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::XMZModTouchClient;
    /// let client = XMZModTouchClient::new("localhost");
    ///
    /// client.get_server_data().lock().unwrap().get_version();
    /// ```
    pub fn get_server_data(&self) -> &Mutex<XMZModTouchServer> {
        &self.server_data
    }

    ///
    /// # Examples
    ///
    /// ```
    /// assert!(true);
    /// ```


    /// Update der Server Daten
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::XMZModTouchClient;
    /// let client = XMZModTouchClient::new("localhost");
    ///
    /// client.update_server_data();
    /// ```
    pub fn update_server_data(&self) {
        // construct a remote url
        let remote_url = format!("http://{}:3000/api/v1", self.hostname);

        // Ask via hypers get method
        if let Ok(mut response) = self.hyperclient.get( &remote_url ).send() {
            // Result string
            let mut s = String::new();

            match response.read_to_string(&mut s) {
                Err(e) => println!("Error: {}", e),
                Ok(_size) => {
                    if let Ok(mut server_data) = self.server_data.lock() {
                        if let Ok(server) = serde_json::from_str::<XMZModTouchServer>(&s) {
                            *server_data = server;
                        }
                    }
                }
            }
        };
    }
}
