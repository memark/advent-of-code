# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import re
from functools import *
import time
from typing import *


def anyTrue(g: Generator[bool, None, None]) -> bool:
    return any(filter(bool, g))


sample_1 = """
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"""

sample_2 = """
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


class Mask(NamedTuple):
    val: str


class Mem(NamedTuple):
    addr: int
    val: int


def getAnswer_1(data: str) -> int:
    rows = []
    for r in data.split('\n'):
        if "mask" in r:
            m = re.match(r'mask = ([X01]{36})', r)
            m1 = Mask('0b' + m.group(1))
            rows.append(m1)
        if "mem" in r:
            m = re.match(r'mem\[(\d+)\] = (\d+)', r)
            m1 = Mem(int(m.group(1)), int(m.group(2)))
            rows.append(m1)

    rmax = max(r.addr for r in rows if isinstance(r, Mem))
    addrs = [0] * (rmax+1)
    mask = ""
    for r in rows:
        if isinstance(r, Mask):
            mask = r.val
        if isinstance(r, Mem):
            rvb = format(r.val, '#038b')

            rvb2 = '0b'
            for i in range(2, 38):
                if mask[i] == 'X':
                    rvb2 += rvb[i]
                else:
                    rvb2 += mask[i]
            rv2 = int(rvb2, 2)
            addrs[r.addr] = rv2

    res = sum(addrs)

    return res


def getAnswer_2(data: str) -> int:
    rows = []
    for r in data.split('\n'):
        if "mask" in r:
            m = re.match(r'mask = ([X01]{36})', r)
            m1 = Mask('0b' + m.group(1))
            rows.append(m1)
        if "mem" in r:
            m = re.match(r'mem\[(\d+)\] = (\d+)', r)
            m1 = Mem(int(m.group(1)), int(m.group(2)))
            rows.append(m1)

    addrs = {}
    mask = ""
    for r in rows:
        if isinstance(r, Mask):
            mask = r.val
        if isinstance(r, Mem):
            rab = format(r.addr, '#038b')
            rab2 = '0b'
            for i in range(2, 38):
                if mask[i] == '0':
                    rab2 += rab[i]
                if mask[i] == '1':
                    rab2 += '1'
                if mask[i] == 'X':
                    rab2 += 'X'
            a = getVolCombs(rab2)
            for aa in a:
                aad = int(aa, 2)
                addrs[aad] = r.val

    res = sum([addrs[key] for key in addrs])

    return res


Addr = str


def getVolCombs(volAddr: Addr) -> Sequence[Addr]:
    bit = volAddr[0]
    bits = volAddr[1:]
    if len(bits) == 0:
        if bit == 'X':
            return ['0', '1']
        else:
            return [bit]
    else:
        if bit == 'X':
            return ['0' + bb for bb in getVolCombs(bits)] + ['1' + bb for bb in getVolCombs(bits)]
        else:
            return [bit + bb for bb in getVolCombs(bits)]


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    tic = time.perf_counter()
    # printt("part one (sample 1)", getAnswer_1(sample_1))
    # printt("part one (sample 2)", getAnswer_1(sample_2))
    printt("part one (input)", getAnswer_1(data))
    toc = time.perf_counter()
    print(f"{(toc-tic)*1000:0.0f} ms")

    tic = time.perf_counter()
    # printt("part two (sample 1)", getAnswer_2(sample_1))
    # printt("part two (sample 2)", getAnswer_2(sample_2))
    printt("part two (input)", getAnswer_2(data))
    toc = time.perf_counter()
    print(f"{(toc-tic)*1000:3.0f} ms")
