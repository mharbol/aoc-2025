use std::{error::Error, fs};

use crate::solution::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12,
};

pub mod argparse;

pub fn get_day(
    day: u32,
) -> Option<(
    Box<fn(&Vec<String>) -> String>,
    Box<fn(&Vec<String>) -> String>,
)> {
    match day {
        1 => Some((Box::new(day01::part1), Box::new(day01::part2))),
        2 => Some((Box::new(day02::part1), Box::new(day02::part2))),
        3 => Some((Box::new(day03::part1), Box::new(day03::part2))),
        4 => Some((Box::new(day04::part1), Box::new(day04::part2))),
        5 => Some((Box::new(day05::part1), Box::new(day05::part2))),
        6 => Some((Box::new(day06::part1), Box::new(day06::part2))),
        7 => Some((Box::new(day07::part1), Box::new(day07::part2))),
        8 => Some((Box::new(day08::part1), Box::new(day08::part2))),
        9 => Some((Box::new(day09::part1), Box::new(day09::part2))),
        10 => Some((Box::new(day10::part1), Box::new(day10::part2))),
        11 => Some((Box::new(day11::part1), Box::new(day11::part2))),
        12 => Some((Box::new(day12::part1), Box::new(day12::part2))),
        _ => None,
    }
}

pub fn get_file_lines(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // this could be done a little better...
    Ok(fs::read_to_string(file_path)?
        .lines()
        .map(String::from)
        .collect())
}
