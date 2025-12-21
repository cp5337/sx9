use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn firefly_calculate(input: i32) -> String {
    format!("🔥 Firefly Core v1.0 calculated power: {}", input * 42)
}

// Pure Rust function for Axum (optional, or it can use the mapped one)
pub fn native_calculate(input: i32) -> i32 {
    input * 42
}
