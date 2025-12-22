use std::collections::{HashMap, HashSet};

use crate::solution::utils::AocVecStringAccess;

pub fn part1(lines: &Vec<String>) -> String {
    let (row, col, _) = lines.row_col_iter().find(|&(_, _, ch)| 'S' == ch).unwrap();
    let mut cache = HashMap::new();
    trace_down_quantum_tachyon(lines, &mut cache, row as isize, col as isize);
    cache.len().to_string()
}

pub fn part2(lines: &Vec<String>) -> String {
    let (row, col, _) = lines.row_col_iter().find(|&(_, _, ch)| 'S' == ch).unwrap();
    let mut cache = HashMap::new();
    trace_down_quantum_tachyon(lines, &mut cache, row as isize, col as isize).to_string()
}

fn trace_down_quantum_tachyon(
    lines: &Vec<String>,
    cache: &mut HashMap<(isize, isize), u64>,
    row: isize,
    col: isize,
) -> u64 {
    if cache.contains_key(&(row, col)) {
        *cache.get(&(row, col)).unwrap()
    } else {
        if let Some(ch) = lines.get_signed(row, col) {
            if '^' == ch {
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

#[allow(dead_code)]
fn trace_down_tachyon(
    lines: &Vec<String>,
    pair_set: &mut HashSet<(isize, isize)>,
    row: isize,
    col: isize,
) {
    if !pair_set.contains(&(row, col)) {
        if let Some(ch) = lines.get_signed(row, col) {
            if '^' == ch {
                pair_set.insert((row, col));
                trace_down_tachyon(lines, pair_set, row, col - 1);
                trace_down_tachyon(lines, pair_set, row, col + 1);
            } else {
                trace_down_tachyon(lines, pair_set, row + 1, col);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_old() {
        let lines = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        assert_eq!(part1(&lines), "21");

        let (row, col, _) = lines.row_col_iter().find(|&(_, _, ch)| 'S' == ch).unwrap();
        let mut pair_set = HashSet::new();
        trace_down_tachyon(&lines, &mut pair_set, row as isize, col as isize);
        assert_eq!(pair_set.len(), 21);
    }
}
