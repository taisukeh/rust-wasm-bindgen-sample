#![feature(use_extern_macros)]

extern crate web_sys;
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn timed(callback: &js_sys::Function) -> f64 {
    let then = js_sys::Date::now();

    let args = js_sys::Array::new();
    args.push(&JsValue::from("hoge"));
    args.push(&JsValue::from("moe"));
    callback.apply(&JsValue::null(), &args).unwrap();

    let now = js_sys::Date::now();
    log(&format!("now - then = {}", now - then));
    now - then
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    log(&format!("console log4???? {}", name));
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello??????, {}!", name));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
