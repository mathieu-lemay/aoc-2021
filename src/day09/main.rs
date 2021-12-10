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

    println!("{:?}", sizes.iter().take(3).collect::<Vec<&usize>>());
    sizes[0..3].iter().product::<usize>()
}

#[derive(Debug)]
struct Chunk {
    row: usize,
    start: usize,
    end: usize,
}

impl Chunk {
    fn len(&self) -> usize {
        self.end + 1 - self.start
    }

    fn overlaps(&self, other: &Chunk) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn get_chunks(row: &[u32], row_idx: usize) -> Vec<Chunk> {
    let mut idx = 0;
    let max = row.len();

    let mut chunks = Vec::new();

    loop {
        while idx < max && row[idx] == 9 {
            idx += 1;
            continue;
        }

        if idx == max {
            break;
        }

        let s = idx;
        while idx < max && row[idx] != 9 {
            idx += 1
        }

        let e = idx;

        chunks.push(Chunk {
            start: s,
            end: e - 1,
            row: row_idx,
        })
    }

    chunks
}

fn part_2b(values: &[Vec<u32>]) -> usize {
    let mut basins: Vec<Vec<Chunk>> = Vec::new();

    for (idx, row) in values.iter().enumerate() {
        for chunk in get_chunks(row, idx) {
            match basins.iter_mut().find(|b| {
                b.iter()
                    .any(|c| idx > 0 && c.row == idx - 1 && chunk.overlaps(&c))
            }) {
                Some(b) => b.push(chunk),
                None => basins.push(vec![chunk]),
            };
        }
    }

    let mut sizes = basins
        .iter()
        .map(|b| b.iter().map(|c| c.len()).sum())
        .collect::<Vec<usize>>();
    sizes.sort();

    println!("{:?}", sizes.iter().rev().take(3).collect::<Vec<&usize>>());
    sizes.iter().rev().take(3).product()
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
    let p2b = part_2b(&values);

    assert_eq!(468, p1);
    assert_eq!(1280496, p2);
    assert_eq!(1280496, p2b);

    (p1, 0)
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
    use crate::{part_1, part_2, Chunk};

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

    #[test]
    fn test_chunk_overlaps() {
        let c1 = Chunk {
            start: 2,
            end: 4,
            row: 0,
        };
        let c2 = Chunk {
            start: 1,
            end: 5,
            row: 0,
        };
        assert!(c1.overlaps(&c2));
        assert!(c2.overlaps(&c1));
    }
}
