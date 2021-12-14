use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Node {
    Start,
    Small(String),
    Big(String),
    End,
}

impl From<&str> for Node {
    fn from(n: &str) -> Self {
        match n {
            "start" => Node::Start,
            "end" => Node::End,
            n => {
                if n.to_uppercase() == n {
                    Node::Big(n.to_owned())
                } else {
                    Node::Small(n.to_owned())
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Edge(Node, Node);

fn parse_input(input: &[String]) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    for l in input {
        let (a, b) = l.split_once('-').unwrap();

        edges.insert(Edge(Node::from(a), Node::from(b)));
    }

    edges
}

fn get_visitable_nodes<'a, F>(
    node: &Node,
    edges: &'a HashSet<Edge>,
    predicate: F,
) -> HashSet<&'a Node>
where
    F: Fn(&'a Node) -> bool,
{
    let mut visitable = HashSet::new();

    visitable.extend(
        edges
            .iter()
            .filter(|e| &e.0 == node && predicate(&e.1))
            .map(|e| &e.1),
    );
    visitable.extend(
        edges
            .iter()
            .filter(|e| &e.1 == node && predicate(&e.0))
            .map(|e| &e.0),
    );

    visitable
}

fn get_nb_paths_with_single_visit(
    edges: &HashSet<Edge>,
    current_node: &Node,
    visited: &[&Node],
) -> usize {
    if current_node == &Node::End {
        return 1;
    }

    let mut visited = visited.to_owned();
    if let Node::Small(_) = current_node {
        visited.push(current_node);
    }

    let mut paths = 0;

    let visitable = |node: &Node| node != &Node::Start && !visited.contains(&node);

    for node in get_visitable_nodes(current_node, edges, visitable) {
        paths += get_nb_paths_with_single_visit(edges, node, &visited)
    }

    paths
}

fn get_nb_paths_with_double_visit(
    edges: &HashSet<Edge>,
    current_node: &Node,
    visited: &HashMap<&Node, u32>,
) -> usize {
    if current_node == &Node::End {
        return 1;
    }

    let mut visited = visited.clone();
    if let Node::Small(_) = current_node {
        visited.insert(current_node, visited.get(current_node).unwrap_or(&0) + 1);
    }

    let mut paths = 0;

    let visitable = |node: &Node| {
        if node == &Node::Start {
            return false;
        }

        if let Node::Big(_) = node {
            return true;
        }

        if visited.values().any(|&i| i == 2) {
            return !visited.contains_key(node);
        }

        true
    };

    for node in get_visitable_nodes(current_node, edges, visitable) {
        paths += get_nb_paths_with_double_visit(edges, node, &visited)
    }

    paths
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let edges = parse_input(input);

    let p1 = get_nb_paths_with_single_visit(&edges, &Node::Start, &Vec::new());
    let p2 = get_nb_paths_with_double_visit(&edges, &Node::Start, &HashMap::new());

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
        HashSet, Node,
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
        expected_edges.insert(Edge(Node::Start, Node::Big("A".to_string())));
        expected_edges.insert(Edge(Node::Start, Node::Small("b".to_string())));
        expected_edges.insert(Edge(
            Node::Big("A".to_string()),
            Node::Small("c".to_string()),
        ));
        expected_edges.insert(Edge(
            Node::Big("A".to_string()),
            Node::Small("b".to_string()),
        ));
        expected_edges.insert(Edge(
            Node::Small("b".to_string()),
            Node::Small("d".to_string()),
        ));
        expected_edges.insert(Edge(Node::Big("A".to_string()), Node::End));
        expected_edges.insert(Edge(Node::Small("b".to_string()), Node::End));

        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn test_part_1_a() {
        let edges = parsed_input(TEST_INPUT_A);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &Node::Start, &vec![]),
            10
        );
    }
    #[test]
    fn test_part_1_b() {
        let edges = parsed_input(TEST_INPUT_B);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &Node::Start, &vec![]),
            19
        );
    }
    #[test]
    fn test_part_1_c() {
        let edges = parsed_input(TEST_INPUT_C);

        assert_eq!(
            get_nb_paths_with_single_visit(&edges, &Node::Start, &vec![]),
            226
        );
    }

    #[test]
    fn test_part_2_a() {
        let edges = parsed_input(TEST_INPUT_A);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &Node::Start, &HashMap::new()),
            36
        );
    }
    #[test]
    fn test_part_2_b() {
        let edges = parsed_input(TEST_INPUT_B);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &Node::Start, &HashMap::new()),
            103
        );
    }
    #[test]
    fn test_part_2_c() {
        let edges = parsed_input(TEST_INPUT_C);

        assert_eq!(
            get_nb_paths_with_double_visit(&edges, &Node::Start, &HashMap::new()),
            3509
        );
    }
}
