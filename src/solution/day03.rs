pub fn part1(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| get_max_joltage(line, 2))
        .sum::<u64>()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| get_max_joltage(line, 12))
        .sum::<u64>()
        .to_string()
}

fn get_max_joltage(line: &String, mut digits: usize) -> u64 {
    let mut ret = 0;
    let mut idx = 0;
    let mut digit;
    while 0 < digits {
        (idx, digit) = get_best_idx_digit(line, idx, digits);
        ret = ret * 10 + digit;
        digits -= 1;
    }
    ret
}

fn get_best_idx_digit(line: &String, curr_idx: usize, digits_remaining: usize) -> (usize, u64) {
    let mut best_idx = curr_idx;
    let mut best_digit = 0;
    for idx in curr_idx..=line.len() - digits_remaining {
        let curr_digit = line.chars().nth(idx).unwrap().to_digit(10).unwrap() as u64;
        if curr_digit > best_digit {
            best_digit = curr_digit;
            best_idx = idx;
        }
    }
    (best_idx + 1, best_digit)
}
