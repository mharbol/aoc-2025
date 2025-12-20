// TODO remove this dependency
use range_set_blaze::RangeSetBlaze;

pub fn part1(lines: &Vec<String>) -> String {
    let (ranges, nums) = parse(lines);
    nums.iter()
        .filter(|&&num| {
            ranges
                .iter()
                .any(|&(left, right)| left <= num && num <= right)
        })
        .count()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("-").unwrap())
        .map(|(left, right)| (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
        .fold(RangeSetBlaze::new(), |acc, (left, right)| {
            acc | RangeSetBlaze::from(left..=right)
        })
        .len()
        .to_string()
}

fn parse(lines: &Vec<String>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut iter = lines.into_iter();
    let ranges = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("-").unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect();
    let ids = iter.map(|line| line.parse().unwrap()).collect();
    (ranges, ids)
}
