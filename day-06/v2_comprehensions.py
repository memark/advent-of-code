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

b"""

data = open('input.txt').read()


def getAnswer_1(data: str) -> int:
    groups = parseData(data)
    result = [len(getQwithYforGroup(group)) for group in groups]
    return sum(result)


def parseData(data: str) -> list[list[list[str]]]:
    return [[list(guy) for guy in d.split("\n")] for d in data.strip().split("\n\n")]


# [['a', 'b'], ['a', 'c']] -> 3
def getQwithYforGroup(group: list[list[str]]) -> set[str]:
    result = set([a for guy in group for a in guy])
    # print(group, "=>", result)
    return result


def getAnswer_2(data: str) -> int:
    groups = parseData(data)
    result = [len(getQwithAllYforGroup(group)) for group in groups]
    return sum(result)


# [['a', 'b'], ['a', 'c']] -> 1
def getQwithAllYforGroup(group: list[list[str]]) -> list[str]:
    allA = [a for guy in group for a in guy]
    allUniqueA = set(allA)
    groupSize = len(group)
    result = [a for a in allUniqueA if allA.count(a) == groupSize]
    # print(group, "=>", result)
    return result


if __name__ == "__main__":
    # data = sample
    data = open('input.txt').read()
    print("part one", getAnswer_1(data))
    print("part two", getAnswer_2(data))
