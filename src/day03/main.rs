use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn parse_values(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|s| {
            let mut n: u32 = 0;
            for c in s.chars() {
                n <<= 1;
                if c == '1' {
                    n |= 1;
                }
            }

            n
        })
        .collect()
}

fn part1(values: &[u32]) -> u32 {
    let nb_entries = values.len();
    let nb_bits = (*values.iter().max().unwrap() as f32).log2().ceil() as usize;

    let mut gamma = 0;

    let mut i = 1 << (nb_bits - 1);

    while i > 0 {
        let nb_ones = values.iter().filter(|&&n| n & i != 0).count();

        if nb_ones >= nb_entries / 2 {
            gamma |= i
        }

        i >>= 1;
    }

    gamma * (gamma ^ ((1 << nb_bits) - 1))
}

fn part2(values: &[u32]) -> u32 {
    let nb_bits = (*values.iter().max().unwrap() as f32).log2().ceil() as usize;

    let mut o2_candidates = values.to_vec();
    let mut co2_candidates = values.to_vec();

    let mut i = 1 << (nb_bits - 1);
    while o2_candidates.len() > 1 {
        let nb_ones = o2_candidates.iter().filter(|&&n| n & i != 0).count();

        if nb_ones >= (o2_candidates.len() - nb_ones) {
            o2_candidates = o2_candidates
                .iter()
                .filter(|&&n| n & i != 0)
                .copied()
                .collect();
        } else {
            o2_candidates = o2_candidates
                .iter()
                .filter(|&&n| n & i == 0)
                .copied()
                .collect();
        }

        i >>= 1;
    }

    let mut i = 1 << (nb_bits - 1);
    while co2_candidates.len() > 1 {
        let nb_zeroes = co2_candidates.iter().filter(|&&n| n & i == 0).count();

        if nb_zeroes <= (co2_candidates.len() - nb_zeroes) {
            co2_candidates = co2_candidates
                .iter()
                .filter(|&&n| n & i == 0)
                .copied()
                .collect();
        } else {
            co2_candidates = co2_candidates
                .iter()
                .filter(|&&n| n & i != 0)
                .copied()
                .collect();
        }

        i >>= 1;
    }

    o2_candidates.first().unwrap() * co2_candidates.first().unwrap()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let values = parse_values(input);

    let p1 = part1(&values);
    let p2 = part2(&values);
    (p1, p2)
}

fn main() {
    let input = get_input("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_values, part1, part2};

    #[test]
    fn test_p1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let input = input
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let res = part1(&parse_values(&input));

        assert_eq!(198, res);
    }

    #[test]
    fn test_p2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let input = input
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let res = part2(&parse_values(&input));

        assert_eq!(230, res);
    }
}
