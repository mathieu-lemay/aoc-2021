#! /usr/bin/env python
from collections import defaultdict
from copy import deepcopy
from time import time as ts


def parse(input_):
    pairs = defaultdict(lambda: 0)
    template = input_[0]
    for a, b in zip(template, template[1:] + " "):
        pairs[a + b] += 1

    rules = {}
    for s in input_[2:]:
        a, b = s.split(" -> ")
        rules[a] = b

    return pairs, rules


def expand(pairs, rules, iterations):
    for _ in range(iterations):
        new_pairs = deepcopy(pairs)

        for (pattern, char) in rules.items():
            if pattern not in pairs:
                continue

            c = pairs[pattern]

            new_pairs[pattern] -= c

            new_pairs[pattern[0] + char] += c
            new_pairs[char + pattern[1]] += c

        pairs = new_pairs

    return pairs


def get_diff_of_elements(pairs, rules, iterations):
    pairs = expand(pairs, rules, iterations)

    counts = defaultdict(lambda: 0)
    for p, c in pairs.items():
        counts[p[0]] += c

    return max(counts.values()) - min(counts.values())


if __name__ == "__main__":
    with open("input/day14.txt") as f:
        template, rules = parse(f.read().strip().split("\n"))

    t = ts()
    p1 = get_diff_of_elements(template, rules, 10)
    p2 = get_diff_of_elements(template, rules, 40)

    t = ts() - t

    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")
    print(f"Duration: {t*1000.0:.3f}ms")


class Test:
    input_ = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C""".split(
        "\n"
    )

    def test_parse(self):
        pairs, rules = parse(self.input_)

        assert pairs == {"NN": 1, "NC": 1, "CB": 1, "B ": 1}
        assert rules == {
            "CH": "B",
            "HH": "N",
            "CB": "H",
            "NH": "C",
            "HB": "C",
            "HC": "B",
            "HN": "C",
            "NN": "C",
            "BH": "H",
            "NC": "B",
            "NB": "B",
            "BN": "B",
            "BB": "N",
            "BC": "B",
            "CC": "N",
            "CN": "C",
        }

    def test_part_1(self):
        template, rules = parse(self.input_)

        assert get_diff_of_elements(template, rules, 10) == 1588

    def test_part_2(self):
        template, rules = parse(self.input_)

        assert get_diff_of_elements(template, rules, 40) == 2188189693529

    def test_expand(self):
        template, rules = parse(self.input_)

        assert expand(template, rules, 10) == {
            "B ": 1,
            "BB": 812,
            "BC": 120,
            "BH": 81,
            "BN": 735,
            "CB": 115,
            "CC": 60,
            "CH": 21,
            "CN": 102,
            "HB": 26,
            "HC": 76,
            "HH": 32,
            "HN": 27,
            "NB": 796,
            "NC": 42,
            "NH": 27,
            "NN": 0,
        }
