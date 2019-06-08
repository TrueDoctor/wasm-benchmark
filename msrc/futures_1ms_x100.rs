#![allow(unused_attributes)]
#![feature(async_await)]
#![feature(await_macro)]

mod binds; use binds::*;
mod sleep; use sleep::*;

async fn main() {
    tlog("before main loop: ");
    for _ in 0..1000 {
        await!(sleep(1));
    }
    tlog("after main loop: ");
}

#[wasm_bindgen(start)]
pub fn entry() {
    use wasm_bindgen_futures::futures_0_3::future_to_promise;
    future_to_promise(async move {
        await!(main());
        Ok(wasm_bindgen::JsValue::UNDEFINED)
    });
}
