# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from typing import *


sample_1 = """
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


class Answer1(NamedTuple):
    val: int
    index: int


def getAnswer_1(data: str, pl: int) -> Answer1:
    ns = [int(d) for d in data.strip().split("\n")]
    return next(Answer1(v, i)
                for i in range(pl, len(ns))
                if not isSum(v := ns[i], ns[i-pl:i]))


def isSum(v: int, pl: list[int]) -> bool:
    return anyTrue(
        v == i + j
        for i in pl
        for j in pl)


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


def getAnswer_2(data: str, pl: int) -> int:
    (goal, lastIndex) = getAnswer_1(data, pl)
    ns = [int(d) for d in data.strip().split("\n")][:lastIndex]
    return next(min(contig) + max(contig)
                for i in range(len(ns))
                for j in range(i, len(ns))
                if sum(contig := ns[i:j]) == goal)


if __name__ == "__main__":
    data_1 = open('input.txt').read().strip()
    printt("part one (sample)", getAnswer_1(sample_1, 5).val)
    printt("part one (input)", getAnswer_1(data_1, 25).val)

    data_2 = open('input.txt').read().strip()
    printt("part two (sample)", getAnswer_2(sample_1, 5))
    printt("part two (input)", getAnswer_2(data_1, 25))
