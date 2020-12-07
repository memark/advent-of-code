# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from typing import *
from pipetools import *


sample = """
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


def parseData(data: str) -> list[str]:
    return data.strip().split("\n")


def getAnswer_1(data: str) -> int:
    rules = parseData(data)
    parsedRules = [parseRule(rule) for rule in rules]
    parsedRulesDict = {pr[0]: pr[
        1] for pr in parsedRules}
    print()
    print(parsedRulesDict)
    print()

    target = "shiny gold"

    def a() -> int:
        res = [b(parsedRulesDict[key]) for key in parsedRulesDict]
        return res.count(True)

    def b(slaves) -> bool:
        # print(slaves)
        if target in slaves:
            return True
        if not slaves:
            return False
        res = [c(slave) for slave in slaves]
        return any(res)

    def c(master) -> bool:
        # print(master)
        slaves = parsedRulesDict[master]
        res = b(slaves)
        return res

    return a()


def parseRule(rule: str) -> Tuple[Any, Any]:
    (master, slave) = rule.split("contain")
    master2 = master.replace("bags", "").strip()
    slaves = slave.split(",")
    slaves2 = [parseSlave(s)
               for s in slaves if parseSlave(s)]
    print(f"{master2} => {slaves2}")

    return (master2, slaves2)


def parseSlave(slave: str) -> Optional[str]:
    x = slave.replace("bags", "").replace("bag", "").strip(".").strip()
    if x == "no other":
        return None
    n = x.split(" ")[0]
    m = " ".join(x.split(" ")[1:])
    return m


def getAnswer_2(data: str) -> int:
    return parseData(data) > foreach(
        pipe
    )


if __name__ == "__main__":
    data = sample.strip()
    data = open('input.txt').read().strip()
    print("part one", getAnswer_1(data))
    # print("part two", getAnswer_2(data))
