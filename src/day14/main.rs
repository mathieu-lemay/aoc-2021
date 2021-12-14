use std::collections::HashMap;
use std::fmt::Display;
use std::iter;
use std::time::Instant;

use itertools::Itertools;

use aoc_2021::get_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rule<'a> {
    pattern: &'a str,
    character: &'a str,
}

fn parse_input(input: &[String]) -> (HashMap<String, usize>, Vec<Rule>) {
    let pairs = input[0]
        .chars()
        .chain(iter::once(' '))
        .tuple_windows()
        .map(|(a, b)| format!("{}{}", a, b))
        .counts();

    let rules: Vec<Rule> = input[2..]
        .iter()
        .map(|s| {
            let (p, c) = s.split_once(" -> ").unwrap();

            Rule {
                pattern: p,
                character: c,
            }
        })
        .collect();

    (pairs, rules)
}

fn expand(
    pairs: &HashMap<String, usize>,
    rules: &[Rule],
    iterations: usize,
) -> HashMap<String, usize> {
    let mut pairs = pairs.clone();

    for _ in 0..iterations {
        let mut new_pairs = pairs.clone();

        for r in rules {
            if !pairs.contains_key(r.pattern) {
                continue;
            }

            let c = pairs[r.pattern];

            *new_pairs.get_mut(r.pattern).unwrap() -= c;

            let pattern = r.pattern[..1].to_string() + r.character;
            *new_pairs.entry(pattern).or_insert(0) += c;

            let pattern = r.character.to_owned() + &r.pattern[1..];
            *new_pairs.entry(pattern).or_insert(0) += c;
        }

        pairs = new_pairs;
    }

    pairs
}

fn get_diff_of_elements(
    pairs: &HashMap<String, usize>,
    rules: &[Rule],
    iterations: usize,
) -> usize {
    let pairs = expand(pairs, rules, iterations);

    let mut counts = HashMap::new();
    for (pair, count) in pairs {
        *counts.entry(pair.chars().next().unwrap()).or_insert(0) += count;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (pairs, rules) = parse_input(input);

    let p1 = get_diff_of_elements(&pairs, &rules, 10);
    let p2 = get_diff_of_elements(&pairs, &rules, 40);

    (p1, p2)
}

fn main() {
    let input = get_input("day14.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{expand, get_diff_of_elements, parse_input, Rule};
    use std::collections::HashMap;

    static TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (pairs, rules) = parse_input(&input);

        let mut expected_pairs = HashMap::new();

        expected_pairs.insert("NN".to_string(), 1);
        expected_pairs.insert("NC".to_string(), 1);
        expected_pairs.insert("CB".to_string(), 1);
        expected_pairs.insert("B ".to_string(), 1);

        assert_eq!(pairs, expected_pairs);

        let expected_rules = vec![
            Rule {
                pattern: "CH",
                character: "B",
            },
            Rule {
                pattern: "HH",
                character: "N",
            },
            Rule {
                pattern: "CB",
                character: "H",
            },
            Rule {
                pattern: "NH",
                character: "C",
            },
            Rule {
                pattern: "HB",
                character: "C",
            },
            Rule {
                pattern: "HC",
                character: "B",
            },
            Rule {
                pattern: "HN",
                character: "C",
            },
            Rule {
                pattern: "NN",
                character: "C",
            },
            Rule {
                pattern: "BH",
                character: "H",
            },
            Rule {
                pattern: "NC",
                character: "B",
            },
            Rule {
                pattern: "NB",
                character: "B",
            },
            Rule {
                pattern: "BN",
                character: "B",
            },
            Rule {
                pattern: "BB",
                character: "N",
            },
            Rule {
                pattern: "BC",
                character: "B",
            },
            Rule {
                pattern: "CC",
                character: "N",
            },
            Rule {
                pattern: "CN",
                character: "C",
            },
        ];

        assert_eq!(rules, expected_rules);
    }

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (pairs, rules) = parse_input(&input);

        let result = get_diff_of_elements(&pairs, &rules, 10);

        assert_eq!(result, 1588);
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (pairs, rules) = parse_input(&input);

        let result = get_diff_of_elements(&pairs, &rules, 40);

        assert_eq!(result, 2188189693529);
    }

    #[test]
    fn test_expand() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let (pairs, rules) = parse_input(&input);

        let result = expand(&pairs, &rules, 10);

        let mut expected: HashMap<String, usize> = HashMap::new();
        expected.insert("B ".to_string(), 1);
        expected.insert("BB".to_string(), 812);
        expected.insert("BC".to_string(), 120);
        expected.insert("BH".to_string(), 81);
        expected.insert("BN".to_string(), 735);
        expected.insert("CB".to_string(), 115);
        expected.insert("CC".to_string(), 60);
        expected.insert("CH".to_string(), 21);
        expected.insert("CN".to_string(), 102);
        expected.insert("HB".to_string(), 26);
        expected.insert("HC".to_string(), 76);
        expected.insert("HH".to_string(), 32);
        expected.insert("HN".to_string(), 27);
        expected.insert("NB".to_string(), 796);
        expected.insert("NC".to_string(), 42);
        expected.insert("NH".to_string(), 27);
        expected.insert("NN".to_string(), 0);

        assert_eq!(result, expected);
    }
}
