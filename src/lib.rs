use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_and_run(code: &str) -> String {
    // In real case, you'd use a Rust interpreter or mini compiler engine.
    // For now, just mock the behavior.
    if code.contains("fn main") {
        return "Code compiled successfully! Output: Hello from Rust!".into();
    } else {
        return "Compilation error: missing fn main".into();
    }
}