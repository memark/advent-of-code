# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import time
from functools import lru_cache
from typing import *


def isSum(v: int, pl: list[int]) -> bool:
    return anyTrue(
        v == i + j
        for i in pl
        for j in pl)


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


sample_1a = """
16
10
15
5
1
11
7
19
6
12
4
"""


sample_1b = """
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


class Answer1(NamedTuple):
    val: int
    index: int


def getAnswer_1(data: str) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    ns.insert(0, 0)
    b = max(ns) + 3
    print(b)
    ns.insert(len(ns), b)
    ns.sort()
    print(ns)

    j1 = 0
    j3 = 0

    for i in range(len(ns) - 1):
        d = ns[i + 1] - ns[i]
        if d == 1:
            j1 += 1
        if d == 3:
            j3 += 1

    print(j1, j3)

    return j1 * j3


def getAnswer_2a(data: str) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    ns.insert(0, 0)
    b = max(ns) + 3
    print(b)
    ns.insert(len(ns), b)
    ns.sort()
    print(ns)

    def a(l) -> int:
        ll = len(l)
        c = 0
        if ll == 1:
            c += 1
        if ll >= 2 and l[1] - l[0] <= 3:
            c += a(l[1:])
        if ll >= 3 and l[2] - l[0] <= 3:
            c += a(l[2:])
        if ll >= 4 and l[3] - l[0] <= 3:
            c += a(l[3:])
        printt(len(ns), ll, l, c)
        return c

    return a(ns)


def getAnswer_2a2(data: str) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    ns.insert(0, 0)
    b = max(ns) + 3
    print(b)
    ns.insert(len(ns), b)
    ns.sort()
    print(ns)

    def a(l) -> int:
        ll = len(l)
        c = 0
        if ll == 1:
            c += 1
        if ll >= 2 and l[1] - l[0] <= 3:
            c += a(l[1:])
        if ll >= 3 and l[2] - l[0] <= 3:
            c += a(l[2:])
        if ll >= 4 and l[3] - l[0] <= 3:
            c += a(l[3:])
        # printt(len(ns), ll, l, c)
        return c

    # return a(ns)

    # idé:
    # recursa fram tills vi springer på ett 3-hopp.
    # räkna ihop möjligheterna dit fram.
    # gångra med tidigare.
    # gå vidare.

    def a2(i) -> int:
        l = ns[i:]
        ll = len(l)
        c = 0
        if ll == 1:
            c += 1
        if ll >= 2 and l[1] - l[0] <= 3:
            c += a(l[1:])
        if ll >= 3 and l[2] - l[0] <= 3:
            c += a(l[2:])
        if ll >= 4 and l[3] - l[0] <= 3:
            c += a(l[3:])
        # printt(len(ns), ll, l, c)
        return c

    return a2(0)


def getAnswer_2b(data: str) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    ns.insert(0, 0)
    nl = len(ns)
    ns.insert(nl, max(ns) + 3)
    ns.sort()
    print("ns", ns)

    print("nl", nl)

    c = 1
    # for i in range(nl - 1):
    #     c1 = 0
    #     if i + 3 < nl and ns[i+3] - ns[i] <= 3:
    #         c1 = 4
    #     elif i + 2 < nl and ns[i+2] - ns[i] <= 3:
    #         c1 = 2
    #     elif i + 1 < nl and ns[i+1] - ns[i] <= 3:
    #         c1 = 1
    #     c *= c1
    #     printt(i, ns[i], c1, c)

    i = 0
    while i < nl - 1:
        c1 = 0
        if i + 4 < nl and ns[i+4] - ns[i] <= 3:
            c1 = 7
            i += 4
        elif i + 3 < nl and ns[i+3] - ns[i] <= 3:
            c1 = 4
            i += 3
        elif i + 2 < nl and ns[i+2] - ns[i] <= 3:
            c1 = 2
            i += 2
        elif i + 1 < nl and ns[i+1] - ns[i] <= 3:
            c1 = 1
            i += 1
        c *= c1

    return c


def getAnswer_2c(data: str) -> int:
    ns = [int(d) for d in data.strip().split("\n")]
    ns.insert(0, 0)
    nl = len(ns)
    ns.insert(nl, max(ns) + 3)
    ns.sort()
    print("ns", ns)

    print("nl", nl)

    c = 1
    # for i in range(nl - 1):
    #     c1 = 0
    #     if i + 3 < nl and ns[i+3] - ns[i] <= 3:
    #         c1 = 4
    #     elif i + 2 < nl and ns[i+2] - ns[i] <= 3:
    #         c1 = 2
    #     elif i + 1 < nl and ns[i+1] - ns[i] <= 3:
    #         c1 = 1
    #     c *= c1
    #     printt(i, ns[i], c1, c)

    i = 0
    while i < nl - 1:
        c1 = 0
        if i + 4 < nl and ns[i+4] - ns[i] <= 3:
            c1 = 7
            i += 4
        elif i + 3 < nl and ns[i+3] - ns[i] <= 3:
            c1 = 4
            i += 3
        elif i + 2 < nl and ns[i+2] - ns[i] <= 3:
            c1 = 2
            i += 2
        elif i + 1 < nl and ns[i+1] - ns[i] <= 3:
            c1 = 1
            i += 1
        c *= c1

    return c


def magic(nums: list[int]) -> int:
    @lru_cache
    def a(index: int) -> int:
        n, *ns = nums[index:]
        lns = len(ns)

        acc = 0
        if lns == 0:
            acc += 1
        if lns >= 1 and ns[0] - n <= 3:
            acc += a(index+1)
        if lns >= 2 and ns[1] - n <= 3:
            acc += a(index+2)
        if lns >= 3 and ns[2] - n <= 3:
            acc += a(index+3)

        return acc
    return a(0)


def parseData(data: str) -> list[int]:
    ns = [int(d) for d in data.strip().split("\n")]
    return sorted([0, *ns, max(ns) + 3])


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    # printt("part one (sample)", getAnswer_1(sample_1a))
    # printt("part one (sample)", getAnswer_1(sample_1b))
    # printt("part one (input)", getAnswer_1(data_1))

    # printt("part two (sample)", getAnswer_2a(sample_1b))
    # printt("part two (sample)", getAnswer_2a2(sample_1b))
    # printt("part two (sample)", getAnswer_2b(sample_1b))
    # 274877906944, too low
    # 302231454903657293676544, too high
    # printt("part two (input)", getAnswer_2b(data))

    tic = time.perf_counter()
    printt("part two (input)", magic(parseData(data)))
    toc = time.perf_counter()
    print(f"Ran for {toc - tic} seconds")
