use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, HtmlAnchorElement};

pub fn save_image(canvas: &HtmlCanvasElement) {
    let data_url = canvas
        .to_data_url()
        .unwrap()
        .replace("image/png", "image/octet-stream");
    let a = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&data_url);
    a.set_download("image.png");
    a.click();
}
