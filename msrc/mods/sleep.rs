#![allow(unused_attributes)]
#![feature(async_await)]
#![feature(await_macro)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn sleep_impl(ms: u32) -> js_sys::Promise;
}

pub async fn sleep(ms: u32) {
    use wasm_bindgen_futures::futures_0_3::JsFuture;
    let promise = sleep_impl(ms);
    JsFuture::from(promise).await.unwrap();
}
