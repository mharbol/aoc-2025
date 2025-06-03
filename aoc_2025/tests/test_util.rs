use aoc_2025::cmd::{get_day, get_file_lines};

pub fn test_day_part(day: u32, part: u32, expected: &str) {
    let lines =
        get_file_lines(format!("../public/resources/{}", format!("day{:0>2}.txt", day)).as_str())
            .unwrap();
    assert_eq!(
        String::from(expected),
        match part {
            1 => get_day(day).unwrap().0(&lines),
            2 => get_day(day).unwrap().1(&lines),
            _ => panic!(),
        }
    );
}
