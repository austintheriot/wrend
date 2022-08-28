use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn strings_to_js_array<T: AsRef<str>>(strings: &[T]) -> Array {
    let vec_strings: Vec<JsValue> = strings
        .iter()
        .map(|s| {
            let s = s.as_ref();
            JsValue::from_str(s)
        })
        .collect();
    Array::from_iter(vec_strings)
}

pub fn js_array_to_strings(array: Array) -> Vec<String> {
    js_sys::try_iter(array.as_ref())
        .expect("`js_array_to_strings` should be passed an array of strings")
        .expect("`js_array_to_strings` should be passed an array of strings")
        .into_iter()
        .map(|el| {
            JsValue::as_string(&el.expect(
                "Each element in the array passed to `js_array_to_strings` should be a string",
            ))
            .unwrap()
        })
        .collect()
}
