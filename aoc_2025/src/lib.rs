use wasm_bindgen::prelude::wasm_bindgen;

pub mod cmd;
pub mod solution;

include!(concat!(env!("OUT_DIR"), "/wasm_src.rs"));
