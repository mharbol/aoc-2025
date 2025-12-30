use std::collections::{HashMap, HashSet};

pub fn part1(lines: &Vec<String>) -> String {
    let mut graph = HashMap::new();
    for &(_, n0, n1) in parse_distances(lines).iter().take(1000) {
        graph.entry(n0).or_insert(HashSet::new()).insert(n1);
        graph.entry(n1).or_insert(HashSet::new()).insert(n0);
    }
    let group_sizes = count_group_sizes(&graph);
    group_sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

// TODO - make this faster
pub fn part2(lines: &Vec<String>) -> String {
    let coords = parse_coordinates(lines);
    let (n0, n1) = build_connected_graph(&coords);
    (coords[n0].1.0 * coords[n1].1.0).to_string()
}

fn build_connected_graph(coords: &Vec<(usize, (u64, u64, u64))>) -> (usize, usize) {
    let mut distances = get_distances(coords);
    distances.sort();
    let mut graph = HashMap::new();
    for (_, n0, n1) in distances {
        graph.entry(n0).or_insert(HashSet::new()).insert(n1);
        graph.entry(n1).or_insert(HashSet::new()).insert(n0);
        let mut visited = HashSet::new();
        visit_network(n0, &graph, &mut visited);
        if visited.len() == coords.len() {
            return (n0, n1);
        }
    }
    (0, 0)
}

fn parse_distances(lines: &Vec<String>) -> Vec<(u64, usize, usize)> {
    let mut ret = get_distances(&parse_coordinates(lines));
    ret.sort();
    ret
}

fn visit_network(
    start: usize,
    graph: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
) {
    if !visited.contains(&start) {
        visited.insert(start);
        if let Some(set) = graph.get(&start) {
            for new_start in set {
                visit_network(*new_start, graph, visited);
            }
        }
    }
}

fn count_group_sizes(graph: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut all_visited = HashSet::new();
    for start in graph.keys() {
        if !all_visited.contains(start) {
            let mut visited: HashSet<usize> = HashSet::new();
            visit_network(*start, graph, &mut visited);
            ret.push(visited.len());
            visited.iter().for_each(|val| {
                all_visited.insert(*val);
            });
        }
    }
    ret.sort();
    ret
}

fn get_distances(coords: &Vec<(usize, (u64, u64, u64))>) -> Vec<(u64, usize, usize)> {
    let mut ret = Vec::new();
    for c0 in 0..coords.len() - 1 {
        for c1 in c0 + 1..coords.len() {
            let (idx0, coord0) = coords[c0];
            let (idx1, coord1) = coords[c1];
            ret.push((squared_dist_3(coord0, coord1), idx0, idx1));
        }
    }
    ret
}

fn squared_dist_3(first: (u64, u64, u64), second: (u64, u64, u64)) -> u64 {
    let dx = first.0.abs_diff(second.0);
    let dy = first.1.abs_diff(second.1);
    let dz = first.2.abs_diff(second.2);
    dx * dx + dy * dy + dz * dz
}

fn parse_coordinates(lines: &Vec<String>) -> Vec<(usize, (u64, u64, u64))> {
    lines
        .iter()
        .map(|line| line.split(","))
        .map(|mut split| {
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .enumerate()
        .collect()
}
