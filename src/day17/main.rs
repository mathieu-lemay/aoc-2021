use std::fmt::Display;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;
use rusttype::{point, vector, Point, Rect, Vector};

use aoc_2021::get_input_as_string;

fn parse(input: &str) -> Rect<i32> {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let cap = re.captures(input).unwrap();

    Rect {
        min: point(cap[1].parse().unwrap(), cap[3].parse().unwrap()),
        max: point(cap[2].parse().unwrap(), cap[4].parse().unwrap()),
    }
}

fn rect_contains<N: Ord>(rect: &Rect<N>, point: &Point<N>) -> bool {
    point.x >= rect.min.x && point.x <= rect.max.x && point.y >= rect.min.y && point.y <= rect.max.y
}

fn point_is_past_rect<N: Ord>(rect: &Rect<N>, point: &Point<N>) -> bool {
    point.x > rect.max.x || point.y < rect.min.y
}

fn get_trajectory(velocity: &mut Vector<i32>, target: &Rect<i32>) -> (Vec<Point<i32>>, bool) {
    let mut pos = point(0, 0);

    let mut trajectory = vec![pos];

    while !rect_contains(target, &pos) {
        if point_is_past_rect(target, &pos) {
            return (trajectory, false);
        }

        pos.x += velocity.x;
        pos.y += velocity.y;

        velocity.x = if velocity.x > 0 { velocity.x - 1 } else { 0 };
        velocity.y -= 1;

        trajectory.push(pos);
    }

    (trajectory, true)
}

fn find_max_possible_height(target: &Rect<i32>) -> i32 {
    let mut max_height = 0;

    for y in 1..=target.min.y.abs() {
        for x in 1..=target.max.x {
            let (traj, is_valid) = get_trajectory(&mut vector(x, y), target);
            if !is_valid {
                continue;
            }

            let max_height_for_velocity = traj.iter().map(|p| p.y).max().unwrap();

            if max_height_for_velocity > max_height {
                max_height = max_height_for_velocity;
            }
        }
    }

    max_height
}

fn get_number_of_valid_velocities(target: &Rect<i32>) -> usize {
    (1..=target.max.x)
        .cartesian_product(target.min.y..=target.min.y.abs())
        .map(|(x, y)| get_trajectory(&mut vector(x, y), target))
        .filter(|(_, valid)| *valid)
        .count()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let target = parse(input);

    let p1 = find_max_possible_height(&target);
    let p2 = get_number_of_valid_velocities(&target);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day17.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use rusttype::{point, vector, Rect};

    use crate::{find_max_possible_height, get_number_of_valid_velocities, get_trajectory, parse};

    static TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse(TEST_INPUT),
            Rect {
                min: point(20, -10),
                max: point(30, -5)
            }
        );
    }

    #[test]
    fn test_get_trajectory() {
        let target_area = parse(TEST_INPUT);

        assert_eq!(
            get_trajectory(&mut vector(7, 2), &target_area),
            (
                vec![
                    point(0, 0),
                    point(7, 2),
                    point(13, 3),
                    point(18, 3),
                    point(22, 2),
                    point(25, 0),
                    point(27, -3),
                    point(28, -7)
                ],
                true
            )
        );
        assert_eq!(
            get_trajectory(&mut vector(6, 3), &target_area),
            (
                vec![
                    point(0, 0),
                    point(6, 3),
                    point(11, 5),
                    point(15, 6),
                    point(18, 6),
                    point(20, 5),
                    point(21, 3),
                    point(21, 0),
                    point(21, -4),
                    point(21, -9)
                ],
                true
            )
        );
        assert_eq!(
            get_trajectory(&mut vector(9, 0), &target_area),
            (
                vec![
                    point(0, 0),
                    point(9, 0),
                    point(17, -1),
                    point(24, -3),
                    point(30, -6)
                ],
                true
            )
        );
        assert_eq!(
            get_trajectory(&mut vector(17, -4), &target_area),
            (vec![point(0, 0), point(17, -4), point(33, -9)], false)
        );
    }

    #[test]
    fn test_get_number_of_valid_velocities() {
        let target_area = parse(TEST_INPUT);

        assert_eq!(find_max_possible_height(&target_area), 45);
    }

    #[test]
    fn test_find_max_possible_height() {
        let target_area = parse(TEST_INPUT);

        assert_eq!(get_number_of_valid_velocities(&target_area), 112);
    }
}
