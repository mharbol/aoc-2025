pub fn part1(lines: &Vec<String>) -> String {
    let tiles = parse_coordinates(lines);
    (0..tiles.len() - 1)
        .flat_map(|first| (first + 1..tiles.len()).map(move |second| (first, second)))
        .map(|(first, second)| (&tiles[first], &tiles[second]))
        .map(|(c1, c2)| c1.area_with(c2))
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(_lines: &Vec<String>) -> String {
    "".to_string()
}

struct Coordinate {
    row: u64,
    col: u64,
}

impl Coordinate {
    fn new(row: u64, col: u64) -> Coordinate {
        Coordinate { row: row, col: col }
    }

    fn from_string(string: &String) -> Coordinate {
        let (row, col) = string.split_once(",").unwrap();
        Coordinate::new(row.parse().unwrap(), col.parse().unwrap())
    }

    fn area_with(&self, other: &Coordinate) -> u64 {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }
}

fn parse_coordinates(lines: &Vec<String>) -> Vec<Coordinate> {
    lines.iter().map(Coordinate::from_string).collect()
}
