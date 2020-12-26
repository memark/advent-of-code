# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from bisect import *
import cProfile
from collections import *
import functools
import operator
import itertools
import math
from re import *
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
5764801
17807724
""".strip()

sample_2 = """
""".strip()

sample_3 = """
""".strip()


sample_4 = """
""".strip()


input = open('input.txt').read().strip()


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    [pk1, pk2] = [int(l) for l in data.splitlines()]
    lz2 = get_loop_size(7, pk2)
    ek1 = get_enc_key(pk1, lz2)

    return ek1


def get_loop_size(sn: int, pk: int):
    v = 1
    loop = 0
    while v != pk:
        loop += 1
        v *= sn
        v %= 20201227
    return loop


def get_enc_key(pk: int, lz: int):
    v = 1
    for _ in range(lz):
        v *= pk
        v %= 20201227
    return v


def main() -> None:
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 14897079)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 10187657)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_3)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_4)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 3672)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
