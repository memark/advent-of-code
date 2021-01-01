# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

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


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


class GameResult(NamedTuple):
    winner: int
    winner_deck: list[int]


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    [p1, p2] = [parsePlayer(d) for d in data.strip().split('\n\n')]

    last_game_num = 0

    def recurse(starting_cards_1: list[int], starting_cards_2: list[int]) -> GameResult:
        nonlocal last_game_num
        game_num = last_game_num + 1
        last_game_num = game_num

        played_rounds: set[str] = set()
        cards1 = starting_cards_1[:]
        cards2 = starting_cards_2[:]

        # print()
        # print(f'=== Game {gn} ===')
        round_num = 1
        while True:
            # print()
            # print(f'-- Round {rn} (Game {gn}) --')
            # print(f'Player 1\'s deck: {cards1}')
            # print(f'Player 2\'s deck: {cards2}')

            if isPartTwo:
                key = get_key(cards1, cards2)
                if key in played_rounds:
                    return GameResult(1, cards1)
                played_rounds.add(key)

            c1 = cards1.pop(0)
            c2 = cards2.pop(0)

            #print(f'Player 1 plays: {c1}')
            #print(f'Player 2 plays: {c2}')

            winner = 0

            if isPartTwo and len(cards1) >= c1 and len(cards2) >= c2:
                # print(f'Playing a sub-game to determine the winner...')
                winner = recurse(cards1[:c1], cards2[:c2]).winner
                # print(f'...anyway, back to game {gn}.')
            else:
                winner = 1 if c1 > c2 else 2

            if winner == 1:
                # print(f'Player 1 wins round {rn} of game {gn}!')
                cards1.append(c1)
                cards1.append(c2)
            else:
                # print(f'Player 2 wins round {rn} of game {gn}!')
                cards2.append(c2)
                cards2.append(c1)

            if len(cards1) == 0:
                # print(f'The winner of game {gn} is player 1!')
                # print()
                return GameResult(2, cards2)

            if len(cards2) == 0:
                # print(f'The winner of game {gn} is player 1!')
                # print()
                return GameResult(1, cards1)

            round_num += 1

    winner_cards = recurse(p1, p2).winner_deck
    # print(f'{winner_cards=}')

    score = sum(map(operator.mul,
                    winner_cards,
                    range(len(winner_cards), 0, -1)))

    return score


def parsePlayer(s: str):
    [name, *cards] = s.splitlines()
    return [int(c) for c in cards]


def get_key(d1: list[int], d2: list[int]):
    # The set of cards is constant, so it's enough to check one deck.
    return '-'.join(map(str, d1))


def main() -> None:
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1, 306)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 30138)

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
