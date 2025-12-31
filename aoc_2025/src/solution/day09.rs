pub fn part1(lines: &Vec<String>) -> String {
    let tiles = parse_tiles(lines);
    (0..tiles.len() - 1)
        .flat_map(|first| (first + 1..tiles.len()).map(move |second| (first, second)))
        .map(|(first, second)| (tiles[first], tiles[second]))
        .map(|((r0, c0), (r1, c1))| (r0.abs_diff(r1) + 1) * (c0.abs_diff(c1) + 1))
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(_lines: &Vec<String>) -> String {
    "".to_string()
}

fn parse_tiles(lines: &Vec<String>) -> Vec<(u64, u64)> {
    lines
        .iter()
        .map(|line| line.split_once(",").unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect()
}
