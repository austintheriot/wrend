use core::hash::Hash;
use std::collections::HashMap;

use js_sys::{Array, Map};
use wasm_bindgen::{JsCast, JsValue};

use crate::StringArray;

pub fn js_array_to_vec<A: AsRef<Array>, T: From<JsValue>>(array: &A) -> Vec<T> {
    js_sys::try_iter(array.as_ref())
        .expect("`js_array_to_vec` should be passed an Array that is iterable from JavaScript")
        .unwrap()
        .into_iter()
        .map(|el| el.unwrap().into())
        .collect()
}

pub fn strings_to_js_array<T: AsRef<str>>(strings: &[T]) -> StringArray {
    let vec_strings: Vec<JsValue> = strings
        .iter()
        .map(|s| {
            let s = s.as_ref();
            JsValue::from_str(s)
        })
        .collect();

    Array::from_iter(vec_strings)
        .dyn_into()
        .expect("Should be able to convert Array of strings into a StringArray")
}

pub fn js_array_to_vec_strings<A: AsRef<Array>>(array: &A) -> Vec<String> {
    js_sys::try_iter(array.as_ref())
        .expect("`js_array_to_vec_strings` should be passed an array of strings")
        .expect("`js_array_to_vec_strings` should be passed an array of strings")
        .into_iter()
        .map(|el| {
            JsValue::as_string(&el.expect(
                "Each element in the array passed to `js_array_to_vec_strings` should be a string",
            ))
            .unwrap()
        })
        .collect()
}

pub fn hash_map_to_js_map<K: Hash + AsRef<str>, V: JsCast>(hash_hap: &HashMap<K, V>) -> Map {
    let map = Map::new();
    for (key, value) in hash_hap.iter() {
        let key = key.as_ref();
        map.set(&JsValue::from_str(key), value.as_ref());
    }
    map
}
