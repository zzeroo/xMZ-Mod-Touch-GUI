use error::*;
use hyper::{self, Client};
use serde_json;
use std::io::Read;
use xmz_mod_touch_server::xmz_mod_touch_server::XMZModTouchServer;


#[derive(Debug)]
pub struct XMZModTouchClient {
    client: Client,
}

impl XMZModTouchClient {
    pub fn new() -> Self {
        XMZModTouchClient {
            client: Client::new(),
        }
    }

    /// # Examples
    ///
    /// ```
    /// assert!(true);
    /// ```
    pub fn get_server(&self) -> Result<XMZModTouchServer> {
        let mut server_return = XMZModTouchServer::new();

        self.client.get("http://localhost:3000/").send().map(|mut res| {
            assert_eq!(res.status, hyper::Ok);
            let mut s = String::new();
            res.read_to_string(&mut s);
            if let Ok(server) = serde_json::from_str(&s) {
                server_return = server;
            }
        });
        
        Ok(server_return)
    }
}

