pub fn part1(lines: &Vec<String>) -> String {
    solve(lines, is_safe)
}

pub fn part2(lines: &Vec<String>) -> String {
    solve(lines, is_drop_safe)
}

fn solve<T>(lines: &Vec<String>, func: T) -> String
where
    T: Fn(&Vec<i32>) -> bool,
{
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(func)
        .count()
        .to_string()
}

fn is_safe(nums: &Vec<i32>) -> bool {
    nums.as_slice()
        .windows(2)
        .map(|arr| arr[0] - arr[1])
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
