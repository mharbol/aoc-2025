use regex::Regex;

pub fn part1(lines: &Vec<String>) -> String {
    Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")
        .unwrap()
        .find_iter(&lines.join(""))
        .map(|s| s.as_str())
        .map(|mul| mul[4..mul.len() - 1].split_once(",").unwrap())
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .map(|(left, right)| left * right)
        .sum::<u32>()
        .to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))")
        .unwrap()
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
                    let (left, right) = mul_cmd[4..mul_cmd.len() - 1].split_once(",").unwrap();
                    Some(left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap())
                } else {
                    Some(0)
                }
            }
        })
        .sum::<u32>()
        .to_string()
}
