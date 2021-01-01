# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

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


def parseData(data: str) -> list[str]:
    return data.strip().split("\n")


def getAnswer_1(data: str) -> int:
    rules = parseData(data)
    parsedRules = [parseRule(rule) for rule in rules]
    parsedRulesDict = {pr[0]: pr[
        1] for pr in parsedRules}
    print()
    # print(parsedRulesDict)
    # print()

    target = "shiny gold"

    def a() -> int:
        res = [b(parsedRulesDict[key]) for key in parsedRulesDict]
        return res.count(True)

    def b(slaves: list[tuple[int, str]]) -> bool:
        # print(slaves)
        if target in [slave[1] for slave in slaves]:
            return True
        if not slaves:
            return False
        res = [c(slave[1]) for slave in slaves]
        return any(res)

    def c(master: str) -> bool:
        # print(master)
        slaves = parsedRulesDict[master]
        res = b(slaves)
        return res

    return a()


# "drab bronze bags contain 4 bright red bags."
def parseRule(rule: str) -> tuple[str, list[tuple[int, str]]]:
    (masterString, slavesString) = rule.split("contain")
    master = parseMaster(masterString)
    slaves = parseSlaves(slavesString)
    # print(rule)

    result = (master, slaves)
    # printt(result)

    return result


def parseMaster(masterString: str) -> str:
    return masterString.replace("bags", "").strip()


def parseSlaves(slavesString: str) -> list[tuple[int, str]]:
    if "no other" in slavesString:
        return []
    slaves = [parseSlave(s)
              for s in slavesString.split(",")
              if parseSlave(s)]
    return slaves


def parseSlave(slaveString: str) -> tuple[int, str]:
    x = slaveString.replace("bags", "").replace("bag", "").strip(".").strip()
    n = int(x.split(" ")[0])
    m = " ".join(x.split(" ")[1:])
    return (n, m)


def getAnswer_2(data: str) -> int:
    rules = parseData(data)
    parsedRules = [parseRule(rule) for rule in rules]
    parsedRulesDict = {pr[0]: pr[
        1] for pr in parsedRules}
    print()
    # print(parsedRulesDict)
    # print()

    def a() -> int:
        target = "shiny gold"
        res = c(target) - 1     # remove one for top bag
        # printt("a", target, res)
        return res

    def b(slaves: list[tuple[int, str]]) -> int:
        res1 = [slave[0] * c(slave[1]) for slave in slaves]
        res2 = sum(res1)
        # printt("b", slaves, res1, res2)
        return res2

    def c(master: str) -> int:
        slaves = parsedRulesDict[master]
        res1 = b(slaves)
        res2 = res1 + 1     # add one for current bag
        # printt("c", slaves, master, res1, res2)
        return res2

    return a()


def printt(*args):
    print(" => ".join((str(a) for a in args)))


if __name__ == "__main__":
    data_1 = sample_1.strip()
    data_1 = open('input.txt').read().strip()
    print("part one", getAnswer_1(data_1))

    data_2 = sample_22.strip()
    data_2 = open('input.txt').read().strip()
    print("part two", getAnswer_2(data_2))
