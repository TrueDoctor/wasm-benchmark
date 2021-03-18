#![allow(unused_attributes)]

mod binds; use binds::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(f: js_sys::Function, ms: u32);
}

fn sleep_impl(ms: u32) -> js_sys::Promise {
    js_sys::Promise::new(&mut (|resolve, _| {setTimeout(resolve, ms);}))
}

pub async fn sleep(ms: u32) {
    use wasm_bindgen_futures::futures_0_3::JsFuture;
    let promise = sleep_impl(ms);
    JsFuture::from(promise).await.unwrap();
}

async fn main(ms: u32, rep: u32) {
    tlog("before main loop: ");
    for _ in 0..rep {
        sleep(ms).await;
    }
    tlog("after main loop: ");
}

#[wasm_bindgen]
pub fn entry(ms: u32, rep: u32) {
    use wasm_bindgen_futures::futures_0_3::future_to_promise;
    future_to_promise(async move {
        main(ms, rep).await;
        Ok(wasm_bindgen::JsValue::UNDEFINED)
    });
}
