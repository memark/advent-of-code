# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import builtins
import re
from functools import *
import time
from typing import *


sample_1 = """
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
""".strip()

sample_2 = """
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
""".strip()


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: Sequence):
    print("\n".join(str(x1) for x1 in x))


class Range(NamedTuple):
    start: int
    end: int


class Rule(NamedTuple):
    field: str
    ranges: Sequence[Range]


def getAnswer_1(data: str) -> int:
    d = data.split('\n\n')
    # print(f"{d=}")

    rules: Sequence['Rule'] = [parseA(dd) for dd in d[0].split('\n')]
    # print(f"{rules=}")
    mt = d[1].replace('your ticket:\n', '')
    # print(f"{mt=}")
    nbts = [[int(ddd) for ddd in dd.split(',')] for dd in d[2].replace(
        'nearby tickets:\n', '').split('\n')]
    # print(f"{nbts=}")

    def isValidValue(value: int) -> bool:
        for rule in rules:
            for range in rule.ranges:
                if range.start <= value <= range.end:
                    # print(f"{range.start <= value <= range.end=}")
                    return True
        return False

    tser = 0

    for nbt in nbts:
        for val in nbt:
            if not isValidValue(val):
                tser += val
    return tser


def parseA(s: str) -> Rule:
    m = re.match(
        r'(?P<field>[\w ]+): (?P<start1>\d+)-(?P<end1>\d+) or (?P<start2>\d+)-(?P<end2>\d+)', s)
    return Rule(m.group('field'), [
        Range(int(m.group('start1')), int(m.group('end1'))),
        Range(int(m.group('start2')), int(m.group('end2')))
    ])


def getAnswer_2(data: str) -> int:
    d = data.split('\n\n')

    rules: Sequence['Rule'] = [parseA(dd) for dd in d[0].split('\n')]
    # print(f"{rules=}")
    mt = [int(dd) for dd in d[1].replace('your ticket:\n', '').split(',')]
    # print(f"{mt=}")
    nbts = [[int(ddd) for ddd in dd.split(',')] for dd in d[2].replace(
        'nearby tickets:\n', '').split('\n')]
    # print(f"{len(nbts)=}")

    def isValidValue(value: int) -> bool:
        for rule in rules:
            for range in rule.ranges:
                if range.start <= value <= range.end:
                    return True
        return False

    def isValidTicket(ticket: list[int]):
        return all(map(isValidValue, ticket))

    validTickets = list(filter(isValidTicket, nbts))
    # print(f"{len(validTickets)=}")

    order: Dict[str, list[int]] = {}
    for rule in rules:
        for i in range(len(rules)):
            def isValidTicket(ticket: list[int]) -> bool:
                value = ticket[i]
                for range in rule.ranges:
                    if range.start <= value <= range.end:
                        return True
                return False

            ruleIsFieldI = all(map(isValidTicket, validTickets))
            if ruleIsFieldI:
                order[rule.field] = order[rule.field] if rule.field in order else list()
                order[rule.field].append(i)

    # printt2([(k, order[k]) for k in order])

    neworder: Dict[str, int] = {}
    usedindexes: Set[int] = set()
    for i in range(len(order)):
        c = i+1
        field = next(k for k in order if len(order[k]) == c)
        # print(f"{i=} {c=} {field=} {order[field]=} {len(order[field])=}")

        possibles = [i2 for i2 in order[field] if i2 not in usedindexes]
        assert len(possibles) == 1
        possible = possibles[0]
        # print(f"{usedindexes=} {possible=}")
        usedindexes.add(possible)
        neworder[field] = possible
    # print(f"{neworder=}")

    relorder = dict(
        filter(lambda elem: elem[0].startswith('departure'), neworder.items()))
    # print(f"{relorder=}")

    # print(f"{mt=}")

    res = 1
    for rf in relorder:
        index = relorder[rf]
        res *= mt[index]
        # print(f"{rf=} {index=} {mt[index]=} {res=}")

    return res


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    tic = time.perf_counter()
    # answer_1 = getAnswer_1(sample_1)
    # answer_1 = getAnswer_1(sample_2)
    answer_1 = getAnswer_1(data)
    toc = time.perf_counter()

    printt(f'{answer_1=}')
    print(f"{(toc-tic)*1000:.0f} ms")
    assert answer_1 == 25895

    tic = time.perf_counter()
    # answer_2 = getAnswer_2(sample_1)
    # answer_2 = getAnswer_2(sample_2)
    answer_2 = getAnswer_2(data)
    toc = time.perf_counter()

    printt(f'{answer_2=}')
    print(f"{(toc-tic)*1000:.0f} ms")
    assert answer_2 == 5865723727753
