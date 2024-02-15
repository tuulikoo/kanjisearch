// web.rs
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde_json::Value;
use kanjisearch::search_kanji_by_strokes;
use kanjisearch::KANJI_DATA;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn find_in_js_array(js_value: JsValue, value: JsValue) -> JsValue {
    //console_log!("js_value in find_in_js_array: {:?}", js_value);

    let target_value = value.as_string().unwrap();

    if let Some(array) = js_value.dyn_into::<Array>().ok() {
        for i in 0..array.length() {
            if let Some(array_item) = array.get(i).as_string() {

                // Deserialize the JSON string into a serde_json::Value
                let parsed_item: Result<Value, _> = serde_json::from_str(&array_item);

                if let Ok(parsed_item) = parsed_item {
                    if let Some(kanji_value) = parsed_item["kanji"].as_str() {
                        if kanji_value == target_value {
                            return value;
                        }
                    }
                }
            }
        }
    }
    JsValue::undefined()
}

#[wasm_bindgen_test]
fn test_search_kanji_by_strokes4() {
    let results = search_kanji_by_strokes(4);
    //console_log!("results in tests4: {:?}", results);
    let string_result = find_in_js_array(results, JsValue::from("友"));
    assert_eq!(string_result, JsValue::from("友"));
}

#[wasm_bindgen_test]
fn test_search_kanji_by_strokes_none() {
    let results = search_kanji_by_strokes(100);

    let string_result = find_in_js_array(results, JsValue::from("友"));
    //console_log!("string_result in tests_none: {:?}", string_result);
    assert_eq!(string_result, JsValue::undefined());
}

#[wasm_bindgen_test]
fn test_kanji_data_columns() {
    // Deserialize JSON string from kanjidata
    let parsed_data: Vec<Value> = serde_json::from_str(crate::KANJI_DATA).unwrap();

    // Assert that the first result in parsed data contains expected columns
    assert!(parsed_data.get(0).map_or(false, |item| {
        item["id"].is_u64() &&
        item["kanji"].is_string() &&
        item["strokes"].is_u64() &&
        item["onreading"].is_string() &&
        item["kunreading"].is_string()
    }));
}