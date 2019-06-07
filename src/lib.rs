mod client_logger;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

#[wasm_bindgen]
pub fn game_logic_entry(worker: web_sys::Worker) {
    client_logger::init_logger();

    info!("hello from game logic wasm");
    worker.post_message(&wasm_bindgen::JsValue::from_str("premsg frm wasm_gLe"))
        .unwrap();
    info!("game logic terminated");
}
