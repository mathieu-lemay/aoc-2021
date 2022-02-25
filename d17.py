#! /usr/bin/env python
import re
from typing import Tuple, List
from collections import namedtuple

from time import time as ts


area = namedtuple("Area", ["min_x", "max_x", "min_y", "max_y"])
point = namedtuple("Point", ["x", "y"])


def parse(input_):
    m = re.search("x=(-?[0-9]+)..(-?[0-9]+)", input_)
    assert m is not None
    min_x = int(m.group(1))
    max_x = int(m.group(2))

    m = re.search("y=(-?[0-9]+)..(-?[0-9]+)", input_)
    assert m is not None
    min_y = int(m.group(1))
    max_y = int(m.group(2))

    return area(min_x, max_x, min_y, max_y)


def in_target_area(pos, target) -> bool:
    return target.min_x <= pos.x <= target.max_x and target.min_y <= pos.y <= target.max_y


def passed_target_area(pos, target) -> bool:
    return pos.x > target.max_x or pos.y < target.min_y


def get_trajectory(velocity, target) -> Tuple[List[point], bool]:
    pos = point(0, 0)
    trajectory = [pos]

    while not in_target_area(pos, target):
        if passed_target_area(pos, target):
            return trajectory, False

        pos = point(pos.x + velocity.x, pos.y + velocity.y)
        velocity = point(velocity.x - 1 if velocity.x > 0 else 0, velocity.y - 1)

        trajectory.append(pos)

    return trajectory, True


def find_max_height(target) -> int:
    max_height = 0

    for y in range(abs(target.min_y), 0, -1):
        max_height_for_y = 0
        for x in range(1, target.max_x):
            traj, is_valid = get_trajectory(point(x, y), target)
            if not is_valid:
                if max_height_for_y > 0:
                    break
                else:
                    continue

            max_height_for_y = max(p.y for p in traj)

        if max_height_for_y > max_height:
            max_height = max_height_for_y
        elif max_height:
            return max_height

    return max_height




def find_number_of_good_velocities(target) -> int:
    c = 0

    for y in range(target.min_y, abs(target.min_y)):
        for x in range(0, target.max_x + 1):
            _, is_valid = get_trajectory(point(x, y), target)
            if is_valid:
                c += 1

    return c



if __name__ == "__main__":
    with open("input/day17.txt") as f:
        target_area = parse(f.read().strip())

    t = ts()
    p1 = find_max_height(target_area)
    t = ts() - t
    print(f"Part 1: {p1}\t{t*1000:.3f}ms")

    t = ts()
    p2 =find_number_of_good_velocities(target_area)
    t = ts() - t
    print(f"Part 2: {p2}\t{t*1000:.3f}ms")


class Test:
    input_ = "target area: x=20..30, y=-10..-5"

    def test_parse(self):
        target_area = parse(self.input_)

        assert target_area == (20, 30, -10, -5)

    def test_get_trajectory(self):
        target_area = parse(self.input_)

        assert get_trajectory(point(7, 2), target_area) == ([(0, 0), (7, 2), (13, 3), (18, 3), (22, 2), (25, 0), (27, -3), (28, -7)], True)
        assert get_trajectory(point(6, 3), target_area) == ([(0, 0), (6, 3), (11, 5), (15, 6), (18, 6), (20, 5), (21, 3), (21, 0), (21, -4), (21, -9)], True)
        assert get_trajectory(point(9, 0), target_area) == ([(0, 0), (9, 0), (17, -1), (24, -3), (30, -6)], True)
        assert get_trajectory(point(17, -4), target_area) == ([(0, 0), (17, -4), (33, -9)], False)

    def test_find_max_height(self):
        target_area = parse(self.input_)

        assert find_max_height(target_area) == 45

    def test_get_valid(self):
        target_area = parse(self.input_)

        assert find_number_of_good_velocities(target_area) == 112
