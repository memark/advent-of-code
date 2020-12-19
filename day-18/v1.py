# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from re import *
import functools
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
1 + 2 * 3 + 4 * 5 + 6
""".strip()

sample_2 = """
1 + (2 * 3) + (4 * (5 + 6))
""".strip()

sample_3 = """
2 * 3 + (4 * 5)
""".strip()

sample_4 = """
5 + (8 * 3 + 9 + 3 * 4 * 3)
""".strip()

sample_5 = """
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
""".strip()

sample_6 = """
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return sum(int(
        evalexpr_1a(d.replace(' ', '')))
        for d in data.splitlines())


def getAnswer_2(data: str) -> int:
    return sum(int(
        evalexpr_2a(d.replace(' ', '')))
        for d in data.splitlines())


parens_pattern = re.compile(r'\(([^\(\)]*)\)')
add_mul_pattern = re.compile(r'\d+[\+\*]\d+')
add_pattern = re.compile(r'\d+[\+]\d+')
mul_pattern = re.compile(r'\d+[\*]\d+')


def evalexpr_1a(expr: str) -> str:
    last = expr
    while 1:
        current = evalexpr_1b(last)
        if current == last:
            return last
        last = current


def evalexpr_1b(expr: str) -> str:
    if m := parens_pattern.search(expr):
        return expr.replace(m.group(0), evalexpr_1a(m.group(1)), 1)
    if m := add_mul_pattern.search(expr):
        return expr.replace(m.group(0), str(eval(m.group(0))), 1)
    return expr


def evalexpr_2a(expr: str) -> str:
    last = expr
    while 1:
        current = evalexpr_2b(last)
        if current == last:
            return current
        last = current


def evalexpr_2b(s: str) -> str:
    if m := parens_pattern.search(s):
        return s.replace(m.group(0), evalexpr_2a(m.group(1)), 1)
    if m := add_pattern.search(s):
        return s.replace(m.group(0), str(eval(m.group(0))), 1)
    if m := mul_pattern.search(s):
        return s.replace(m.group(0), str(eval(m.group(0))), 1)
    return s


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 71)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2, 51)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3, 26)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_4, 437)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_5, 12240)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_6, 13632)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2, 51)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_3, 46)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_4, 1445)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_5, 669060)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_6, 23340)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 94240043727614)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
