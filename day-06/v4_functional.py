# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from typing import *


sample = """abc

a
b
c

ab
ac

a
a
a
a

b
"""


def parseData(data: str) -> list[list[str]]:
    return [[guy for guy in d.split("\n")] for d in data.strip().split("\n\n")]


def getAnswer_1(data: str) -> int:
    return sum(map(getYForAnyInGroup, parseData(data)))


# [['ab'], ['ac']] -> 3
def getYForAnyInGroup(group: list[str]) -> int:
    result = "".join(set("".join(group)))
    print(f"{group} => {result} => {len(result)}")
    return len(result)


def getAnswer_2(data: str) -> int:
    return sum(map(getYForAllInGroup, parseData(data)))


# [['ab'], ['ac']] -> 1
def getYForAllInGroup(group: list[str]) -> int:
    allY = [a for g in group for a in g]
    result = list(filter(lambda a: allY.count(a) == len(group), set(allY)))
    # print(f"* {'-'.join(group)} => {''.join(sorted(set(allY)))} => {''.join(sorted(allY))} => {''.join(sorted(result))} => {len(result)}")
    return len(result)


if __name__ == "__main__":
    # data = sample
    data = open('input.txt').read()
    print()
    print("part one", getAnswer_1(data))
    print()
    print("part two", getAnswer_2(data))
