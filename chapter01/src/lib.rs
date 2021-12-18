
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn echo() -> String {
     format!("{}","鲤鱼纹")
}