# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from array import array
import re
from functools import *
import time
from typing import *


sample_1 = """
0,3,6
""".strip()

sample_2 = """
""".strip()


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def printt2(x: list):
    print("\n".join(str(x1) for x1 in x))


def getAnswer_1(data: str) -> int:
    # print(data)
    nums = [int(d) for d in data.split(",")]
    print(nums)

    goal = 2020
    # Key: Turn
    # Val: Number
    store = {}
    for turn in range(1, goal+1):
        if turn <= len(nums):
            new_num = nums[turn-1]  # 0-indexed
            store[turn] = new_num
            # print(f"{turn=}: {new_num=}")
        else:
            last_num = store[turn-1]
            used_before = [key for key in store if store[key] == last_num]
            if len(used_before) == 1:
                new_num = 0
                store[turn] = new_num
                # print(f"{turn=}: {last_num=} {new_num=}")
            else:
                prev_time = used_before[-1]
                prev_prev_time = used_before[-2]
                new_num = prev_time - prev_prev_time
                store[turn] = new_num
                # print(
                #     f"{turn=}: {last_num=} {prev_time=} {prev_prev_time=} {new_num=}")

    return store[goal]


def getAnswer_2(data: str) -> int:
    # print(data)
    nums = [int(d) for d in data.split(",")]
    print(nums)
    nl = len(nums)

    goal = 30000000  # 30000000
    # Key: Number
    # Val: list of turns
    store: dict[int, array[int]] = {}
    last_num = 0
    new_num = 0
    for turn in range(1, goal+1):
        if turn <= nl:
            new_num = nums[turn-1]  # 0-indexed
            store[new_num] = array('L')
            store[new_num].append(turn)
            last_num = new_num
            # print(f"{turn=}: {new_num=}")
        else:
            if len(store[last_num]) == 1:
                new_num = 0
                # store[new_num] = [*store[new_num], turn]
                # print(f"{turn=}: {last_num=} {new_num=}")
            else:
                prev_time = turn-1
                prev_prev_time = store[last_num][-2]
                new_num = prev_time - prev_prev_time
                # print(
                #     f"{turn=}: {last_num=} {prev_time=} {prev_prev_time=} {new_num=}")
            if new_num in store:
                store[new_num].pop(0)
            else:
                store[new_num] = array('L')
            store[new_num].append(turn)
            last_num = new_num

    return new_num


if __name__ == "__main__":
    data = open('input.txt').read().strip()

    tic = time.perf_counter()
    # answer_1 = getAnswer_1(sample_1)
    # answer_1 = getAnswer_1(sample_2)
    answer_1 = getAnswer_1(data)
    toc = time.perf_counter()

    printt(f'{answer_1=}')
    print(f"{(toc-tic)*1000:0.0f} ms\n")
    # assert answer_1 == 13105044880745

    tic = time.perf_counter()
    # answer_2 = getAnswer_2(sample_1)
    # answer_2 = getAnswer_2(sample_2)
    answer_2 = getAnswer_2(data)
    toc = time.perf_counter()

    printt(f'{answer_2=}')
    print(f"{(toc-tic)*1000:3.0f} ms")
    # assert answer_2 == 3505392154485
