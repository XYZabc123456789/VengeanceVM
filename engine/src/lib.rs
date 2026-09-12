use wasm_bindgen::prelude::*;
use web_sys::console::*;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::log_1(&wasm_bindgen::JsValue::from_str(&format!($($arg)*)));
    };
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn add(left: u128, right: u128) -> u128 {
    println!("Hello World");
    left + right 
}