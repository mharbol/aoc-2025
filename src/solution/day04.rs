use crate::solution::utils::AocVecStringAccess;

pub fn part1(lines: &Vec<String>) -> String {
    lines
        .row_col_iter()
        .filter(|&(row, col, ch)| '@' == ch && can_lift(lines, row as isize, col as isize))
        .count()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut acc = 0;
    let mut remove = get_all_can_remove(lines);
    let mut lines_copy = lines.clone();
    while 0 < remove.len() {
        acc += remove.len();
        remove
            .iter()
            .for_each(|&(row, col)| lines_copy[row].replace_range(col..=col, "."));
        remove = get_all_can_remove(&lines_copy);
    }
    acc.to_string()
}

fn can_lift(lines: &Vec<String>, row: isize, col: isize) -> bool {
    (row - 1..=row + 1)
        .flat_map(|r| (col - 1..=col + 1).map(move |c| (r, c)))
        .filter_map(|(row, col)| lines.get_signed(row, col))
        .filter(|&ch| '@' == ch)
        .count()
        < 4 + 1 // 4 + 1 because we know (row, col) is '@'
}

fn get_all_can_remove(lines: &Vec<String>) -> Vec<(usize, usize)> {
    lines
        .row_col_iter()
        .filter(|&(row, col, ch)| '@' == ch && can_lift(lines, row as isize, col as isize))
        .map(|(row, col, _)| (row, col))
        .collect()
}
