use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u128, right: u128) -> u128 {
    left + right 
}