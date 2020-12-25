# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import operator
import builtins
import re
from functools import *
import time
from typing import *


sample_1 = """
.#.
..#
###
""".strip()

sample_2 = """
""".strip()

input = open('input.txt').read().strip()


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: Sequence):
    print("\n".join(str(x1) for x1 in x))


class Coord(NamedTuple):
    x: int
    y: int
    z: int
    w: int


State = Literal['.', '#']


Matrix = dict[Coord, State]


def printMatrix(matrix: Matrix) -> None:
    xs = [coord.x for coord in matrix]
    ys = [coord.y for coord in matrix]
    zs = [coord.z for coord in matrix]
    ws = [coord.w for coord in matrix]

    for w in range(min(ws), max(ws)+1):
        for z in range(min(zs), max(zs)+1):
            print(f"{z=}, {w=}")
            for y in range(min(ys), max(ys)+1):
                states = [matrix[Coord(coord.x, y, z, w)]
                          for coord in matrix if coord.w == w and coord.z == z and coord.y == y]
                print("".join(states))
                # for x in range(min(xs), max(xs)):
            print()


numCycles = 6


def getAnswer_2(data: str) -> int:
    d = [[ddd for ddd in dd] for dd in data.split('\n')]

    def createMatrix() -> Matrix:
        res = {}
        for y in range(len(d)):
            for x in range(len(d[y])):
                z = 0
                w = 0
                res[Coord(x, y, z, w)] = '#' if d[x][y] == '#' else '.'
        return res

    matrix: Matrix = createMatrix()
    print('Before any cycles:\n')
    printMatrix(matrix)

    def getState(coord: Coord) -> State:
        return '#' if coord in matrix and matrix[coord] == '#' else '.'

    def activeNeighbours(coord: Coord) -> int:
        neighbours = [newCoord
                      for w in range(coord.w-1, coord.w+1+1)
                      for z in range(coord.z-1, coord.z+1+1)
                      for y in range(coord.y-1, coord.y+1+1)
                      for x in range(coord.x-1, coord.x+1+1)
                      if (newCoord := Coord(x, y, z, w)) != coord]

        res = [getState(n) for n in neighbours].count('#')
        return res

    def getNewState(coord: Coord) -> State:
        # If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
        if getState(coord) == '#':
            return '#' if activeNeighbours(coord) in [2, 3] else '.'
        # If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        else:
            return '#' if activeNeighbours(coord) in [3] else '.'

    def getCoordsToCheck() -> list[Coord]:
        ws = [coord.w for coord in matrix]
        zs = [coord.z for coord in matrix]
        ys = [coord.y for coord in matrix]
        xs = [coord.x for coord in matrix]
        return [Coord(x, y, z, w)
                for w in range(min(ws)-1, max(ws)+1+1)
                for z in range(min(zs)-1, max(zs)+1+1)
                for y in range(min(ys)-1, max(ys)+1+1)
                for x in range(min(xs)-1, max(xs)+1+1)]

    for cycle in range(1, numCycles+1):
        coordsToCheck: list[Coord] = getCoordsToCheck()
        print(f"{cycle=}: Checking {len(coordsToCheck)} possible cubes")
        # print(f"{coordsToCheck=}")

        newMatrix: Matrix = {}
        for coord in coordsToCheck:
            newMatrix[coord] = getNewState(coord)

        # printMatrix(newMatrix)
        print(
            f"{cycle=}: Went from {list(matrix.values()).count('#')} to {list(newMatrix.values()).count('#')} active cubes")
        matrix = newMatrix

    return list(matrix.values()).count('#')


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    printt(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
