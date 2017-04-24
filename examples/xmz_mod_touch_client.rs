extern crate xmz_mod_touch_gui;

use xmz_mod_touch_gui::XMZModTouchClient;


fn main() {
    let client = XMZModTouchClient::new();

    println!("client {:#?}", client);
    println!("client.get_server() {:#?}", client.get_server());
}


