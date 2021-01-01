# pyright: reportWildcardImportFromLibrary=false

from typing import *


def getID(code: str):
    b = code \
        .replace("F", "0").replace("B", "1") \
        .replace("L", "0").replace("R", "1")
    return int(b, 2)


def getAnswer_1(data: List[str]):
    return max(map(getID, data))


def getAnswer_2(data: List[str]):
    IDs = set(map(getID, data))
    IDRange = set(range(min(IDs), max(IDs)))
    def isMiddle(x): return x - 1 in IDs and x + 1 in IDs
    return next(filter(isMiddle, IDRange - IDs))


if __name__ == "__main__":
    data = open('input.txt').read().splitlines()

    print("part one", getAnswer_1(data))
    print("part two", getAnswer_2(data))
