use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

fn find_low_points(values: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let rows = values.len();
    let cols = values[0].len();

    let mut low_points = Vec::new();

    for x in 0..rows {
        for y in 0..cols {
            let n = values[x][y];

            if x > 0 && n >= values[x - 1][y] {
                continue;
            }

            if x < rows - 1 && n >= values[x + 1][y] {
                continue;
            }

            if y > 0 && n >= values[x][y - 1] {
                continue;
            }

            if y < cols - 1 && n >= values[x][y + 1] {
                continue;
            }

            low_points.push((x, y));
        }
    }

    low_points
}

fn get_basin(
    values: &[Vec<u32>],
    x: usize,
    y: usize,
    visited: &mut Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut basin = Vec::new();

    if values[x][y] == 9 {
        return basin;
    }

    visited.push((x, y));
    basin.push((x, y));

    let rows = values.len();
    let cols = values[0].len();

    if x > 0 && !visited.contains(&(x - 1, y)) {
        visited.push((x - 1, y));
        basin.append(&mut get_basin(values, x - 1, y, visited));
    }

    if x < rows - 1 && !visited.contains(&(x + 1, y)) {
        visited.push((x + 1, y));
        basin.append(&mut get_basin(values, x + 1, y, visited));
    }

    if y > 0 && !visited.contains(&(x, y - 1)) {
        visited.push((x, y - 1));
        basin.append(&mut get_basin(values, x, y - 1, visited));
    }

    if y < cols - 1 && !visited.contains(&(x, y + 1)) {
        visited.push((x, y + 1));
        basin.append(&mut get_basin(values, x, y + 1, visited));
    }

    basin
}

fn part_1(values: &[Vec<u32>]) -> u32 {
    let low_points = find_low_points(values);

    low_points
        .iter()
        .map(|&(x, y)| values[x][y] + 1)
        .sum::<u32>()
}

fn part_2(values: &[Vec<u32>]) -> usize {
    let low_points = find_low_points(values);

    let basins = low_points
        .iter()
        .map(|&(x, y)| get_basin(values, x, y, &mut Vec::new()))
        .collect::<Vec<Vec<(usize, usize)>>>();

    let mut sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0..3].iter().product::<usize>()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let values = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let p1 = part_1(&values);
    let p2 = part_2(&values);

    (p1, p2)
}

fn main() {
    let input = get_input("day09.txt");

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

    static TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|v| {
                v.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        assert_eq!(15, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|v| {
                v.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        assert_eq!(1134, part_2(&input));
    }
}
