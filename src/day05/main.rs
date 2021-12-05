use std::cmp::{self, Ordering};
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Vector {
    a: Point,
    b: Point,
}

impl Vector {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_line(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    fn get_all_points(&self) -> Vec<Point> {
        let dx = self.b.x - self.a.x;
        let dy = self.b.y - self.a.y;

        if dx != 0 && dy != 0 && dx.abs() != dy.abs() {
            panic!("Invalid vector");
        }

        let dx = match dx.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        let dy = match dy.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        let mut x = self.a.x;
        let mut y = self.a.y;

        let mut pts = Vec::new();

        while x != self.b.x || y != self.b.y {
            pts.push(Point { x, y });

            x += dx;
            y += dy;
        }

        pts.push(Point { x, y });

        pts
    }
}

fn parse_input(input: &[String]) -> Vec<Vector> {
    input
        .iter()
        .map(|i| i.split_once(" -> ").unwrap())
        .map(|(a, b)| {
            let (x, y) = a.split_once(',').unwrap();
            let p1 = Point::new(x.parse().unwrap(), y.parse().unwrap());

            let (x, y) = b.split_once(',').unwrap();
            let p2 = Point::new(x.parse().unwrap(), y.parse().unwrap());

            Vector::new(p1, p2)
        })
        .collect::<Vec<Vector>>()
}

fn part_1(vectors: &[Vector], board: &mut Vec<i32>, width: usize) -> usize {
    for v in vectors.iter().filter(|v| v.is_line()) {
        for p in v.get_all_points() {
            board[p.x as usize * width + p.y as usize] += 1;
        }
    }

    board.iter().filter(|&&i| i > 1).count()
}

fn part_2(vectors: &[Vector], board: &mut Vec<i32>, width: usize) -> usize {
    for v in vectors.iter().filter(|v| !v.is_line()) {
        for p in v.get_all_points() {
            board[p.x as usize * width + p.y as usize] += 1;
        }
    }

    board.iter().filter(|&&i| i > 1).count()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let vectors = parse_input(input);

    let size_x = vectors
        .iter()
        .map(|v| cmp::max(v.a.x, v.b.x))
        .max()
        .unwrap() as usize
        + 1;
    let size_y = vectors
        .iter()
        .map(|v| cmp::max(v.a.y, v.b.y))
        .max()
        .unwrap() as usize
        + 1;

    let mut board = vec![0; (size_x * size_y) as usize];

    let p1 = part_1(&vectors, &mut board, size_y);
    let p2 = part_2(&vectors, &mut board, size_y);

    (p1, p2)
}

fn main() {
    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1, part_2, Point, Vector};

    static TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let res = parse_input(&input);

        assert_eq!(
            res,
            vec![
                Vector::new(Point::new(0, 9), Point::new(5, 9)),
                Vector::new(Point::new(8, 0), Point::new(0, 8)),
                Vector::new(Point::new(9, 4), Point::new(3, 4)),
                Vector::new(Point::new(2, 2), Point::new(2, 1)),
                Vector::new(Point::new(7, 0), Point::new(7, 4)),
                Vector::new(Point::new(6, 4), Point::new(2, 0)),
                Vector::new(Point::new(0, 9), Point::new(2, 9)),
                Vector::new(Point::new(3, 4), Point::new(1, 4)),
                Vector::new(Point::new(0, 0), Point::new(8, 8)),
                Vector::new(Point::new(5, 5), Point::new(8, 2)),
            ]
        );
    }

    #[test]
    fn test_get_all_points() {
        let v = Vector::new(Point::new(0, 0), Point::new(5, 5));
        assert_eq!(
            v.get_all_points(),
            vec![
                Point::new(0, 0),
                Point::new(1, 1),
                Point::new(2, 2),
                Point::new(3, 3),
                Point::new(4, 4),
                Point::new(5, 5),
            ]
        );

        let v = Vector::new(Point::new(5, 5), Point::new(0, 0));
        assert_eq!(
            v.get_all_points(),
            vec![
                Point::new(5, 5),
                Point::new(4, 4),
                Point::new(3, 3),
                Point::new(2, 2),
                Point::new(1, 1),
                Point::new(0, 0),
            ]
        );

        let v = Vector::new(Point::new(2, 9), Point::new(2, 3));
        assert_eq!(
            v.get_all_points(),
            vec![
                Point::new(2, 9),
                Point::new(2, 8),
                Point::new(2, 7),
                Point::new(2, 6),
                Point::new(2, 5),
                Point::new(2, 4),
                Point::new(2, 3),
            ]
        );

        let v = Vector::new(Point::new(5, 5), Point::new(7, 5));
        assert_eq!(
            v.get_all_points(),
            vec![Point::new(5, 5), Point::new(6, 5), Point::new(7, 5),]
        );

        let v = Vector::new(Point::new(0, 2), Point::new(2, 0));
        assert_eq!(
            v.get_all_points(),
            vec![Point::new(0, 2), Point::new(1, 1), Point::new(2, 0),]
        );
    }

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let mut board = vec![0; 100];

        let res = part_1(&parse_input(&input), &mut board, 10);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let mut board = vec![0; 100];

        part_1(&parse_input(&input), &mut board, 10);
        let res = part_2(&parse_input(&input), &mut board, 10);

        assert_eq!(res, 12);
    }
}
