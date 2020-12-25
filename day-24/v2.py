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


Coord = tuple[int, int]


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    lines = [parseMoves(l) for l in data.splitlines()]

    blacks = set()

    def flip(tile, grid):
        if tile in grid:
            grid.remove(tile)
        else:
            grid.add(tile)

    for L in lines:
        q = 0
        r = 0
        # Skriv om med fold() eller reduce()
        for m in L:
            q += moves[m][0]
            r += moves[m][1]

        flip((q, r), blacks)

    if not isPartTwo:
        return len(blacks)

    def get_coords_to_check():
        # Skriv som comprehension
        # Tar det längre tid att först göra lista och sen köra set()?
        # Går det att få ut ett set av en comprehension?
        checks = set()
        for b in blacks:
            checks.add(b)
            # checks.union() här:
            for m in moves.values():
                checks.add((b[0]+m[0], b[1]+m[1]))
        return checks

    def get_black_neighbours(c: Coord):
        # return sum([1
        #             for m in moves.values()
        #             if (x := (c[0] + m[0], c[1] + m[1], c[2] + m[2])) != c and x in blacks])
        # We can stop at >2
        res = 0
        for m in moves.values():
            n = (c[0] + m[0], c[1] + m[1])
            if n != c and n in blacks:
                res += 1
            if res > 2:
                return res
        return res

    num_days = 100

    for day in range(1, num_days+1):
        tic = time.perf_counter()

        coords_to_check = get_coords_to_check()

        new_blacks = blacks.copy()
        for c in coords_to_check:
            black_neighbours = get_black_neighbours(c)
            # print(c, black_neighbours)

            # Any black tile with zero or more than 2 black tiles immediately adjacent to it
            # is flipped to white.
            if c in blacks and (black_neighbours == 0 or black_neighbours > 2):
                flip(c, new_blacks)

            # Any white tile with exactly 2 black tiles immediately adjacent to it
            # is flipped to black.
            if c not in blacks and black_neighbours == 2:
                flip(c, new_blacks)

        blacks = new_blacks

        toc = time.perf_counter()
        # print(
        #     f'Day {day}: {len(blacks)=} {len(coords_to_check)=} => {(toc-tic)*1000:.0f} ms => {(toc-tic)*(num_days-day):.0f} s remaining')

    return len(blacks)


def parseMoves(line: str) -> list[str]:
    return re.findall(r'e|se|sw|w|nw|ne', line)


moves = {
    'e':  [+1,  0],
    'se': [0,  +1],
    'sw': [-1, +1],
    'w':  [-1,  0],
    'nw': [0,  -1],
    'ne': [+1, -1],
}


def main() -> None:
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2, 1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3, 10)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 228)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_3, 566)  # 50
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_3, 2208)  # 100
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
