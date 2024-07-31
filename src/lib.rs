mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlVideoElement, MediaStream, MediaStreamConstraints};

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
    let navigator = window.navigator();

    // Add video feed
    let video_output = document.create_element("video")?;
    video_output.set_attribute("width", "720")?;
    video_output.set_attribute("autoplay", "")?;
    body.append_child(&video_output)?;

    // line break
    body.append_child(&document.create_element("br").unwrap())?;

    // Add hello world button
    let button = document.create_element("button")?;
    let rust_greet = Closure::<dyn FnMut()>::new(|| {
        greet();
    });
    button.set_inner_html("Click me");
    button
        .dyn_ref::<HtmlElement>()
        .expect("Buttons are HtmlElement")
        .set_onclick(Some(rust_greet.as_ref().unchecked_ref()));
    rust_greet.forget();
    body.append_child(&button)?;

    // Make Camera output on Stream
    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from_bool(true));
    let move_stream_to_video_tag = Closure::<dyn FnMut(JsValue)>::new(move |stream| {
        video_output
            .dyn_ref::<HtmlVideoElement>()
            .expect("It is a video element")
            .set_src_object(Some(&MediaStream::from(stream)));
    });
    let _ = navigator
        .media_devices()
        .expect(
            "We need permissions and site has to be acceced over HTTPS or through local network",
        )
        .get_user_media_with_constraints(&constraints)
        .expect("Second unwrap")
        .then(&move_stream_to_video_tag);
    move_stream_to_video_tag.forget();

    Ok(())
}
