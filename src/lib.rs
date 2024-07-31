mod utils;

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, bachelor-card-game-visual!");
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let button = document.create_element("button")?;
    let rust_greet = Closure::<dyn FnMut()>::new( || {
        greet();
    });
    button.set_inner_html("Click me");
    button.dyn_ref::<HtmlElement>()
        .expect("Buttons are HtmlElement")
        .set_onclick(Some(rust_greet.as_ref().unchecked_ref()));
    rust_greet.forget();
    body.append_child(&button)?;
    Ok(())
}
