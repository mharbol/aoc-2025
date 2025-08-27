use std::process::ExitCode;

use aoc_2025::cmd::{self, argparse::ArgParse, argparse::ArgParseStatus};

fn main() -> ExitCode {
    match ArgParse::new().parse() {
        ArgParseStatus::Ok(day, part, lines) => {
            println!("{}", run_problem(day, part, &lines));
            ExitCode::SUCCESS
        }
        ArgParseStatus::Err(msg) => {
            eprintln!("{}", msg);
            ExitCode::FAILURE
        }
        ArgParseStatus::Help => {
            ArgParse::show_help();
            ExitCode::SUCCESS
        }
    }
}

fn run_problem(day: u32, part: u32, lines: &Vec<String>) -> String {
    match part {
        1 => cmd::get_day(day).unwrap().0(lines),
        2 => cmd::get_day(day).unwrap().1(lines),
        _ => panic!(),
    }
}
