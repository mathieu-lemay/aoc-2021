use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input_as_int;

fn part1(values: &[u32]) -> u32 {
    let mut cur = values[0];
    let mut increases = 0;

    for v in values.iter().skip(1) {
        if *v > cur {
            increases += 1;
        }

        cur = *v;
    }

    increases
}

fn part2(values: &[u32]) -> u32 {
    let mut cur = values[0] + values[1] + values[2];
    let mut increases = 0;

    for i in 1..values.len() - 2 {
        let v = values[i] + values[i + 1] + values[i + 2];
        if v > cur {
            increases += 1;
        }

        cur = v;
    }

    increases
}

fn solve(input: &[u32]) -> (impl Display, impl Display) {
    let p1 = part1(input);
    let p2 = part2(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_int("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_p1() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let input = input
            .split("\n")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u32>>();
        let res = part1(&input);

        assert_eq!(7, res);
    }

    #[test]
    fn test_p2() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let input = input
            .split("\n")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u32>>();

        let res = part2(&input);

        assert_eq!(5, res);
    }
}
