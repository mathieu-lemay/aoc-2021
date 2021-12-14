use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

type Segments = Vec<char>;

fn parse_values(input: &[String]) -> Vec<(Vec<Segments>, Vec<Segments>)> {
    let mut values = Vec::with_capacity(input.len());

    for i in input {
        let (patterns, digits) = i.split_once(" | ").unwrap();

        let patterns = patterns
            .split_whitespace()
            .map(|p| p.chars().collect())
            .collect();
        let digits = digits
            .split_whitespace()
            .map(|p| p.chars().collect())
            .collect();

        values.push((patterns, digits));
    }

    values
}

fn get_pattern_diff(p1: &[char], p2: &[char]) -> Vec<char> {
    let h1: HashSet<char> = HashSet::from_iter(p1.iter().cloned());
    let h2: HashSet<char> = HashSet::from_iter(p2.iter().cloned());

    let diff = h1.difference(&h2);

    Vec::from_iter(diff.cloned())
}

fn is_pattern_same(p1: &[char], p2: &[char]) -> bool {
    let h1: HashSet<char> = HashSet::from_iter(p1.iter().cloned());
    let h2: HashSet<char> = HashSet::from_iter(p2.iter().cloned());

    h1 == h2
}

fn get_value(patterns: &[Segments], digits: &[Segments]) -> usize {
    let pattern_1 = patterns.iter().find(|p| p.len() == 2).unwrap().clone();
    let pattern_4 = patterns.iter().find(|p| p.len() == 4).unwrap().clone();
    let pattern_7 = patterns.iter().find(|p| p.len() == 3).unwrap().clone();
    let pattern_8 = patterns.iter().find(|p| p.len() == 7).unwrap().clone();

    let mut patterns: HashSet<Vec<char>> = HashSet::from_iter(patterns.iter().cloned());
    patterns.remove(&pattern_1);
    patterns.remove(&pattern_4);
    patterns.remove(&pattern_7);
    patterns.remove(&pattern_8);

    let pattern_6 = patterns
        .iter()
        .find(|p| p.len() == 6 && get_pattern_diff(&pattern_1, p).len() == 1)
        .unwrap()
        .clone();
    patterns.remove(&pattern_6);

    let segment_c = *get_pattern_diff(&pattern_1, &pattern_6).first().unwrap();

    let pattern_5 = patterns
        .iter()
        .find(|p| p.len() == 5 && !p.contains(&segment_c))
        .unwrap()
        .clone();
    patterns.remove(&pattern_5);

    let segment_f = *pattern_1.iter().find(|&&c| c != segment_c).unwrap();

    let pattern_2 = patterns
        .iter()
        .find(|p| p.len() == 5 && !p.contains(&segment_f))
        .unwrap()
        .clone();
    patterns.remove(&pattern_2);

    let segment_e = *get_pattern_diff(&pattern_2, &pattern_5)
        .iter()
        .find(|&&c| c != segment_c)
        .unwrap();

    let pattern_9 = patterns
        .iter()
        .find(|p| p.len() == 6 && !p.contains(&segment_e))
        .unwrap()
        .clone();
    patterns.remove(&pattern_9);

    let pattern_3 = patterns.iter().find(|p| p.len() == 5).unwrap().clone();
    patterns.remove(&pattern_3);

    assert_eq!(1, patterns.len());
    let pattern_0 = patterns.iter().next().unwrap().clone();

    let patterns = vec![
        pattern_0, pattern_1, pattern_2, pattern_3, pattern_4, pattern_5, pattern_6, pattern_7,
        pattern_8, pattern_9,
    ];

    digits
        .iter()
        .map(|d| {
            patterns
                .iter()
                .enumerate()
                .find(|(_, p)| is_pattern_same(d, p))
                .map(|(i, _)| i)
                .unwrap()
        })
        .fold(0, |acc, d| acc * 10 + d)
}

fn part_1(values: &[(Vec<Segments>, Vec<Segments>)]) -> usize {
    values
        .iter()
        .map(|v| {
            v.1.iter()
                .filter(|&s| matches!(s.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn part_2(values: &[(Vec<Segments>, Vec<Segments>)]) -> usize {
    values.iter().map(|(p, d)| get_value(p, d)).sum()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let segments = parse_values(input);

    let p1 = part_1(&segments);
    let p2 = part_2(&segments);

    (p1, p2)
}

fn main() {
    let input = get_input("day08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_values, part_1, part_2};

    static TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let values = parse_values(&input);

        assert_eq!(26, part_1(&values));
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let values = parse_values(&input);

        assert_eq!(61229, part_2(&values));
    }
}
