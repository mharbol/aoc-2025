use std::process::ExitCode;

use aoc_2025::cmd;

fn main() -> ExitCode {
    match cmd::argparse::parse_args() {
        Ok((day, part, lines)) => {
            println!("{}", run_problem(day, part, &lines));
            ExitCode::SUCCESS
        }
        Err(msg) => {
            eprintln!("{}", msg);
            ExitCode::FAILURE
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
