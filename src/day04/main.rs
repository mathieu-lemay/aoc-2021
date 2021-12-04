use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn parse_input(input: &[String]) -> (Vec<u8>, Vec<Vec<u8>>) {
    let draw_numbers = input[0]
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let mut boards = Vec::new();
    let mut current_board = Vec::new();

    for l in input.iter().skip(2) {
        if l == "" {
            boards.push(current_board);
            current_board = Vec::new();
            continue;
        }

        let row: Vec<u8> = l
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        current_board.extend(row);
    }

    boards.push(current_board);

    (draw_numbers, boards)
}

fn get_winning_board(boards: &[Vec<u8>], numbers: &[&u8]) -> Option<(usize, Vec<u8>)> {
    for (idx, b) in boards.iter().enumerate() {
        for i in 0..5 {
            // check rows
            if b.iter().skip(i * 5).take(5).all(|n| numbers.contains(&n)) {
                return Some((idx, b.clone()));
            }

            // check columns
            if b.iter().skip(i).step_by(5).all(|n| numbers.contains(&n)) {
                return Some((idx, b.clone()));
            }
        }
    }

    None
}

fn part1(draw_numbers: &[u8], boards: &[Vec<u8>]) -> u32 {
    let mut numbers = Vec::new();

    for n in draw_numbers {
        numbers.push(n);

        if let Some((_, winner)) = get_winning_board(boards, &numbers) {
            let res = winner
                .iter()
                .filter(|n| !numbers.contains(n))
                .map(|&n| n as u32)
                .sum::<u32>();
            return res * *n as u32;
        }
    }

    0
}

fn part2(draw_numbers: &[u8], boards: &mut Vec<Vec<u8>>) -> u32 {
    let mut numbers = Vec::new();

    for n in draw_numbers {
        numbers.push(n);

        while let Some((idx, _)) = get_winning_board(boards, &numbers) {
            if boards.len() == 1 {
                let res = boards[0]
                    .iter()
                    .filter(|n| !numbers.contains(n))
                    .map(|&n| n as u32)
                    .sum::<u32>();
                return res * *n as u32;
            }

            boards.remove(idx);
        }
    }

    panic!("Boards remaining: {}", boards.len());
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (draw_numbers, mut boards) = parse_input(&input);

    let p1 = part1(&draw_numbers, &boards);
    let p2 = part2(&draw_numbers, &mut boards);

    (p1, p2)
}

fn main() {
    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_input() {
        let input = INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let res = parse_input(&input);

        assert_eq!(
            res.0,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            res.1,
            vec![
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19
                ],
                vec![
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6
                ],
                vec![
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7
                ]
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let (draw_numbers, boards) = parse_input(&input);
        let res = part1(&draw_numbers, &boards);

        assert_eq!(res, 4512);
    }

    #[test]
    fn test_part_2() {
        let input = INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let (draw_numbers, mut boards) = parse_input(&input);
        let res = part2(&draw_numbers, &mut boards);

        assert_eq!(res, 1924);
    }
}
