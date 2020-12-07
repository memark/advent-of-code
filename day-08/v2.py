# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import re
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


class Instr(NamedTuple):
    op: str
    val: int


Sheet = list[Instr]


def getAnswer_1(data: str) -> int:
    matches = re.findall(r"(\w+) ([+-]\d+)", data)
    rows = [Instr(row[0], int(row[1])) for row in matches]

    return run_1(rows)


def run_1(sheet: Sheet) -> int:
    acc = 0
    visited = set()
    p = 0

    while p not in visited:
        row = sheet[p]
        visited.add(p)
        if row.op == "acc":
            acc += row.val
        if row.op == "jmp":
            p += row.val
            continue
        if row.op == "nop":
            pass
        p = p + 1

    return acc


def getAnswer_2(data: str) -> Optional[int]:
    matches = re.findall(r"(\w+) ([+-]\d+)", data)
    rows = [Instr(row[0], int(row[1])) for row in matches]

    runs = (run_2(flipInSheet(rows, i))
            for i in range(len(rows) - 1)
            if flippable(rows[i]))

    return firstNonNull(runs)


def run_2(sheet: Sheet) -> Optional[int]:
    acc = 0
    visited = set()
    p = 0

    while p <= len(sheet) - 1:
        if p in visited:
            return None
        visited.add(p)
        row = sheet[p]
        if row.op == "acc":
            acc += row.val
        if row.op == "jmp":
            p += row.val
            continue
        if row.op == "nop":
            pass
        p = p + 1

    return acc


def flipInSheet(sheet: Sheet, index: int) -> Sheet:
    res = sheet.copy()
    res[index] = flip(res[index])
    return res


def flip(instr: Instr) -> Instr:
    if instr.op == "jmp":
        return Instr("nop", instr.val)
    if instr.op == "nop":
        return Instr("jmp", instr.val)
    return instr


def flippable(instr: Instr) -> bool:
    return instr.op in ["jmp", "nop"]


T = TypeVar('T')


def firstNonNull(g: Generator[T, None, None]) -> Optional[T]:
    return next(filter(bool, g))


if __name__ == "__main__":
    data_1 = sample_1.strip()
    data_1 = open('input.txt').read().strip()
    print("part one", getAnswer_1(data_1))

    data_2 = sample_1.strip()
    data_2 = open('input.txt').read().strip()
    print("part two", getAnswer_2(data_2))
