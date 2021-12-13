use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Edge(String, String);

fn parse_input(input: &[String]) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    for l in input {
        let (a, b) = l.split_once('-').unwrap();

        edges.insert(Edge(a.to_owned(), b.to_owned()));
    }

    edges
}

fn get_visitable_nodes_1<'a>(
    node: &str,
    edges: &'a HashSet<Edge>,
    visited: &'a [String],
) -> HashSet<&'a String> {
    let mut visitable = HashSet::new();

    visitable.extend(
        edges
            .iter()
            .filter(|e| e.0 == node && !visited.contains(&e.1))
            .map(|e| &e.1),
    );
    visitable.extend(
        edges
            .iter()
            .filter(|e| e.1 == node && !visited.contains(&e.0))
            .map(|e| &e.0),
    );

    visitable
}

fn get_nb_paths_with_single_visit(
    edges: &HashSet<Edge>,
    current_node: &str,
    visited: &[String],
) -> usize {
    if current_node == "end" {
        return 1;
    }

    let mut visited = visited.to_owned();
    if current_node.to_lowercase() == current_node {
        visited.push(current_node.to_string());
    }

    let mut paths = 0;

    for node in get_visitable_nodes_1(current_node, edges, &visited) {
        paths += get_nb_paths_with_single_visit(edges, node, &visited)
    }

    paths
}

fn is_visitable(node: &str, visited: &HashMap<String, u32>) -> bool {
    if node == "start" {
        return false;
    }

    if node.to_uppercase() == node {
        return true;
    }

    if visited.values().any(|&i| i == 2) {
        return !visited.contains_key(node);
    }

    true
}

fn get_visitable_nodes_2<'a>(
    node: &str,
    edges: &'a HashSet<Edge>,
    visited: &'a HashMap<String, u32>,
) -> HashSet<&'a String> {
    let mut visitable = HashSet::new();

    visitable.extend(
        edges
            .iter()
            .filter(|e| e.0 == node && is_visitable(&e.1, visited))
            .map(|e| &e.1),
    );
    visitable.extend(
        edges
            .iter()
            .filter(|e| e.1 == node && is_visitable(&e.0, visited))
            .map(|e| &e.0),
    );

    visitable
}

fn get_nb_paths_with_double_visit(
    edges: &HashSet<Edge>,
    current_node: &str,
    visited: &HashMap<String, u32>,
) -> usize {
    if current_node == "end" {
        return 1;
    }

    let mut visited = visited.clone();
    if current_node.to_lowercase() == current_node {
        visited.insert(
            current_node.to_string(),
            visited.get(current_node).unwrap_or(&0) + 1,
        );
    }

    let mut paths = 0;

    for node in get_visitable_nodes_2(current_node, edges, &visited) {
        paths += get_nb_paths_with_double_visit(edges, node, &visited)
    }

    paths
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let edges = parse_input(input);

    let p1 = get_nb_paths_with_single_visit(&edges, &"start".to_string(), &Vec::new());
    let p2 = get_nb_paths_with_double_visit(&edges, &"start".to_string(), &HashMap::new());

    assert_eq!(p1, 5457);
    assert_eq!(p2, 128506);

    (p1, p2)
}

fn main() {
    let input = get_input("day12.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{
        get_nb_paths_with_double_visit, get_nb_paths_with_single_visit, parse_input, Edge, HashMap,
        HashSet,
    };

    static TEST_INPUT_A: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static TEST_INPUT_B: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static TEST_INPUT_C: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    fn parsed_input(input: &str) -> HashSet<Edge> {
        let input = input
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        parse_input(&input)
    }

    #[test]
    fn test_parse_input() {
        let edges = parsed_input(TEST_INPUT_A);

        let mut expected_edges = HashSet::new();
        expected_edges.insert(Edge("start".to_string(), "A".to_string()));
        expected_edges.insert(Edge("start".to_string(), "b".to_string()));
        expected_edges.insert(Edge("A".to_string(), "c".to_string()));
        expected_edges.insert(Edge("A".to_string(), "b".to_string()));
        expected_edges.insert(Edge("b".to_string(), "d".to_string()));
        expected_edges.insert(Edge("A".to_string(), "end".to_string()));
        expected_edges.insert(Edge("b".to_string(), "end".to_string()));

        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn test_part_1_a() {
        let edges = parsed_input(TEST_INPUT_A);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &"start".to_string(), &vec![]),
            10
        );
    }
    #[test]
    fn test_part_1_b() {
        let edges = parsed_input(TEST_INPUT_B);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &"start".to_string(), &vec![]),
            19
        );
    }
    #[test]
    fn test_part_1_c() {
        let edges = parsed_input(TEST_INPUT_C);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &"start".to_string(), &vec![]),
            226
        );
    }

    #[test]
    fn test_part_2_a() {
        let edges = parsed_input(TEST_INPUT_A);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &"start".to_string(), &HashMap::new()),
            36
        );
    }
    #[test]
    fn test_part_2_b() {
        let edges = parsed_input(TEST_INPUT_B);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &"start".to_string(), &HashMap::new()),
            103
        );
    }
    #[test]
    fn test_part_2_c() {
        let edges = parsed_input(TEST_INPUT_C);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &"start".to_string(), &HashMap::new()),
            3509
        );
    }
}
