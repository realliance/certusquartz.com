#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {

        mod systems;
        mod app;

        use wasm_bindgen::prelude::*;
        use std::panic;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            app::run();

            Ok(())
        }
    }
}
