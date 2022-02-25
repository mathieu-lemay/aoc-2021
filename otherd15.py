#!/usr/bin/env python
# -*- coding: utf-8 -*-
""" Advent of Code 2021 Day 15 """

import numpy as np
import networkx as nx


class Grid:
    """ Day 15 AoC """
    def __init__(self, in_file: str):
        self.map = np.genfromtxt(in_file, delimiter=1, dtype=int)

    def create_graph(self):
        """ Get shortest_path"""
        self.graph = nx.DiGraph()

        for x in range(len(self.map)):
            for y in range(len(self.map)):

                if x < len(self.map) - 1:
                    self.graph.add_edge((x, y), (x + 1, y),
                                        weight=self.map[x + 1][y])

                    self.graph.add_edge((x + 1, y), (x, y),
                                        weight=self.map[x][y])

                if y < len(self.map) - 1:
                    self.graph.add_edge((x, y), (x, y + 1),
                                        weight=self.map[x][y + 1])

                    self.graph.add_edge((x, y + 1), (x, y),
                                        weight=self.map[x][y])

    def extend_map(self):
        """ Create 5x5 """

        new_map = self.map.copy()

        for i in range(4):
            new_map += 1
            new_map[np.where(new_map == 10)] = 1
            self.map = np.concatenate((self.map, new_map), axis=1)

        new_map = self.map.copy()

        for i in range(4):
            new_map += 1
            new_map[np.where(new_map == 10)] = 1
            self.map = np.concatenate((self.map, new_map), axis=0)

    def get_path_sum(self):
        """ Do the get_path_sum """
        my_sum = -self.map[0][0]
        for point in nx.shortest_path(self.graph,
                                      source=(0, 0),
                                      target=(len(self.map) - 1,
                                              len(self.map) - 1),
                                      weight='weight'):

            my_sum += self.map[point]

        return my_sum


def test_day_15():
    """ Run the tests """

    submarine = Grid("input15_test")
    submarine.create_graph()
    assert submarine.get_path_sum() == 40
    submarine.extend_map()
    submarine.create_graph()
    assert submarine.get_path_sum() == 315


def main():
    """ Run the exercise """
    submarine = Grid("input/day15.txt")
    submarine.create_graph()

    print(submarine.get_path_sum())
    # submarine.extend_map()
    # submarine.create_graph()
    # print(submarine.get_path_sum())


if __name__ == "__main__":
    main()
