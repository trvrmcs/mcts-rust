/*
    Dummy file so we can build & cache dependencies faster in docker
*/

use wasm_bindgen::prelude::*;
 
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
 
