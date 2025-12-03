pub fn part1(lines: &Vec<String>) -> String {
    solve(lines, is_invalid_p1)
}

pub fn part2(lines: &Vec<String>) -> String {
    solve(lines, is_invalid_p2)
}

fn solve(lines: &Vec<String>, is_invalid: fn(usize) -> bool) -> String {
    lines[0]
        .split(",")
        .map(|range| range.split_once('-').unwrap())
        .map(|(lhs, rhs)| (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap()))
        .map(|(lhs, rhs)| lhs..(rhs + 1))
        .flatten()
        .filter(|&n| is_invalid(n))
        .sum::<usize>()
        .to_string()
}

fn is_invalid_p1(n: usize) -> bool {
    let mut mag = 1;
    let mut count = 0;
    while mag < n {
        mag *= 10;
        count += 1;
    }
    if 1 == count % 2 {
        false
    } else {
        let mag = 10_usize.pow(count / 2);
        n / mag == n % mag
    }
}

fn is_invalid_p2(n: usize) -> bool {
    let mut mag = 1;
    let mut count = 0;
    while mag < n {
        mag *= 10;
        count += 1;
    }
    let mut mag = 10_usize.pow(count / 2);
    while mag > 1 {
        if is_invalid_p2_for_mag(n, mag) {
            return true;
        }
        mag /= 10;
    }
    false
}

fn is_invalid_p2_for_mag(mut n: usize, mag: usize) -> bool {
    let pattern = n % mag;
    if pattern * 10 < mag {
        return false;
    }
    while n > pattern && pattern == n % mag {
        n /= mag;
    }
    n == pattern
}
