# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import time
from enum import Enum
import itertools
from typing import *


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


sample_1 = """
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


Matrix = list[list[str]]


def getAnswer_1(data: str) -> int:
    def num_nb(matrix: Matrix, row: int, col: int):
        return num_vis_nb_combined(matrix, row, col, 1)

    return getAnswer(data, 4, num_nb)


def getAnswer_2(data: str) -> int:
    def num_vis_nb(matrix: Matrix, row: int, col: int):
        return num_vis_nb_combined(matrix, row, col, None)

    return getAnswer(data, 5, num_vis_nb)


def num_vis_nb_combined(matrix: Matrix, row: int, col: int, maxSteps: Optional[int]):
    return countTrue(occ_in_dir(
        d[1], d[2], maxSteps, matrix, row, col) for d in dirs)


def countTrue(g: Generator[bool, None, None]) -> int:
    return len(list(filter(bool, g)))


def getAnswer(data: str, minOccForChange: int, countNeighborFunc: Callable[[Matrix, int, int], int]) -> int:
    g = [[col for col in row] for row in data.strip().split("\n")]
    # print("starting")
    # for g1 in g:
    #     print("".join(g1))
    # print()

    num_rows = len(g)
    num_cols = len(g[0])

    # Run simulation
    s1 = g
    runs = 0
    changed = True
    while changed:  # and runs < 3:
        runs += 1
        # print("run", runs)

        s2 = [s11.copy() for s11 in s1]
        changed = False
        for row in range(num_rows):
            for col in range(num_cols):
                if s1[row][col] == 'L' and countNeighborFunc(s1, row, col) == 0:
                    s2[row][col] = '#'
                    changed = True

                elif s1[row][col] == '#' and countNeighborFunc(s1, row, col) >= minOccForChange:
                    s2[row][col] = 'L'
                    changed = True

                # printt((row, col), num_nb(s1, row, col), s1[row][col], s2[row][col])
        s1 = s2
        # for s11 in s1:
        #     print("".join(s11))
        # print()

    num_occ = len([1 for row in s1 for col in row if col == '#'])

    return num_occ


def occ_in_dir(d_row: int, d_col: int, maxSteps: Optional[int], matrix: Matrix, row: int, col: int):
    m = max(len(matrix), len(matrix[0]))
    debug = False

    end = (maxSteps or m) + 1
    for i in range(1, end):  # Fundera på denna
        new_row = row + d_row * i
        new_col = col + d_col * i
        if 0 <= new_row < len(matrix) and 0 <= new_col < len(matrix[0]):
            if debug:
                print("trying", (new_row, new_col))
            if matrix[new_row][new_col] == '#':
                return True
            if matrix[new_row][new_col] == 'L':
                return False

    return False


dirs = [
    ['N',  -1,  0],
    ['NE', -1, +1],
    ['E',   0, +1],
    ['SE', +1, +1],
    ['S',  +1,  0],
    ['SW', +1, -1],
    ['W',   0, -1],
    ['NW', -1, -1],
]


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    # printt("part one (sample)", getAnswer_1(sample_1))
    printt("part one (input)", getAnswer_1(data))

    # printt("part two (sample)", getAnswer_2(sample_1))
    printt("part two (input)", getAnswer_2(data))
