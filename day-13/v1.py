# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import numpy as np
from functools import *
import time
from enum import Enum
import itertools
from typing import *


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


sample_1 = """
939
7,13,x,x,59,x,31,19
"""

sample_2 = """
1
1789,37,47,1889
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


class BidOff(NamedTuple):
    bid: int
    off: int


def getAnswer_1(data: str) -> int:
    dd = data.strip().split("\n")
    et = int(dd[0])
    bids = [int(x) for x in dd[1].split(",") if x != 'x']
    print(et, bids)

    for t in range(et, et*10):
        for bid in bids:
            if t % bid == 0:
                printt(t, bid, t-et, bid*(t-et))
                return bid*(t-et)

    return 42


def getAnswer_2(data: str) -> int:
    dd = data.strip().split("\n")
    bids = [x for x in dd[1].split(",")]
    print(bids)

    l: list[BidOff] = []
    for j in range(len(bids)):
        bid = bids[j]
        if bid != 'x':
            l.append(BidOff(int(bid), j))

    print(l)

    def tryT(t: int) -> bool:
        for l1 in l:
            if (t+l1.off) % l1.bid == 0:
                continue
            return False
        return True

        # while (i < len(bids)):
        #     bid = bids[i]
        #     # printt(t, i, bid)
        #     if bid == 'x':
        #         i += 1
        #         continue
        #     if (t+i) % int(bid) == 0:
        #         # printt(t, i, t+i, bid)
        #         i += 1
        #         continue
        #     i += 1
        #     return False
        # return True

    # t = 0  # 1068773 - 10
    # while (True):  # (t < 1068773 + 10):
    #     tt = tryT(t)
    #     if tt:
    #         return t
    #     t += 1

    # def tryBuses(bl: list[BidOff]):
    #     b, bs = bl

    # def a(bl: list[BidOff], t: int) -> bool:
    #     b, *bs = bl
    #     lbs = len(bs)

    #     print(f"{t=}, {b.bid=}, {b.off=}")

    #     if t % b.bid != b.off:
    #         print("no match for bus", b.bid)
    #         return False

    #     if lbs == 0:
    #         print(t)
    #         return True
    #     else:
    #         t2 = 0
    #         while (True):
    #             if a(bs, t + t2):
    #                 return True
    #             t2 += b.bid

    # return a(l, 0)

    def mult(a: BidOff, b: BidOff) -> BidOff:
        return BidOff(b.bid * a.bid, b.off * a.bid)

    def plus(a: BidOff, b: BidOff) -> BidOff:
        return BidOff(b.bid + a.bid, b.off + a.off)

    # print(f"{reduce(mult, l)=}")
    # print(f"{reduce(plus, l)=}")

    # for l1 in l:
    #     for l2 in l:

    # A = np.array([[4, 3, 2], [-2, 2, 3], [3, -5, 2]])
    # B = np.array([25, -10, -4])
    # X2 = np.linalg.solve(A, B)
    # print(X2)

    # l = l[0:2]
    # print(l)

    # a = [[l1.bid, 0, 0, 0, 0] for l1 in l]
    a = [
        [7, 0, 0, 0, 0],
        [0, 13, 0, 0, 0],
        [0, 0, 59, 0, 0],
        [0, 0, 0, 31, 0],
        [0, 0, 0, 0, 19],
    ]
    A = np.array(a)
    print(A)

    # b = [[l1.off] for l1 in l]
    b = [0, 1, 4, 6, 7]
    B = np.array(b)
    print(B)

    X2 = np.linalg.solve(A, B)
    print(X2)

    return 42


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    # printt("part one (sample)", getAnswer_1(sample_1))
    # printt("part one (input)", getAnswer_1(data))

    printt("part two (sample 1)", getAnswer_2(sample_1))
    # printt("part two (sample 2)", getAnswer_2(sample_2))
    # printt("part two (input)", getAnswer_2(data))
