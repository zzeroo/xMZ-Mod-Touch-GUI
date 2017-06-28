use errors::*;
use hyper::{self, Client as HyperClient};
use serde_json;
use std::io::Read;
use xmz_mod_touch_server::Server;


#[derive(Debug)]
pub struct ApiClient {
    hyperclient: HyperClient,
    server_data: Server,
    hostname: String,
}

impl ApiClient {
    /// Erzeugt einen neuen Clienten
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost".to_string());
    ///
    /// assert_eq!(client.get_hostname(), "localhost".to_string());
    /// ```
    pub fn new(hostname: String) -> Self {
        ApiClient {
            hyperclient: HyperClient::new(),
            server_data: Server::new(),
            hostname,
        }
    }

    /// Get hostname
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost".to_string());
    ///
    /// assert_eq!(client.get_hostname(), "localhost".to_string());
    /// ```
    pub fn get_hostname(&self) -> String {
        self.hostname.clone()
    }

    /// Get server_data
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_mod_touch_gui::ApiClient;
    /// let client = ApiClient::new("localhost".to_string());
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
    /// let mut client = ApiClient::new("localhost".to_string());
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
