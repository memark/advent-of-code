# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from re import *
from os import error
import re
from functools import *
import time
from typing import *


sample_1 = """
0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aab
aba
""".strip()


sample_2 = """
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
""".strip()


sample_3 = """
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
""".strip()


input = open('input.txt').read().strip()


def getAnswer_1(data: str) -> int:
    return getAnswer(data, isPartTwo=False)


def getAnswer_2(data: str) -> int:
    return getAnswer(data, isPartTwo=True)


def getAnswer(data: str, *, isPartTwo: bool) -> int:
    rules = parseRules(data.split('\n\n')[0].splitlines(), isPartTwo=isPartTwo)
    messages = data.split('\n\n')[1].splitlines()

    # this divisor is trial-and-error for correctness and speed
    maxDepth = max(map(len, messages)) // 3
    pattern = createRegex(rules, maxDepth)

    return sum(map(bool, (map(pattern.match, messages))))


def parseRules(rules: list[str], *, isPartTwo: bool) -> dict[str, Any]:
    def parseRule(rule: str) -> Tuple[str, Any]:
        r = modifyRuleForPartTwo(rule) if isPartTwo else rule

        if m := re.match(r'^(\d+): "([ab])"$', r):
            return (m.group(1), m.group(2))

        if m := re.match(r'^(\d+): ([\d ]+)$', r):
            r2 = m.group(2).split(' ')
            return (m.group(1), r2)

        if m := re.match(r'^(\d+): ([\d ]+\|[\d ]+)$', r):
            r2 = [x.strip().split(' ') for x in m.group(2).split('|')]
            return (m.group(1), r2)

        raise error(f'unmatched {r=}')

    return {p[0]: p[1] for r in rules if (p := parseRule(r))}


def createRegex(rules: dict[str, Any], maxDepth: int) -> Pattern:
    def recurse(rule: Any, depth: int = 0) -> str:
        if depth > maxDepth:
            return ''

        # letters 'a', 'b'
        if isinstance(rule, str):
            return rule

        # list 'x y'
        if isinstance(rule, list) and not all(isinstance(elem, list) for elem in rule):
            return "".join(recurse(rules[r], depth+1) for r in rule)

        # pipe 'x | y'
        if isinstance(rule, list) and all(isinstance(elem, list) for elem in rule):
            groups = [f"(?:{recurse(r, depth+1)})" for r in rule]
            return f'(?:{"|".join(groups)})'

        raise error(f'unmatched {rule=}')

    return re.compile(f'^{recurse(rules["0"])}$')


def modifyRuleForPartTwo(rule: str) -> str:
    if rule == '8: 42':
        return '8: 42 | 42 8'
    if rule == '11: 42 31':
        return '11: 42 31 | 42 11 31'
    return rule


def main() -> None:
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_1)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_2)
    # getAndPrintAndAssertAndTimeAnswer(getAnswer_1, sample_3)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_1, input, 210)

    # getAndPrintAndAssertAndTimeAnswer(getAnswer_2, sample_1)
    getAndPrintAndAssertAndTimeAnswer(getAnswer_2, input, 422)


def getAndPrintAndAssertAndTimeAnswer(func: Callable[[str], int], data: str, expected: Optional[int] = None) -> None:
    tic = time.perf_counter()
    answer = func(data)
    toc = time.perf_counter()

    correction = f"should be {expected}" if expected and answer != expected else ""
    print(f'{(toc-tic)*1000:.0f} ms\t{func.__qualname__} => {answer}\t\t{correction}')


if __name__ == "__main__":
    main()
