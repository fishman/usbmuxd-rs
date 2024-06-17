//include!(concat!(env!("OUT_DIR"), "/bindings_/usr/include/usbmuxd_h.rs"));
//include!(concat!(env!("OUT_DIR"), "/bindings_/usr/include/usbmuxd-proto_h.rs"));

use log::info;

fn main() {
    env_logger::init();

    info!(target: "usbmuxd", "");

    println!("Hello, world!");
}
