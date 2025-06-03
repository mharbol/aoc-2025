use wasm_bindgen::prelude::wasm_bindgen;

pub mod solution;
pub mod cmd;

include!(concat!(env!("OUT_DIR"), "/wasm_src.rs"));
