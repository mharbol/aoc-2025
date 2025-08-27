use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> String {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in lines {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut right_count: HashMap<u32, u32> = HashMap::new();
    let mut left_vec: Vec<u32> = Vec::new();
    for line in lines {
        let mut nums = line.split_whitespace();
        left_vec.push(nums.next().unwrap().parse().unwrap());
        *right_count
            .entry(nums.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
    }
    left_vec.iter()
        .filter_map(|left| Some(right_count.get(left)? * left))
        .sum::<u32>()
        .to_string()
}
