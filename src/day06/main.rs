use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn compute_population(input: &[u8], days: usize) -> u64 {
    let mut state = vec![0u64; 9];

    for i in input {
        state[*i as usize] += 1;
    }

    for _ in 0..days {
        let nb_new = state.remove(0);
        state.push(nb_new);
        state[6] += nb_new;
    }

    state.iter().sum::<u64>()
}

fn solve(input: &[u8]) -> (impl Display, impl Display) {
    let p1 = compute_population(input, 80);
    let p2 = compute_population(input, 256);
    (p1, p2)
}

fn main() {
    let input = get_input("day06.txt")[0]
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u8>>();

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::compute_population;

    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_compute_population() {
        let input = TEST_INPUT
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u8>>();

        let res = compute_population(&input, 80);

        assert_eq!(res, 5934);

        let res = compute_population(&input, 256);
        assert_eq!(res, 26984457539);
    }
}
