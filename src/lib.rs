
use std::include_str;
use serde::{Deserialize, Serialize};use wasm_bindgen::prelude::*;
use serde_json;
use serde_wasm_bindgen::to_value;





pub static KANJI_DATA: &str = include_str!("kanjidata.json");
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
/*macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}*/

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