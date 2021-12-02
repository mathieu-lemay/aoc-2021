use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn parse_commands(input: &[String]) -> Vec<(&str, u32)> {
    input
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(cmd, n)| (cmd, n.parse().unwrap()))
        .collect()
}

fn part1(commands: &[(&str, u32)]) -> u32 {
    let mut pos = 0;
    let mut depth = 0;

    for (cmd, n) in commands {
        match *cmd {
            "forward" => pos += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => panic!("Invalid command: {}", cmd),
        }
    }

    pos * depth
}

fn part2(commands: &[(&str, u32)]) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, n) in commands {
        match *cmd {
            "forward" => {
                pos += n;
                depth += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!("Invalid command: {}", cmd),
        }
    }

    pos * depth
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let commands = parse_commands(input);

    let p1 = part1(&commands);
    let p2 = part2(&commands);
    (p1, p2)
}

fn main() {
    let input = get_input("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_commands, part1, part2};

    #[test]
    fn test_p1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let input = input
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let commands = parse_commands(&input);

        let res = part1(&commands);

        assert_eq!(150, res);
    }

    #[test]
    fn test_p2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let input = input
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let commands = parse_commands(&input);

        let res = part2(&commands);

        assert_eq!(900, res);
    }
}
