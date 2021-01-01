# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from pipetools import *
import re
from typing import *


sample_1 = """
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""


sample_21 = """
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""


sample_21b = """
shiny gold bags contain 2 faded blue bag.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""


sample_22 = """
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"""


def printt(*args):
    print(" => ".join((str(a) for a in args)))


def parseRules(data: str):
    rules = data.strip().split("\n")
    parsedRules = [parseRule(rule) for rule in rules]
    return {pr[0]: pr[1] for pr in parsedRules}


# "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
def parseRule(rule: str) -> tuple[str, list[tuple[int, str]]]:
    (masterString, slavesString) = rule.split(" contain ")
    master = re.match(r"(\w+ \w+) bags", masterString)[1]
    slaves = [(int(r[0]), r[1])
              for r in re.findall(r"(\d+) (\w+ \w+) bag", slavesString)]
    return (master, slaves)


def getAnswer_1(data: str) -> int:
    parsedRulesDict = parseRules(data)
    target = "shiny gold"

    def a() -> int:
        return sum(b(parsedRulesDict[key]) for key in parsedRulesDict)

    def b(slaves: list[tuple[int, str]]) -> bool:
        if target in [slave[1] for slave in slaves]:
            return True
        if not slaves:
            return False
        return any(c(slave[1]) for slave in slaves)

    def c(master: str) -> bool:
        return b(parsedRulesDict[master])

    return a()


def getAnswer_2(data: str) -> int:
    parsedRulesDict = parseRules(data)
    target = "shiny gold"

    def a() -> int:
        return c(target) - 1     # remove one for top bag

    def b(slaves: list[tuple[int, str]]) -> int:
        return sum(slave[0] * c(slave[1]) for slave in slaves)

    def c(master: str) -> int:
        return b(parsedRulesDict[master]) + 1     # add one for current bag

    return a()


if __name__ == "__main__":
    data_1 = sample_1.strip()
    data_1 = open('input.txt').read().strip()
    print("part one", getAnswer_1(data_1))

    data_2 = sample_22.strip()
    data_2 = open('input.txt').read().strip()
    print("part two", getAnswer_2(data_2))
