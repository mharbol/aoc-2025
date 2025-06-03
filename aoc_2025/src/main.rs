use aoc_2025::cmd;

fn main() {
    match cmd::argparse::parse_args() {
        Ok((day, part, lines)) => println!("{}", run_problem(day, part, &lines)),
        Err(msg) => println!("{}", msg),
    };
}

fn run_problem(day: u32, part: u32, lines: &Vec<String>) -> String {
    match part {
        1 => cmd::get_day(day).unwrap().0(lines),
        2 => cmd::get_day(day).unwrap().1(lines),
        _ => panic!(),
    }
}
