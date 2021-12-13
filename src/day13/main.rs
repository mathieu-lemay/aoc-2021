use std::time::Instant;

use itertools::Itertools;

use aoc_2021::get_input;

#[derive(Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

impl TryFrom<&str> for Axis {
    type Error = String;

    fn try_from(s: &str) -> Result<Axis, Self::Error> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(format!("Invalid axis: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Fold {
    axis: Axis,
    position: u32,
}

fn parse_input(input: &[String]) -> (Vec<Point>, Vec<Fold>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    for s in input {
        if s.is_empty() {
            continue;
        }

        if s.starts_with("fold along") {
            let (a, p) = s[11..].split_once('=').unwrap();

            let axis = match Axis::try_from(a) {
                Ok(axis) => axis,
                Err(e) => panic!("{}", e),
            };

            let position = p.parse::<u32>().unwrap();

            folds.push(Fold { axis, position });
        } else {
            let (x, y) = s.split_once(',').unwrap();

            points.push(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }
    }

    (points, folds)
}

fn fold(points: &[Point], fold: &Fold) -> Vec<Point> {
    let points: Box<dyn Iterator<Item = Point>> = match fold.axis {
        Axis::X => Box::new(points.iter().map(|p| {
            if p.x > fold.position {
                Point {
                    x: fold.position * 2 - p.x,
                    y: p.y,
                }
            } else {
                p.clone()
            }
        })),
        Axis::Y => Box::new(points.iter().map(|p| {
            if p.y > fold.position {
                Point {
                    x: p.x,
                    y: fold.position * 2 - p.y,
                }
            } else {
                p.clone()
            }
        })),
    };

    points
        .sorted_by_key(|p| (p.x, p.y))
        .dedup_by(|a, b| a == b)
        .collect()
}

fn print_points(points: &[Point]) {
    let width = points.iter().map(|p| p.x).max().unwrap();
    let height = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            if points.iter().any(|p| p.x == x && p.y == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn part_1(points: &[Point], folds: &[Fold]) -> usize {
    let f = &folds[0];

    fold(points, f).len()
}

fn part_2(points: &[Point], folds: &[Fold]) -> Vec<Point> {
    let mut points = fold(points, &folds[0]);

    for f in folds.iter().skip(1) {
        points = fold(&points, f);
    }

    points
}

fn main() {
    let input = get_input("day13.txt");

    let start = Instant::now();

    let (points, folds) = parse_input(&input);

    let r1 = part_1(&points, &folds);
    let r2 = part_2(&points, &folds);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2:");
    print_points(&r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1, part_2, Axis, Fold, Point};

    static TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (points, folds) = parse_input(&input);

        assert_eq!(
            points,
            vec![
                Point { x: 6, y: 10 },
                Point { x: 0, y: 14 },
                Point { x: 9, y: 10 },
                Point { x: 0, y: 3 },
                Point { x: 10, y: 4 },
                Point { x: 4, y: 11 },
                Point { x: 6, y: 0 },
                Point { x: 6, y: 12 },
                Point { x: 4, y: 1 },
                Point { x: 0, y: 13 },
                Point { x: 10, y: 12 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 0 },
                Point { x: 8, y: 4 },
                Point { x: 1, y: 10 },
                Point { x: 2, y: 14 },
                Point { x: 8, y: 10 },
                Point { x: 9, y: 0 },
            ]
        );

        assert_eq!(
            folds,
            vec![
                Fold {
                    axis: Axis::Y,
                    position: 7
                },
                Fold {
                    axis: Axis::X,
                    position: 5
                },
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (points, folds) = parse_input(&input);

        assert_eq!(part_1(&points, &folds), 17)
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (points, folds) = parse_input(&input);

        assert_eq!(
            part_2(&points, &folds),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
                Point { x: 0, y: 4 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 4 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 4 },
                Point { x: 3, y: 0 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 0 },
                Point { x: 4, y: 1 },
                Point { x: 4, y: 2 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 4 }
            ]
        )
    }
}
