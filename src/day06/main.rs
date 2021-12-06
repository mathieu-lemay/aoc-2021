use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn compute_population(state: &mut VecDeque<u64>, days: usize) -> u64 {
    for _ in 0..days {
        let nb_new = state.pop_front().unwrap();
        state.push_back(nb_new);
        state[6] += nb_new;
    }

    state.iter().sum::<u64>()
}

fn solve(input: &[u8]) -> (impl Display, impl Display) {
    let mut state = VecDeque::from(vec![0u64; 9]);

    for i in input {
        state[*i as usize] += 1;
    }

    let p1 = compute_population(&mut state, 80);
    let p2 = compute_population(&mut state, 256 - 80);

    (p1, p2)
}

fn main() {
    let input = get_input("day06.txt")[0]
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<u8>>();

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::compute_population;
    use std::collections::VecDeque;

    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_compute_population() {
        let input = TEST_INPUT
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u8>>();

        let mut state = VecDeque::from(vec![0u64; 9]);

        for i in input {
            state[i as usize] += 1;
        }

        let res = compute_population(&mut state, 80);

        assert_eq!(res, 5934);

        let res = compute_population(&mut state, 256 - 80);
        assert_eq!(res, 26984457539);
    }
}
