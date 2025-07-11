use crate::cmd;
use std::env;

// TODO make this a better command line parser
// Right now just accepts args as <day> <part> <file-path>
pub fn parse_args() -> Result<(u32, u32, Vec<String>), String> {
    let args: Vec<String> = env::args().collect();
    Ok((
        args[1].parse().map_err(|_| String::from("Bad day"))?,
        args[2].parse().map_err(|_| String::from("Bad part"))?,
        cmd::get_file_lines(args[3].as_str()).map_err(|_| String::from("Bad file"))?,
    ))
}

pub enum ArgParseStatus {
    Ok(u32, u32, String),
    Err(String),
    Help,
}

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
}
