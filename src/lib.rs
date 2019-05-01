use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    return format!(r#"
Hello visitor: {}<br/>
If you see this, WebAssembly is running correctly in your browser!"#, name);
}
