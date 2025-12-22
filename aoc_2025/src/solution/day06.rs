use crate::solution::utils::AocVecStringAccess;

pub fn part1(lines: &Vec<String>) -> String {
    let (nums, ops) = parse_p1(&lines);
    let nums_len = nums.len();
    (0..nums[0].len())
        .map(|col| {
            if ops[col] {
                (0..nums_len).map(|idx| nums[idx][col]).sum::<u64>()
            } else {
                (0..nums_len).map(|idx| nums[idx][col]).product::<u64>()
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    parse_ops_p2(lines)
        .map(|(idx, op)| parse_calc_col(lines, op, idx))
        .sum::<u64>()
        .to_string()
}

fn parse_p1(lines: &Vec<String>) -> (Vec<Vec<u64>>, Vec<bool>) {
    (
        lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect(),
        lines
            .last()
            .unwrap()
            .split(" ")
            .filter(|o| !o.is_empty())
            .map(|operation| if "+" == operation { true } else { false })
            .collect(),
    )
}

fn parse_ops_p2(lines: &Vec<String>) -> impl Iterator<Item = (usize, char)> {
    lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|&(_, ch)| ' ' != ch)
}

fn parse_col(lines: &Vec<String>, col: isize) -> Option<u64> {
    let ret = (0..lines.len() - 1)
        .map(|row| lines.get_signed(row as isize, col))
        .filter_map(|ch| ch)
        .filter(|&ch| ' ' != ch)
        .fold(0, |acc, e| 10 * acc + e.to_digit(10).unwrap() as u64);
    if 0 == ret { None } else { Some(ret) }
}

fn parse_calc_col(lines: &Vec<String>, op: char, col: usize) -> u64 {
    if '+' == op {
        (col..).map_while(|c| parse_col(lines, c as isize)).sum()
    } else {
        (col..)
            .map_while(|c| parse_col(lines, c as isize))
            .product()
    }
}
