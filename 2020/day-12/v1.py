# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import time
from enum import Enum
import itertools
from typing import *


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


sample_1 = """
F10
N3
F7
R90
F11
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


class I(NamedTuple):
    c: str
    n: int


dirList = [
    ['N',  -1,  0],
    ['E',   0, +1],
    ['S',  +1,  0],
    ['W',   0, -1],
]

# c   ns   ew
dirDict = {
    'N':  [-1,  0],
    'E':  [0, +1],
    'S':  [+1,  0],
    'W':  [0, -1],
}

# print(dirList)
# print(dirDict)


def newDir(oldDir: str, lr: str, deg: int):
    oldIndex = next(dirList.index(d) for d in dirList if d[0] == oldDir)
    deltaIndex = (deg % 360) // 90
    newIndex = \
        (oldIndex + deltaIndex) % 4 \
        if lr == 'R' \
        else (oldIndex - deltaIndex) % 4
    return dirList[newIndex][0]


def getAnswer_1(data: str) -> int:
    dd = [I(d[0], int(d[1:])) for d in data.strip().split("\n")]

    ew = 0
    ns = 0
    dir = 'E'

    # Action N means to move north by the given value.
    # Action S means to move south by the given value.
    # Action E means to move east by the given value.
    # Action W means to move west by the given value.
    # Action L means to turn left the given number of degrees.
    # Action R means to turn right the given number of degrees.
    # Action F means to move forward by the given value in the direction the ship is currently facing.

    for d in dd:
        if d.c in ['N', 'S', 'E', 'W']:
            ns += dirDict[d.c][0] * d.n
            ew += dirDict[d.c][1] * d.n

        if d.c in ['L', 'R']:
            dir = newDir(dir, d.c, d.n)

        if d.c == 'F':
            ns += dirDict[dir][0] * d.n
            ew += dirDict[dir][1] * d.n
        print((ew, ns))

    mp = abs(ew) + abs(ns)
    printt(ew, ns, mp)

    return mp


# c   ns   ew
def getAnswer_2(data: str) -> int:
    dd = [I(d[0], int(d[1:])) for d in data.strip().split("\n")]

    wp_ew: int = 10
    wp_ns: int = 1

    ew: int = 0
    ns: int = 0

    for d in dd:
        if d.c in ['N', 'S', 'E', 'W']:
            # Action N means to move the waypoint north by the given value.
            # Action S means to move the waypoint south by the given value.
            # Action E means to move the waypoint east by the given value.
            # Action W means to move the waypoint west by the given value.

            if d.c == 'N':
                wp_ns += d.n
            if d.c == 'E':
                wp_ew += d.n
            if d.c == 'S':
                wp_ns -= d.n
            if d.c == 'W':
                wp_ew -= d.n

        if d.c in ['L', 'R']:
            # Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
            # Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.

            if d.n == 180:
                wp_ns = -wp_ns
                wp_ew = -wp_ew

            if d.n == 360:
                pass

            if (d.n == 90 and d.c == 'R') or (d.n == 270 and d.c == 'L'):
                wp_ew_o = wp_ew
                wp_ns_o = wp_ns

                wp_ew = +wp_ns_o
                wp_ns = -wp_ew_o

            if (d.n == 90 and d.c == 'L') or (d.n == 270 and d.c == 'R'):
                wp_ew_o = wp_ew
                wp_ns_o = wp_ns

                wp_ew = -wp_ns_o
                wp_ns = +wp_ew_o

        if d.c == 'F':
            # Action F means to move forward to the waypoint a number of times equal to the given value.

            ew += wp_ew * d.n
            ns += wp_ns * d.n

        # print((ew, ns))
        # printt(d, (wp_ew, wp_ns), (ew, ns))

    mp = abs(ew) + abs(ns)
    printt(ew, ns, mp)

    return mp


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    # printt("part one (sample)", getAnswer_1(sample_1))
    printt("part one (input)", getAnswer_1(data))

    # printt("part two (sample)", getAnswer_2(sample_1))
    printt("part two (input)", getAnswer_2(data))
