use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn part_1(positions: &[i32]) -> i32 {
    let a = *positions.iter().min().unwrap();
    let b = *positions.iter().max().unwrap();

    (a..=b)
        .map(|t| get_fuel_cost_linear(&positions, t))
        .min()
        .unwrap()
}

fn part_2(positions: &[i32]) -> i32 {
    let a = *positions.iter().min().unwrap();
    let b = *positions.iter().max().unwrap();

    (a..=b)
        .map(|t| get_fuel_cost_increasing(&positions, t))
        .min()
        .unwrap()
}

fn get_fuel_cost_linear(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|&p| (p - target).abs()).sum()
}

fn get_fuel_cost_increasing(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|&p| {
            let x = (p - target).abs();
            x * (x + 1) / 2
        })
        .sum()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let positions = input[0]
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();

    let p1 = part_1(&positions);
    let p2 = part_2(&positions);

    (p1, p2)
}

fn main() {
    let input = get_input("day07.txt");

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

    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();

        let res = part_1(&input);

        assert_eq!(res, 37);
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();

        let res = part_2(&input);

        assert_eq!(res, 168);
    }
}
