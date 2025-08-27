use std::env;

pub enum ArgParseStatus {
    Ok(u32, u32, Vec<String>),
    Err(String),
    Help,
}

#[allow(dead_code)]
pub struct ArgParse {
    args: Vec<String>,
}

impl ArgParse {
    pub fn new() -> ArgParse {
        ArgParse {
            args: env::args().collect(),
        }
    }

    pub fn parse(self: &ArgParse) -> ArgParseStatus {
        // TODO
        ArgParseStatus::Help
    }

    pub fn show_help() {
        // TODO
    }
}
