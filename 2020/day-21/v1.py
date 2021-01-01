# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

import itertools
import math
from re import *
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"""

sample_2 = """
i1 i2 (contains a1)
i3 (contains a2)
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    foods = [parseLine(d) for d in data.strip().splitlines()]
    print(f'{len(foods)=}')

    allIngredients = list(set([i for f in foods for i in f.ingredients]))
    allAllergens = list(set([a for f in foods for a in f.allergens]))
    print(f'{len(allIngredients)=}')
    print(f'{len(allAllergens)=}')

    ing2all = {i: f.allergens
               for i in allIngredients
               for f in foods
               if i in f.ingredients
               }
    # print(f'{ing2all=}')

    all2ing = {a: f.ingredients
               for a in allAllergens
               for f in foods
               if a in f.allergens
               }
    print(f'{all2ing=}')
    # sorted(all2ing.items())
    all2ingSorted = dict(sorted(
        all2ing.items(), key=lambda item: len(item[1])))
    print(f'{all2ingSorted=}')

    def rec(d: dict):
        for kv in d.items():
            a = kv[0]
            for i in kv[1]:
                print(f'assuming {a} is {i}')

    rec(all2ingSorted.copy())

    return 42


class Food(NamedTuple):
    ingredients: list[str]
    allergens: list[str]


def parseLine(s: str):
    m = re.search(r'^(.*) \(contains (.*)\)$', s)
    ingredients = m.group(1).split(' ')
    allergens = m.group(2).split(', ')
    return Food(ingredients, allergens)


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
