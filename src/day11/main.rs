use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;
use itertools::Itertools;

fn tick(values: &mut Vec<u32>) -> usize {
    let mut flashes = Vec::new();
    let nb_values = values.len();
    let row_size = (nb_values as f32).sqrt() as usize;

    for i in 0..values.len() {
        values[i] += 1;
    }

    loop {
        let mut flashing = values
            .iter()
            .enumerate()
            .filter(|(idx, &val)| val > 9 && !flashes.contains(idx))
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();

        if flashing.is_empty() {
            break;
        }

        for idx in &flashing {
            let x = idx / row_size;
            let xs = if x == 0 {
                vec![0, 1]
            } else if x == row_size - 1 {
                vec![row_size - 1, row_size - 2]
            } else {
                vec![x - 1, x, x + 1]
            };

            let y = idx % row_size;
            let ys = if y == 0 {
                vec![0, 1]
            } else if y == row_size - 1 {
                vec![row_size - 1, row_size - 2]
            } else {
                vec![y - 1, y, y + 1]
            };

            for (x, y) in xs.iter().cartesian_product(&ys) {
                let n = x * row_size + y;
                if &n != idx {
                    values[n] += 1;
                }
            }
        }

        flashes.append(&mut flashing);
    }

    for i in 0..values.len() {
        if values[i] > 9 {
            values[i] = 0;
        }
    }

    flashes.len()
}

fn part_1(values: &[u32], ticks: usize) -> usize {
    let mut values = values.to_owned();
    let mut nb_flashes = 0;

    for _ in 0..ticks {
        nb_flashes += tick(&mut values);
    }

    nb_flashes
}

fn part_2(values: &[u32]) -> usize {
    let mut values = values.to_owned();

    for i in 1.. {
        if tick(&mut values) == values.len() {
            return i;
        }
    }

    panic!();
}

fn parse_input(input: &[String]) -> Vec<u32> {
    let mut values = Vec::with_capacity(100);

    for s in input {
        values.append(
            &mut s
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        )
    }

    values
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let values = parse_input(input);

    let p1 = part_1(&values, 100);
    let p2 = part_2(&values);

    (p1, p2)
}

fn main() {
    let input = get_input("day11.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1, part_2};

    static TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let values = parse_input(&input);

        assert_eq!(204, part_1(&values, 10));
        assert_eq!(1656, part_1(&values, 100));
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let values = parse_input(&input);

        assert_eq!(195, part_2(&values));
    }
}
