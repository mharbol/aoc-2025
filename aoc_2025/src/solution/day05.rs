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

pub fn part2(_lines: &Vec<String>) -> String {
    "".to_string()
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

struct RangeSet {
    ranges: Vec<(u64, u64)>,
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: Vec::new() }
    }

    fn size(self: &RangeSet) -> usize {
        self.ranges
            .iter()
            .map(|&(left, right)| (right - left + 1) as usize)
            .sum()
    }

    fn push(self: &mut RangeSet, range: (u64, u64)) {
        debug_assert!(range.0 < range.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        assert_eq!(
            parse(&lines),
            (
                vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
        assert_eq!(part1(&lines), "3");
    }
}
