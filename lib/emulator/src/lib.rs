use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn set_input_state(input_state: &[u8]) {
    log(&format!("Hello, {:?}!", input_state));
}



// i/o
// - Gamepad API and custom Keybaord API in js
//   Store input state in Memory Buffer for lookup from webassambly
// - OffscreenCanvas no work in safari so.... 
//   and SharedArrayBuffer not supported so..... 
//   copy buffer I guess or like WebAssembly.Memory or something 




