use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> String {
    let (col, _) = lines[0]
        .chars()
        .enumerate()
        .find(|&(_, ch)| 'S' == ch)
        .unwrap();
    let mut cache = HashMap::new();
    trace_down_quantum_tachyon(lines, &mut cache, 0, col);
    cache.len().to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let (col, _) = lines[0]
        .chars()
        .enumerate()
        .find(|&(_, ch)| 'S' == ch)
        .unwrap();
    let mut cache = HashMap::new();
    trace_down_quantum_tachyon(lines, &mut cache, 0, col).to_string()
}

fn trace_down_quantum_tachyon(
    lines: &Vec<String>,
    cache: &mut HashMap<(usize, usize), u64>,
    row: usize,
    col: usize,
) -> u64 {
    if cache.contains_key(&(row, col)) {
        *cache.get(&(row, col)).unwrap()
    } else {
        if row < lines.len() {
            if '^' == lines[row].chars().nth(col).unwrap() {
                let ret = trace_down_quantum_tachyon(lines, cache, row, col - 1)
                    + trace_down_quantum_tachyon(lines, cache, row, col + 1);
                cache.insert((row, col), ret);
                ret
            } else {
                trace_down_quantum_tachyon(lines, cache, row + 1, col)
            }
        } else {
            1
        }
    }
}
