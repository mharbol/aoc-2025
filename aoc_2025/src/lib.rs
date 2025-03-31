use wasm_bindgen::prelude::wasm_bindgen;

pub mod solution;

include!(concat!(env!("OUT_DIR"), "/wasm_src.rs"));
