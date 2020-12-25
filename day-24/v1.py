# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from bisect import *
import cProfile
from collections import *
import functools
import operator
import itertools
import math
from re import *
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
esew
""".strip()

sample_2 = """
nwwswee
""".strip()

sample_3 = """
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
""".strip()


sample_4 = """
e
w
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


Coord = tuple[int, int, int]


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    lines = [parseMoves(l) for l in data.splitlines()]

    floor = set()

    def flip(tile, grid=floor):
        if tile in grid:
            grid.remove(tile)
        else:
            grid.add(tile)

    for L in lines:
        x = 0
        y = 0
        z = 0
        for m in L:
            x += moves[m][0]
            y += moves[m][1]
            z += moves[m][2]

        t = (x, y, z)

        flip(t)

    def count_black_tiles():
        return len(floor)

    if not isPartTwo:
        return count_black_tiles()

    def get_coords_to_consider() -> list[Coord]:
        max_x = max([c[0] for c in floor])
        max_y = max([c[1] for c in floor])
        max_z = max([c[2] for c in floor])

        min_x = min([c[0] for c in floor])
        min_y = min([c[1] for c in floor])
        min_z = min([c[2] for c in floor])

        return [
            (x, y, z)
            for x in range(min_x-1, max_x+1+1)
            for y in range(min_y-1, max_y+1+1)
            for z in range(min_z-1, max_z+1+1)
        ]

    def get_color(c: Coord):
        # All tiles start white.
        return 'b' if c in floor else 'w'

    def get_black_neighbours(c: Coord):
        # return sum([1
        #             for m in moves.values()
        #             if (x := (c[0] + m[0], c[1] + m[1], c[2] + m[2])) != c and x in floor])
        # We can stop at >2
        res = 0
        for m in moves.values():
            x = (c[0] + m[0], c[1] + m[1], c[2] + m[2])
            if x != c and x in floor:
                res += 1
            if res > 2:
                return res
        return res

    num_days = 100

    for day in range(1, num_days+1):
        # tic = time.perf_counter()

        coords_to_consider = get_coords_to_consider()

        new_floor = floor.copy()

        for c in coords_to_consider:
            black_neighbours = get_black_neighbours(c)

            # Any black tile with zero or more than 2 black tiles immediately adjacent to it
            # is flipped to white.
            if c in floor and (black_neighbours == 0 or black_neighbours > 2):
                flip(c, new_floor)

            # Any white tile with exactly 2 black tiles immediately adjacent to it
            # is flipped to black.
            if c not in floor and black_neighbours == 2:
                flip(c, new_floor)

        floor = new_floor

        # toc = time.perf_counter()
        # print(
        #     f'Day {day}: {count_black_tiles()=} {len(coords_to_consider)=} => {(toc-tic)*1000:.0f} ms => {(toc-tic)*(num_days-day):.0f} s remaining')

    return count_black_tiles()


def parseMoves(line: str) -> list[str]:
    return re.findall(r'e|se|sw|w|nw|ne', line)


moves = {
    'e':  [+1, -1,  0],
    'se': [0,  -1, +1],
    'sw': [-1,  0, +1],
    'w':  [-1, +1,  0],
    'nw': [0,  +1, -1],
    'ne': [+1,  0, -1],
}


def main() -> None:
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2, 1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3, 10)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 228)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_3, 2208)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_4)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 3672)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
