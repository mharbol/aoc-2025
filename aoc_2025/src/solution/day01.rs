pub fn part1(lines: &Vec<String>) -> String {
    let mut state = 50;
    let mut count = 0;

    for add in lines.iter().map(|line| {
        if 'L' == line.chars().nth(0).unwrap() {
            -line[1..].parse::<i32>().unwrap()
        } else {
            line[1..].parse::<i32>().unwrap()
        }
    }) {
        state = (state + add).rem_euclid(100);
        if 0 == state {
            count += 1;
        }
    }
    count.to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let mut state = 50;
    let mut count = 0;

    for add in lines.iter().map(|line| {
        if 'L' == line.chars().nth(0).unwrap() {
            -line[1..].parse::<i32>().unwrap()
        } else {
            line[1..].parse::<i32>().unwrap()
        }
    }) {
        let old_state = state;
        count += add.abs() / 100;
        state += add % 100;
        if 0 == state || 0 != old_state && (0 > state || 100 <= state) {
            count += 1;
        }
        state = state.rem_euclid(100);
    }
    count.to_string()
}
