use itertools::Itertools;

pub fn part1(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(is_safe)
        .count()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(is_drop_safe)
        .count()
        .to_string()
}

fn is_safe(nums: &Vec<i32>) -> bool {
    nums.iter()
        .tuple_windows()
        .map(|(left, right)| left - right)
        .scan(None, |state: &mut Option<bool>, diff| {
            Some(if *state.get_or_insert(diff > 0) {
                diff > 0 && diff <= 3
            } else {
                diff < 0 && diff >= -3
            })
        })
        .all(|c| c)
}

fn is_drop_safe(nums: &Vec<i32>) -> bool {
    for drop in 0..nums.len() {
        if is_drop_safe_idx(nums, drop) {
            return true;
        }
    }
    false
}

fn is_drop_safe_idx(nums: &Vec<i32>, drop_idx: usize) -> bool {
    is_safe(
        &nums
            .iter()
            .enumerate()
            .filter_map(|(idx, num)| if idx == drop_idx { None } else { Some(*num) })
            .collect(),
    )
}
