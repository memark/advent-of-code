# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from functools import *
import time
from typing import *


sample_1 = """
.#.
..#
###
""".strip()


input = open('input.txt').read().strip()


class Coord(NamedTuple):
    x: int
    y: int
    z: int
    w: int


def getAnswer_2(data: str) -> int:
    d = [[ddd for ddd in dd] for dd in data.split('\n')]

    actives = {Coord(x, y, 0, 0)
               for y in range(len(d))
               for x in range(len(d[y]))
               if d[x][y] == '#'
               }
    # print('Before any cycles:\n')
    # printMatrix(actives)

    numCycles = 6
    res = recurse(actives, numCycles)

    return len(res)


def printMatrix(actives: set[Coord]) -> None:
    xs = [coord.x for coord in actives]
    ys = [coord.y for coord in actives]
    zs = [coord.z for coord in actives]
    ws = [coord.w for coord in actives]

    for w in range(min(ws), max(ws)+1):
        for z in range(min(zs), max(zs)+1):
            print(f"{z=}, {w=}")
            for y in range(min(ys), max(ys)+1):
                states = ['#' if Coord(x, y, z, w) in actives else '.'
                          for x in range(min(xs), max(xs)+1)]
                print("".join(states))
            print()


def recurse(actives: set[Coord], maxCycles: int, cycle: int = 1):
    if cycle > maxCycles:
        return actives

    coordsToCheck = {aa for a in actives for aa in [a, *getNeighbours(a)]}
    print(f"{cycle=}: checking {len(coordsToCheck)} relevant coords")

    newActives = {c for c in coordsToCheck if getNewState(c, actives)}
    print(
        f"{cycle=}: going from {len(actives)} to {len(newActives)} active cubes")
    print()

    return recurse(newActives, maxCycles, cycle+1)


def getNewState(coord: Coord, actives: set[Coord]):
    activeNeighbours = sum((n in actives) for n in getNeighbours(coord))

    # If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
    # Otherwise, the cube becomes inactive.
    if coord in actives:
        return activeNeighbours in [2, 3]

    # If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
    # Otherwise, the cube remains inactive.
    else:
        return activeNeighbours in [3]


@cache
def getNeighbours(coord: Coord):
    res = [Coord(x, y, z, w)
           for w in range(coord.w-1, coord.w+1+1)
           for z in range(coord.z-1, coord.z+1+1)
           for y in range(coord.y-1, coord.y+1+1)
           for x in range(coord.x-1, coord.x+1+1)]
    res.remove(coord)
    return res


def main():
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input)

    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 2192)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
