use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wasm_src.rs");
    fs::write(&dest_path, add_wasm_targets()).unwrap();
    // println!("cargo::rerun-if-changed=solution/mod.rs");
}

fn add_wasm_targets() -> String {
    fs::read_dir("./src/solution/")
        .unwrap()
        .into_iter()
        .map(|file| file.unwrap())
        .map(|file| file.file_name().to_str().unwrap().to_string())
        .filter(|file_name| {
            file_name.starts_with("day") && file_name.ends_with(".rs") && file_name.len() == 8
        })
        .map(|file_name| file_name[0..5].to_string())
        .map(|day_name| {
            format!(
                "
#[wasm_bindgen]
pub fn {}_part1(lines: Vec<String>) -> String {{
    solution::{}::part1(&lines)
}}

#[wasm_bindgen]
pub fn {}_part2(lines: Vec<String>) -> String {{
    solution::{}::part2(&lines)
}}
",
                day_name, day_name, day_name, day_name
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
