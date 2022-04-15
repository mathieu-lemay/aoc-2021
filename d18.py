#! /usr/bin/python
from dataclasses import dataclass
from typing import Optional, List
from copy import deepcopy


@dataclass
class Node:
    value: Optional[int] = None
    a: Optional['Node'] = None
    b: Optional['Node'] = None

    def is_leaf(self) -> bool:
        return self.a is None and self.b is None

    def to_leaf(self, value):
        self.value = value
        self.a = None
        self.b = None

    def __repr__(self):
        if self.is_leaf():
            return f'Node(value={self.value})'
        else:
            return f"Node(a={self.a}, b={self.b})"

    def __str__(self):
        if self.is_leaf():
            return str(self.value)
        else:
            return f"[{self.a}, {self.b}]"


def parse(input_):
    t = 0
    for idx, c in enumerate(input_):
        if c == "," and t == 1:
            i = idx
            break
        elif c == "[":
            t += 1
        elif c == "]":
            t -= 1
    else:
        raise Exception(f"Error parsing {input_}")

    x = input_[1:i]
    y = input_[i + 1: -1]

    node = Node(None)

    if "," in x:
        x = parse(x)
    else:
        x = Node(int(x), None, None)

    if "," in y:
        y = parse(y)
    else:
        y = Node(int(y), None, None)

    node.a = x
    node.b = y

    return node


def flatten_tree(node):
    nodes = []

    if not node:
        return nodes

    if node.is_leaf():
        return [node]

    if node.a:
        nodes += flatten_tree(node.a)

    if node.b:
        nodes += flatten_tree(node.b)

    return nodes


def explode_recur(node, level=0):
    x = a = b = None

    if level >= 4:
        a = node.a.value
        b = node.b.value

        node.to_leaf(0)

        return node, a, b

    if not node.a.is_leaf():
        x, a, b = explode_recur(node.a, level + 1)
        if x:
            return x, a, b

    if not node.b.is_leaf():
        x, a, b = explode_recur(node.b, level + 1)
        if x:
            return x, a, b

    return x, a, b


def explode(root):
    node, a, b = explode_recur(root)

    if not node:
        return root, False

    flattened = flatten_tree(root)
    idx, _ = next(((i, n) for i, n in enumerate(flattened) if n is node), (None, None))

    if a and idx > 0:
        flattened[idx - 1].value += a

    if b and idx < len(flattened) - 1:
        flattened[idx + 1].value += b

    return root, True


def split(root):
    flattened = flatten_tree(root)
    for n in flattened:
        if n.is_leaf() and n.value >= 10:
            a = b = n.value // 2
            if n.value % 2 == 1:
                b += 1

            n.value = None
            n.a = Node(a)
            n.b = Node(b)

            return root, True

    return root, False


def reduce(root):
    while True:
        root, did_explode = explode(root)
        if did_explode:
            continue

        root, did_split = split(root)
        if did_split:
            continue

        break

    return root


def solve(equation: List[Node]):
    assert len(equation) > 1

    root = equation[0]

    for val in equation[1:]:
        root = Node(a=root, b=val)
        root = reduce(root)

    return root


def magnitude(node) -> int:
    if node.is_leaf():
        return node.value

    return 3 * magnitude(node.a) + 2 * magnitude(node.b)


def permutations(equation: List[Node]):
    max_ = 0

    for i in range(len(equation) - 1):
        for j in range(i+1, len(equation)):
            if i == j:
                continue

            m = magnitude(solve([deepcopy(equation[i]), deepcopy(equation[j])]))
            if m > max_:
                max_ = m

            m = magnitude(solve([deepcopy(equation[j]), deepcopy(equation[i])]))
            if m > max_:
                max_ = m

    return max_


if __name__ == '__main__':
    with open("input/day18.txt") as f:
        equation = list(map(lambda l: parse(l.strip()), f.readlines()))

    solved = solve(deepcopy(equation))

    print(magnitude(solved))
    print(permutations(equation))


class Test:
    def test_parse(self):
        assert parse("[1,2]") == Node(a=Node(1), b=Node(2))
        assert parse("[[1,2],3]") == Node(a=Node(a=Node(1), b=Node(2)), b=Node(3))
        assert parse("[9,[8,7]]") == Node(a=Node(9), b=Node(a=Node(8), b=Node(7)))

    def test_explode(self):
        for orig, exploded in (
                ('[[[[[9,8],1],2],3],4]', '[[[[0,9],2],3],4]'),
                ('[7,[6,[5,[4,[3,2]]]]]', '[7,[6,[5,[7,0]]]]'),
                ('[[6,[5,[4,[3,2]]]],1]', '[[6,[5,[7,0]]],3]'),
                ('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]', '[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]'),
                ('[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]', '[[3,[2,[8,0]]],[9,[5,[7,0]]]]'),
        ):
            assert explode(parse(orig)) == (parse(exploded), True)

    def test_split(self):
        for orig, splitted in (
                ('[[[[0,7],4],[15,[0,13]]],[1,1]]', '[[[[0,7],4],[[7,8],[0,13]]],[1,1]]'),
                ('[[[[0,7],4],[[7,8],[0,13]]],[1,1]]', '[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]'),
        ):
            assert split(parse(orig)) == (parse(splitted), True)

    def test_reduce(self):
        assert reduce(parse('[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]')) == parse('[[[[0,7],4],[[7,8],[6,0]]],[8,1]]')

    def test_solve(self):
        equation = [parse(l.strip()) for l in """
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]
        """.split("\n") if l.strip()]

        assert solve(equation) == parse('[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]')

    def test_magnitude(self):
        assert magnitude(parse('[[1,2],[[3,4],5]]')) == 143
        assert magnitude(parse('[[[[0,7],4],[[7,8],[6,0]]],[8,1]]')) == 1384
        assert magnitude(parse('[[[[1,1],[2,2]],[3,3]],[4,4]]')) == 445
        assert magnitude(parse('[[[[3,0],[5,3]],[4,4]],[5,5]]')) == 791
        assert magnitude(parse('[[[[5,0],[7,4]],[5,5]],[6,6]]')) == 1137
        assert magnitude(parse('[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]')) == 3488

        equation = [parse(l.strip()) for l in """
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        """.split("\n") if l.strip()]

        assert magnitude(solve(equation)) == 4140

    def test_permutations(self):
        equation = [parse(l.strip()) for l in """
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        """.split("\n") if l.strip()]

        assert permutations(equation) == 3993
