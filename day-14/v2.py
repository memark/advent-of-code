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
    addr: 'DecAddr'
    val: int


BinAddr = str
DecAddr = int


bl = 36


def parseRow(row: str) -> Union[Mask, Mem]:
    if "mask" in row:
        m = re.match(r'mask = (?P<mask>[X01]{' + f'{bl}' + '})', row)
        m1 = Mask(m.group('mask'))
        return m1
    if "mem" in row:
        m = re.match(r'mem\[(?P<addr>\d+)\] = (?P<val>\d+)', row)
        m1 = Mem(int(m.group('addr')), int(m.group('val')))
        return m1
    raise ValueError


def getAnswer_1(data: str) -> int:
    rows = map(parseRow, data.split('\n'))

    addrs = {}
    mask = ""
    for r in rows:
        if isinstance(r, Mask):
            mask = r.val
        if isinstance(r, Mem):
            def getValBit(mask_i: str, rvb_i: str) -> str:
                if mask_i == 'X':
                    return rvb_i
                return mask_i

            rvb = format(r.val, f'0{bl}b')
            rvb2 = [getValBit(mask[i], rvb[i]) for i in range(bl)]
            rvb2s = "".join(rvb2)
            rv2 = int(rvb2s, 2)

            addrs[r.addr] = rv2

    res = sum(addrs.values())

    return res


def getAnswer_2(data: str) -> int:
    rows = map(parseRow, data.split('\n'))

    memory = {}
    currentMask = ""
    for r in rows:
        if isinstance(r, Mask):
            currentMask = r.val
        if isinstance(r, Mem):
            rab = "".join(
                map(getAddrBit,
                    zip(currentMask,
                        format(r.addr, f'0{bl}b'))))
            memory |= {intFromBin(c): r.val for c in getVolCombos(rab)}

    return sum(memory.values())


def intFromBin(x: str):
    return int(x, 2)


Bit = str


def getAddrBit(maskAndAddr: Tuple[Bit, Bit]) -> str:
    if maskAndAddr[0] == '0':
        return maskAndAddr[1]
    if maskAndAddr[0] == '1':
        return '1'
    return 'X'


def getVolCombos(volAddr: BinAddr) -> Sequence[BinAddr]:
    bit = volAddr[0]
    bits = volAddr[1:]
    if len(bits) == 0:
        if bit == 'X':
            return ['0', '1']
        else:
            return [bit]
    else:
        if bit == 'X':
            return ['0' + bb for bb in getVolCombos(bits)] + ['1' + bb for bb in getVolCombos(bits)]
        else:
            return [bit + bb for bb in getVolCombos(bits)]


def getBitByShift(val: int, i: int) -> int:
    return val >> i & 1


def getBitByIndex(val: int, i: int) -> int:
    b = format(val, f"0{bl}b")[::-1]
    return int(b[i], 2)


for i in range(36):
    assert getBitByShift(123, i) == getBitByIndex(123, i)


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    tic = time.perf_counter()
    # answer_1 = getAnswer_1(sample_1)
    # answer_1 = getAnswer_1(sample_2)
    answer_1 = getAnswer_1(data)
    toc = time.perf_counter()

    printt(f'{answer_1=}')
    print(f"{(toc-tic)*1000:0.0f} ms")
    assert answer_1 == 13105044880745

    tic = time.perf_counter()
    # answer_2 = getAnswer_2(sample_1)
    # answer_2 = getAnswer_2(sample_2)
    answer_2 = getAnswer_2(data)
    toc = time.perf_counter()

    printt(f'{answer_2=}')
    print(f"{(toc-tic)*1000:3.0f} ms")
    assert answer_2 == 3505392154485
