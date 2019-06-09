mod binds; use binds::*;

struct Context {
    tick_nr: u32,
}

static mut CONTEXT: Context = Context { tick_nr: 0 };

fn get_context() -> &'static mut Context {
    unsafe { &mut CONTEXT }
}

#[wasm_bindgen]
pub fn myloop(maxv: u32) -> bool {
    let context = get_context();
    if context.tick_nr == 0 {
        tlog("tick_nr 0: ");
    } else if context.tick_nr == maxv - 1 {
        tlog(&format!("tick_nr {}: ", context.tick_nr));
    }
    context.tick_nr += 1;
    context.tick_nr < maxv
}

#[wasm_bindgen(start)]
pub fn entry() {
}
