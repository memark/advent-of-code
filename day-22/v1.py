# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

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
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
""".strip()

sample_2 = """
Player 1:
43
19

Player 2:
2
29
14
""".strip()


input = open('input.txt').read().strip()


def ignore_unhashable(func):
    uncached = func.__wrapped__
    attributes = functools.WRAPPER_ASSIGNMENTS + ('cache_info', 'cache_clear')

    @functools.wraps(func, assigned=attributes)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except TypeError as error:
            if 'unhashable type' in str(error):
                return uncached(*args, **kwargs)
            raise
    wrapper.__uncached__ = uncached
    return wrapper


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


hgn = 0


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    [p1, p2] = [parsePlayer(d) for d in data.strip().split('\n\n')]
    # print(f'{p1=}')
    # print(f'{p2=}')

    def get_key(d1: list[int], d2: list[int]) -> str:
        return '-'.join(map(str, d1)) + '+' + '-'.join(map(str, d2))

    # @ignore_unhashable
    # @cache
    def recurse(cards1: list[int], cards2: list[int]) -> Tuple[int, list[int]]:
        global hgn
        gn = hgn+1
        hgn = gn

        played_rounds: list[str] = list()

        # print()
        #print(f'=== Game {gn} ===')
        rn = 1
        while True:
            # print()
            #print(f'-- Round {rn} (Game {gn}) --')
            #print(f'Player 1\'s deck: {cards1}')
            #print(f'Player 2\'s deck: {cards2}')

            # if there was a previous round in this game
            # that had exactly the same cards in the same order in the same players' decks,
            # the game instantly ends in a win for player 1.
            key = get_key(cards1, cards2)
            ## print(f'Checking {key=} against {played_rounds=}')
            if key in played_rounds:
                #print(f'Using special rule => Player 1 is the winner')
                # print(f'{played_rounds=}')
                # time.sleep(1)
                return (1, cards1)

            played_rounds.append(key)

            c1 = cards1.pop(0)
            c2 = cards2.pop(0)

            #print(f'Player 1 plays: {c1}')
            #print(f'Player 2 plays: {c2}')

            # if gn > 1:
            # time.sleep(1)

            p1_wins = True

            # If both players have at least as many cards remaining in their deck as the value of the card they just drew,
            # the winner of the round is determined by playing a new game of Recursive Combat
            if len(cards1) >= c1 and len(cards2) >= c2:
                #print(f'Playing a sub-game to determine the winner...')
                # time.sleep(1)
                # To play a sub-game of Recursive Combat,
                # each player creates a new deck by making a copy of the next cards in their deck
                # (the quantity of cards copied is equal to the number on the card they drew to trigger the sub-game).
                sub_res = recurse(cards1[:c1], cards2[:c2])
                p1_wins = sub_res[0] == 1
                #print(f'...anyway, back to game {gn}.')
            else:
                if c1 == c2:
                    raise error("Same value")
                p1_wins = c1 > c2

            if p1_wins:
                #print(f'Player 1 wins round {rn} of game {gn}!')
                cards1.append(c1)
                cards1.append(c2)
            else:
                #print(f'Player 2 wins round {rn} of game {gn}!')
                cards2.append(c2)
                cards2.append(c1)

            if len(cards1) == 0:
                #print(f'The winner of game {gn} is player 1!')
                # print()
                # time.sleep(1)
                return (2, cards2)

            if len(cards2) == 0:
                #print(f'The winner of game {gn} is player 1!')
                # print()
                # time.sleep(1)
                return (1, cards1)

            rn += 1

    # print(f'{p1=}')
    # print(f'{p2=}')

    result = recurse(p1, p2)
    # print(f'{result=}')
    # First value is winner
    winner_cards: list[int] = result[1]
    # print(f'{winner_cards=}')

    factors = list(reversed(range(1, len(winner_cards)+1)))
    # print(factors)

    z = list(zip(winner_cards, factors))
    # print(z)
    score = 0
    for z1 in z:
        score += z1[0] * z1[1]
    # scores = map(operator.mul, zip(winner, factors))
    return score


def parsePlayer(s: str):
    [name, *cards] = s.splitlines()
    return [int(c) for c in cards]


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 306)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 30138)

    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1, 291)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_2)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 31587)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
