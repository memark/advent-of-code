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
    return next((v, i)
                for i in range(pl, len(ns))
                if not (
                    any(filter(bool, (
                        (v := ns[i]) == j + k
                        for j in ns[i-pl:i]
                        for k in ns[i-pl:i])))
    ))


def getAnswer_2(data: str, pl: int) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    (aa, ii) = getAnswer_1(data, pl)
    return next(min(contig) + max(contig)
                for i in range(ii)
                for j in range(i, ii)
                if aa == sum(contig := ns[i:j]))


T = TypeVar('T')


def firstNonNull(g: Generator[T, None, None]) -> Optional[T]:
    return next(filter(bool, g))


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


if __name__ == "__main__":
    data_1 = open('input.txt').read().strip()
    printt("part one (sample)", getAnswer_1(sample_1, 5)[0])
    printt("part one (input)", getAnswer_1(data_1, 25)[0])

    data_2 = open('input.txt').read().strip()
    printt("part two (sample)", getAnswer_2(sample_1, 5))
    printt("part two (input)", getAnswer_2(data_1, 25))
