use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn find_empty_chunk(chars: &[char]) -> Option<usize> {
    for i in 0..chars.len() - 1 {
        let a = chars[i];
        let b = chars[i + 1];

        match (a, b) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => return Some(i),
            _ => {}
        }
    }

    None
}

fn remove_empty_chunks(value: &str) -> Vec<char> {
    let mut chars = value.chars().collect::<Vec<char>>();

    while let Some(i) = find_empty_chunk(&chars) {
        chars.remove(i);
        chars.remove(i);
    }

    chars
}

fn get_incorrect_char(value: &str) -> Option<char> {
    let chars = remove_empty_chunks(value);

    chars
        .iter()
        .find(|c| matches!(c, ')' | ']' | '}' | '>'))
        .cloned()
}

fn part_1(values: &[String]) -> usize {
    let mut res = 0;

    for val in values {
        res += match get_incorrect_char(val) {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            Some(_) => panic!("Invalid char"),
            None => 0,
        };
    }

    res
}

fn get_missing_chars(value: &str) -> Vec<char> {
    let chars = remove_empty_chunks(value);

    chars
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Invalid char: {}", c),
        })
        .collect::<Vec<char>>()
}

fn part_2(values: &[String]) -> usize {
    let values = values
        .iter()
        .filter(|s| get_incorrect_char(s).is_none())
        .collect::<Vec<&String>>();

    let mut points: Vec<usize> = values
        .iter()
        .map(|v| {
            get_missing_chars(v).iter().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Invalid char: {}", c),
                    }
            })
        })
        .collect();

    points.sort_unstable();

    points[points.len() / 2]
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = part_1(input);
    let p2 = part_2(input);

    (p1, p2)
}

fn main() {
    let input = get_input("day10.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    static TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(26397, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(288957, part_2(&input));
    }
}
