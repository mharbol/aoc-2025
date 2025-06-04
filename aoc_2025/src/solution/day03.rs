use regex::Regex;

pub fn part1(lines: &Vec<String>) -> String {
    let cmd_pattern = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    cmd_pattern
        .find_iter(&lines.join(""))
        .map(|s| s.as_str())
        .map(|mul| mul[4..mul.len() - 1].split_once(",").unwrap())
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .map(|(left, right)| left * right)
        .sum::<u32>()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let cmd_pattern = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    cmd_pattern
        .find_iter(&lines.join(""))
        .map(|s| s.as_str())
        .scan(true, |is_do, command| match command {
            "do()" => {
                *is_do = true;
                Some(0)
            }
            "don't()" => {
                *is_do = false;
                Some(0)
            }
            mul_cmd => {
                if *is_do {
                    let matches = mul_cmd[4..mul_cmd.len() - 1].split_once(",").unwrap();
                    Some(matches.0.parse::<u32>().unwrap() * matches.1.parse::<u32>().unwrap())
                } else {
                    Some(0)
                }
            }
        })
        .sum::<u32>()
        .to_string()
}
