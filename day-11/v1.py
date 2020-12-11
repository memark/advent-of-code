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


class NT(NamedTuple):
    val: int
    index: int


# class State(Enum):
#     RED = 1
#     GREEN = 2
#     BLUE = 3


def getAnswer_1(data: str) -> int:
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
    while changed:  # and runs < 2:
        runs += 1
        # print("run", runs)

        s2 = [s11.copy() for s11 in s1]
        changed = False
        for row in range(num_rows):
            for col in range(num_cols):
                if s1[row][col] == 'L' and num_nb(s1, row, col) == 0:
                    s2[row][col] = '#'
                    changed = True

                elif s1[row][col] == '#' and num_nb(s1, row, col) >= 4:
                    s2[row][col] = 'L'
                    changed = True

                # printt((row, col), num_nb(s1, row, col),
                #        s1[row][col], s2[row][col])
        s1 = s2
        # for s11 in s1:
        #     print("".join(s11))
        # print()

    # Count occupied
    o = 0
    for row in range(num_rows):
        for col in range(num_cols):
            seat = s1[row][col]
            if seat == '#':
                o += 1

    return o


def getAnswer_2(data: str) -> int:
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
                if s1[row][col] == 'L' and num_vis_nb(s1, row, col) == 0:
                    s2[row][col] = '#'
                    changed = True

                elif s1[row][col] == '#' and num_vis_nb(s1, row, col) >= 5:
                    s2[row][col] = 'L'
                    changed = True

                # printt((row, col), num_nb(s1, row, col),
                #        s1[row][col], s2[row][col])
        s1 = s2
        # for s11 in s1:
        #     print("".join(s11))
        # print()

    # Count occupied
    o = 0
    for row in range(num_rows):
        for col in range(num_cols):
            seat = s1[row][col]
            if seat == '#':
                o += 1

    return o


def num_vis_nb(matrix: list[list[str]], row: int, col: int):
    m = max(len(matrix), len(matrix[0]))

    debug = False  # row == 0 and col == 3

    def vis_occ(d_row: int, d_col: int):
        for i in range(1, m):  # Fundera på denna
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

    n = 0
    # N
    if vis_occ(-1, 0):
        if debug:
            print("N")
        n += 1
    # NE
    if vis_occ(-1, +1):
        if debug:
            print("NE")
        n += 1
    # E
    if vis_occ(0, +1):
        if debug:
            print("E")
        n += 1
    # SE
    if vis_occ(+1, +1):
        if debug:
            print("SE")
        n += 1
    # S
    if vis_occ(+1, 0):
        if debug:
            print("S")
        n += 1
    # SW
    if vis_occ(+1, -1):
        if debug:
            print("SW")
        n += 1
    # W
    if vis_occ(0, -1):
        if debug:
            print("W")
        n += 1
    # NW
    if vis_occ(-1, -1):
        if debug:
            print("NW")
        n += 1

    if debug:
        print("Tot", n)

    return n


def num_nb(matrix, row, col):
    n = 0
    # N
    if state(matrix, row - 1, col) == '#':
        n += 1
    # NE
    if state(matrix, row - 1, col + 1) == '#':
        n += 1
    # E
    if state(matrix, row, col + 1) == '#':
        n += 1
    # SE
    if state(matrix, row + 1, col + 1) == '#':
        n += 1
    # S
    if state(matrix, row + 1, col) == '#':
        n += 1
    # SW
    if state(matrix, row + 1, col - 1) == '#':
        n += 1
    # W
    if state(matrix, row, col - 1) == '#':
        n += 1
    # NW
    if state(matrix, row - 1, col - 1) == '#':
        n += 1

    return n


def state(matrix, row, col):
    if 0 <= row < len(matrix) and 0 <= col < len(matrix[0]):
        return matrix[row][col]
    return None


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    # printt("part one (sample)", getAnswer_1(sample_1))
    printt("part one (input)", getAnswer_1(data))

    # printt("part two (sample)", getAnswer_2(sample_1))
    printt("part two (input)", getAnswer_2(data))
