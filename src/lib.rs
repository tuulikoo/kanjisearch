use std::include_str;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use serde_json;
use js_sys::Array;
use serde_wasm_bindgen::to_value;
use serde_json::Value;






static KANJI_DATA: &str = include_str!("kanjidata.json");
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Deserialize, Serialize)]
struct Kanji {
    id: u32,
    kanji: String,
    strokes: u32,
    onreading: String,
    kunreading: String,
}

#[wasm_bindgen]
pub fn search_kanji_by_strokes(strokes: u32) -> JsValue {
    let parsed_data: Vec<Kanji> = serde_json::from_str(KANJI_DATA).unwrap();
    let results = parsed_data
        .iter()
        .filter(|kanji| kanji.strokes == strokes)
        .map(|kanji| serde_json::to_string(&kanji).unwrap())
        .collect::<Vec<_>>();

    let js_results = to_value(&results).unwrap();

    js_results
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    macro_rules! console_log {
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }

    #[wasm_bindgen]
pub fn find_in_js_array(js_value: JsValue, value: JsValue) -> JsValue {
    console_log!("js_value in find_in_js_array: {:?}", js_value);

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
        let parsed_data: Vec<Value> = serde_json::from_str(KANJI_DATA).unwrap();

        // Assert that first result in parsed data contains expected columns
        assert!(parsed_data.get(0).map_or(false, |item| {
            item["id"].is_u64() &&
            item["kanji"].is_string() &&
            item["strokes"].is_u64() &&
            item["onreading"].is_string() &&
            item["kunreading"].is_string()
        }));
    }

}
