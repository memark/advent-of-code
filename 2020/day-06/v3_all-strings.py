# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import string
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


def parseData(data: str) -> list[list[str]]:
    return [[guy for guy in d.split("\n")] for d in data.split("\n\n")]


def getAnswer_1(data: str) -> int:
    groups = parseData(data)
    result = [len(getQwithAnyYforGroup(group)) for group in groups]
    return sum(result)


# [['ab'], ['ac']] -> 3
def getQwithAnyYforGroup(group: list[str]) -> str:
    result = "".join(set("".join(group)))
    print(f"{group} => {result} => {len(result)}")
    return result


def getAnswer_2(data: str) -> int:
    groups = parseData(data)
    result = [len(getQwithAllYforGroup(group)) for group in groups]
    return sum(result)


# [['ab'], ['ac']] -> 1
def getQwithAllYforGroup(group: list[str]) -> str:
    allA = "".join(group)
    allQ = "".join(set(allA))
    groupSize = len(group)
    result = "".join([a for a in allQ if allA.count(a) == groupSize])
    print(f"{group} => {allA} => {allQ} => {result} => {len(result)}")
    return result


# print(getAnswer_2(sample))


# 3121 fel, 06.30
# 3120 fel
# 3121 fortf fel, omskrivet 07.23
# 15697 fel, 07.36

if __name__ == "__main__":
    data = sample
#     data = """nfw
# nwf
# wnf
# pnfwq
# wnfe"""
#     data = open('input.txt').read()
    print(data)
    print()
    print("part one", getAnswer_1(data))
    print()
    r2 = getAnswer_2(data)
    print("part two", r2)
    # if r2 != 3121:
    #     print("BOOOM!")
