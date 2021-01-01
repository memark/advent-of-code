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
389125467
""".strip()

sample_2 = """
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    def pretty_cups():
        n = 8
        new_cups = [*cups[:n], '  -  ', cups[-n:]] if isPartTwo else cups
        # Skriv om som list comprehension!
        s = list()
        for c in new_cups:
            if c == curr_label:
                s.append(f'({c})')
            else:
                s.append(f' {c} ')
        return ' '.join(s)

    # The cups will be arranged in a circle and labeled clockwise (your puzzle input).
    # For example, if your labeling were 32415, there would be five cups in the circle;
    # going clockwise around the circle from the first cup,
    # the cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.

    cups = [int(d) for d in data]
    if isPartTwo:
        cups.extend(range(max(cups)+1, 1000000+1))
    lowest_cup_label = min(cups)
    highest_cup_label = max(cups)
    print(lowest_cup_label, '-', highest_cup_label)

    # Before the crab starts, it will designate the first cup in your list as the current cup.
    curr_label = cups[0]

    # Part One: The crab is then going to do 100 moves.
    # Part Two: After discovering where you made the mistake in translating Crab Numbers,
    # you realize the small crab isn't going to do merely 100 moves;
    # the crab is going to do ten million (10000000) moves!
    num_moves = 10  # 100 if isPartTwo else 100

    L = len(cups)

    cache: dict[int, int] = dict()

    def cachedIndex(item: int) -> int:
        if item in cache and cups[cache[item]] == item:
            return cache[item]

        if item in cache:
            index = cache[item]
            for i in range(-4, 4+1):
                temp = cups[index+i]
                if temp == item:
                    cache[item] = temp
                    return temp

        res = cups.index(item)
        cache[item] = res
        return res

    # Each move, the crab does the following actions:
    for m in range(1, num_moves + 1):
        print(f'-- move {m} --')
        print(f'cups: {pretty_cups()}')

        # The crab picks up the three cups
        # that are immediately clockwise of the current cup.
        # They are removed from the circle;
        # cup spacing is adjusted as necessary to maintain the circle.

        picked_up = list()
        curr_index = cachedIndex(curr_label)
        for i in range(3):
            pop_index = (curr_index+1) % L
            picked_up.append(cups.pop(pop_index))
            L -= 1
            if pop_index < curr_index:
                curr_index -= 1
                # print('adjusted curr_index')

        # picked_up = [cups.pop((cups.index(curr_label)+1) % len(cups)),
        #              cups.pop((cups.index(curr_label)+1) % len(cups)),
        #              cups.pop((cups.index(curr_label)+1) % len(cups))]

        print(f'pick up: {picked_up}')
        print(f'cups: {pretty_cups()}')

        # The crab selects a destination cup:
        # the cup with a label equal to the current cup's label minus one.
        # If this would select one of the cups that was just picked up,
        # the crab will keep subtracting one until it finds a cup that wasn't just picked up.
        # If at any point in this process the value goes below the lowest value on any cup's label,
        # it wraps around to the highest value on any cup's label instead.
        dest_label = curr_label - 1
        # while dest_label not in cups:
        while dest_label in picked_up or dest_label == 0:
            dest_label -= 1
            if dest_label < lowest_cup_label:
                dest_label = highest_cup_label
        # dest_index = cups.index(dest_label)
        dest_index = cachedIndex(dest_label)
        # print(f'destination: {dest_label}')

        # The crab places the cups it just picked up
        # so that they are immediately clockwise of the destination cup.
        # They keep the same order as when they were picked up.
        # Att använda cups.insert() gav ingen skillnad i hastighet.
        cups[dest_index+1:dest_index+1] = picked_up
        L += 3
        # print(f'cups: {pretty_cups()}')

        # The crab selects a new current cup:
        # the cup which is immediately clockwise of the current cup.
        # new_index = (cups.index(curr_label)+1) % L
        new_index = (cachedIndex(curr_label)+1) % L
        curr_label = cups[new_index]

        # print()

    # print(f'-- final --')
    # print(f'cups: {pretty_cups()}')
    # print()

    def score():
        if not isPartTwo:
            index1 = cups.index(1)
            s = ''
            for i in range(index1+1, index1+len(cups)):
                s = s + str(cups[i % len(cups)])
            # print(s)
            return int(s)
        else:
            index1 = cups.index(1)
            label_star_1 = int(cups[(index1+1) % len(cups)])
            label_star_2 = int(cups[(index1+2) % len(cups)])
            # print(label_star_1)
            # print(label_star_2)
            return label_star_1 * label_star_2

    return score()


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(        getAnswer_1, sample_1, 92658374) # För 10
    # getAndPrintAndAssertAndTimeAnswer(
    #     getAnswer_1, sample_1, 67384529)  # För 100
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 64893572)  # För 1000

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1, 12)  # För 100
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1, 12)  # För 1000
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
