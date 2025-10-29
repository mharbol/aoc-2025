pub fn part1(lines: &Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| {
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
        })
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
        .filter(|nums| {
            (0..nums.len()).any(|drop_idx| {
                nums.iter()
                    .enumerate()
                    .filter_map(|(idx, num)| if idx == drop_idx { None } else { Some(*num) })
                    .collect::<Vec<i32>>()
                    .as_slice()
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
            })
        })
        .count()
        .to_string()
}
