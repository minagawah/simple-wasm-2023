use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// `wee_alloc` is a tiny allocator designed for WebAssembly
// that has a (pre-compression) code-size footprint of only
// a single kilobyte. When the `wee_alloc` feature is enabled,
// this uses `wee_alloc` as the global allocator. If you don't
// want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    web_sys::console::log_1(&JsValue::from_str("debug"));

    #[cfg(not(debug_assertions))]
    web_sys::console::log_1(&JsValue::from_str("release"));

    let window = web_sys::window().expect("No window");
    let document = window.document().expect("No document");
    // let body = document.body().expect("No body");

    let div = document.create_element("div")?;
    div.set_text_content(Some("It is working!"));

    let wrapper = document.query_selector("#wrapper")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    // wrapper.set_inner_text(&"hello");

    wrapper.append_child(&div)?;

    Ok(())
}
