use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input_as_int;

fn count_increases(values: &[u32], step: usize) -> u32 {
    let mut increases = 0;

    for i in 0..values.len() - step {
        if values[i + step] > values[i] {
            increases += 1;
        }
    }

    increases
}

fn solve(input: &[u32]) -> (impl Display, impl Display) {
    let p1 = count_increases(input, 1);
    let p2 = count_increases(input, 3);

    (p1, p2)
}

fn main() {
    let input = get_input_as_int("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::count_increases;

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
        let res = count_increases(&input, 1);

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

        let res = count_increases(&input, 3);

        assert_eq!(5, res);
    }
}
