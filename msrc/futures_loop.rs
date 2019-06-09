#![allow(unused_attributes)]
#![feature(async_await)]
#![feature(await_macro)]

mod binds; use binds::*;
mod sleep; use sleep::*;

async fn main(ms: u32, rep: u32) {
    tlog("before main loop: ");
    for _ in 0..rep {
        await!(sleep(ms));
    }
    tlog("after main loop: ");
}

#[wasm_bindgen]
pub fn entry(ms: u32, rep: u32) {
    use wasm_bindgen_futures::futures_0_3::future_to_promise;
    future_to_promise(async move {
        await!(main(ms, rep));
        Ok(wasm_bindgen::JsValue::UNDEFINED)
    });
}
