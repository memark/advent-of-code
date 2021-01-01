# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from functional import seq
from pipetools import *
from typing import *


sample_1 = """
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""


sample_2 = """

"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def getAnswer_1(data: str) -> int:
    x = data.split("\n")
    y = [x1.split(" ") for x1 in x]

    a = 0
    v = set()
    i = 0

    while i not in v:
        y1 = y[i]
        v.add(i)
        if y1[0] == "acc":
            a = a + int(y1[1])
        if y1[0] == "jmp":
            i = i + int(y1[1])
            continue
        if y1[0] == "nop":
            pass
        i = i + 1

    return a


def getAnswer_2(data: str) -> Optional[int]:
    x = data.split("\n")
    y = [x1.split(" ") for x1 in x]

    z: list[list[list[str]]] = list()     # all new lists

    i = 0
    while i <= len(y) - 1:
        y1 = y[i]
        if y1[0] == "jmp" or y1[0] == "nop":
            z.append(createModifiedList(y, i))
        i = i + 1

    for z1 in z:
        res = evaluate(z1)
        if res:
            return res


def createModifiedList(y: list[list[str]], index: int) -> list[list[str]]:
    res: list[list[str]] = list()
    for i in range((len(y))):
        y1 = y[i]
        if i == index:
            if y1[0] == "jmp":
                res.append(["nop", y1[1]])
            if y1[0] == "nop":
                res.append(["jmp", y1[1]])
        else:
            res.append([y1[0], y1[1]])
    return res


def evaluate(y: list[list[str]]) -> Optional[int]:
    maxTries = 1000
    a = 0       # accumulator
    i = 0       # index
    n = 0       # tries

    while i <= len(y) - 1:
        n = n + 1
        if n > maxTries:
            return None
        y1 = y[i]
        if y1[0] == "acc":
            a = a + int(y1[1])
        if y1[0] == "jmp":
            i = i + int(y1[1])
            continue
        if y1[0] == "nop":
            pass
        i = i + 1

    return a


if __name__ == "__main__":
    data_1 = sample_1.strip()
    data_1 = open('input.txt').read().strip()
    print("part one", getAnswer_1(data_1))

    data_2 = sample_1.strip()
    data_2 = open('input.txt').read().strip()
    print("part two", getAnswer_2(data_2))
