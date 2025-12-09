use crate::solution::utils::AocVecStringAccess;

pub fn part1(lines: &Vec<String>) -> String {
    let mut count = 0;
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if can_lift(lines, row as isize, col as isize) {
                count += 1;
            }
        }
    }
    count.to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut acc = 0;
    let mut remove = get_all_can_remove(lines);
    let mut lines_copy = lines.clone();
    while 0 < remove.len() {
        acc += remove.len();
        for (row, col) in &remove {
            lines_copy[*row].replace_range(*col..*col + 1, ".");
        }
        remove = get_all_can_remove(&lines_copy);
    }
    acc.to_string()
}

fn can_lift(lines: &Vec<String>, row: isize, col: isize) -> bool {
    lines.get_signed(row, col).unwrap() == '@' && {
        let mut count_adj = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if let Some(ch) = lines.get_signed(r, c) {
                    if '@' == ch {
                        count_adj += 1;
                    }
                }
            }
        }
        5 > count_adj // 4 + 1 because we know (row, col) is '@'
    }
}

fn get_all_can_remove(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if can_lift(lines, row as isize, col as isize) {
                ret.push((row, col));
            }
        }
    }
    ret
}
