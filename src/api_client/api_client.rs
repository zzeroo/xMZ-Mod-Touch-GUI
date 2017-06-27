use errors::*;
use hyper::{self, Client as HyperClient};
use serde_json;
use std::io::Read;
use xmz_mod_touch_server::Server;


#[derive(Debug)]
pub struct ApiClient<'a> {
    hyperclient: HyperClient,
    server_data: Server,
    hostname: &'a str,
}

impl<'a> ApiClient<'a> {
    /// Erzeugt einen neuen Clienten
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost");
    ///
    /// assert_eq!(client.get_hostname(), "localhost");
    /// ```
    pub fn new(hostname: &'a str) -> Self {
        ApiClient {
            hyperclient: HyperClient::new(),
            server_data: Server::new(),
            hostname: hostname,
        }
    }

    /// Get hostname
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost");
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
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost");
    ///
    /// client.get_server_data().get_version();
    /// ```
    pub fn get_server_data(&self) -> &Server {
        &self.server_data
    }

    /// Update der Server Daten
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let mut client = ApiClient::new("localhost");
    ///
    /// client.update_server_data();
    /// ```
    pub fn update_server_data(&mut self) {
        // construct a remote url
        let remote_url = format!("http://{}:3000/api/v1", self.hostname);

        // Ask via hypers get method
        if let Ok(mut response) = self.hyperclient.get( &remote_url ).send() {
            // Result string
            let mut s = String::new();

            match response.read_to_string(&mut s) {
                Err(e) => println!("Error: {}", e),
                Ok(_size) => {
                    if let Ok(server) = serde_json::from_str::<Server>(&s) {
                        self.server_data = server;
                    }
                }
            }
        };
    }
}
