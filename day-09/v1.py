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


class Instr(NamedTuple):
    op: str
    val: int


Sheet = list[Instr]


def getAnswer_1(data: str, pl: int) -> Tuple[int, int]:
    ns = [int(d) for d in data.strip().split("\n")]

    a = 0
    for i in range(pl, len(ns)):
        v = ns[i]
        p = ns[i-pl:i]
        if not isSum(v, p):
            return (v, i)

    return (0, 0)


def isSum(v: int, pl: list[int]) -> bool:
    for i in pl:
        for j in pl:
            if v == i + j:
                return True
    return False


def getAnswer_2(data: str, pl: int) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    (aa, ii) = getAnswer_1(data, pl)

    for i in range(ii):
        for j in range(i, ii):
            if aa == sum(ns[i:j]):
                return min(ns[i:j]) + max(ns[i:j])

    return 0


T = TypeVar('T')


def firstNonNull(g: Generator[T, None, None]) -> Optional[T]:
    return next(filter(bool, g))


if __name__ == "__main__":
    data_1 = open('input.txt').read().strip()
    printt("part one (sample)", getAnswer_1(sample_1, 5)[0])
    printt("part one (input)", getAnswer_1(data_1, 25)[0])

    data_2 = open('input.txt').read().strip()
    printt("part two (sample)", getAnswer_2(sample_1, 5))
    printt("part two (input)", getAnswer_2(data_1, 25))
