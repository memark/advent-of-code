# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import itertools
import math
from re import *
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
Tile 1:
##
##

Tile 2:
#.
#.

Tile 3:
##
..

Tile 4:
# .
.#
"""

sample_2 = """
Tile 2311:
..##.#..#.
# ..#.....
# ...##..#.
####.#...#
# .##.###.
##...#.###
.#.#.#..##
..#....#..
# ...#.#.
..###..###

Tile 1951:
# .##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
# .##.##.
.###....#.
..#.#..#.#
# ...##.#..

Tile 1171:
# ...##.
#..##.#..#
# .#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
# .##.####.
# ..#...
.....##...

Tile 1427:
# .##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
# .#.#....
..##...#..
.##..##...
..#...#...
# ...#.
#..#.#.#.#
...#.#.#..
# .#...##.
..##.##.##
# .##.#..

Tile 2473:
# ....####.
# ..#.##...
# .##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
# ...##.#.
..###.#.#.

Tile 2971:
..#.#....#
# ...###...
# .#.###...
# .##..#..
.#####..##
.#..####.#
# ..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
# .#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
# .#.#..
# .####...
# ..#.##..
# .##...##.

Tile 3079:
# .#.#####.
.#..######
..#.......
# ....
# .#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


# By rotating, flipping, and rearranging them,
# you can find a square arrangement
# that causes all adjacent borders to line up.


Id = str
Tile = list[str]


class ModId(NamedTuple):
    id: Id
    rot: int
    hflip: int
    vflip: int


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    tiles = [parseFrame(d) for d in data.strip().split('\n\n')]
    # print(f'{b=}')

    noot = len(tiles)
    th = len(tiles[0][1])
    tw = len(tiles[0][1][0])
    s_noot_h = int(math.sqrt(noot))
    s_noot_w = int(math.sqrt(noot))
    print(f'{noot=} {tw=} {th=} {s_noot_h=} {s_noot_w=}')

    return 42


# def a(tiles):
#     for t in tiles:


def parseFrame(s: str) -> Tuple[Id, Tile]:
    (id, *rest) = s.splitlines()
    id2 = re.search(r'Tile (\d+):', id).group(1)
    return (id2, rest)


def getFlipsEtc(ids: list[Id]) -> list[ModId]:
    c = [modId
         for id in ids
         for modId in getFlipsEtc2(id)]
    return c


def getFlipsEtc2(id: Id) -> list[ModId]:
    return [ModId(id, rt, hf, vf)
            for rt in [0, 1, 2, 3]
            for hf in [0, 1]
            for vf in [0, 1]]


# print(f'{len(getFlipsEtc(range(2)))=}')
# print(f'{len(getFlipsEtc(range(9)))=}')
# print(f'{len(getFlipsEtc(range(144)))=}')


def getModTile(tile: Tile, modId: ModId) -> Tile:
    def rot(tile: Tile) -> Tile:
        l = len(tile)
        temp = [['' for i in range(l)] for j in range(l)]
        for r in range(l):
            for c in range(l):
                temp[r][c] = tile[-1-c][r]
        res = [''.join(t) for t in temp]
        return res

    def hflip(tile: Tile) -> Tile:
        return [row[::-1] for row in tile]

    def vflip(tile: Tile) -> Tile:
        return list(reversed(tile))

    res = tile
    for _ in range(modId.rot):
        res = rot(res)
    for _ in range(modId.hflip):
        res = hflip(res)
    for _ in range(modId.vflip):
        res = vflip(res)
    return res


# print(getModTile(['ab', 'cd'], ModId("0", 0, 0, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 1, 0, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 2, 0, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 3, 0, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 4, 0, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 0, 1, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 0, 2, 0)))
# print(getModTile(['ab', 'cd'], ModId("0", 0, 0, 1)))
# print(getModTile(['ab', 'cd'], ModId("0", 0, 0, 2)))


def main() -> None:
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
