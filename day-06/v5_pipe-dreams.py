# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from typing import *
from pipetools import *


def parseData(data: str) -> list[list[str]]:
    return data.strip().split("\n\n") > foreach(X.split("\n"))


def getAnswer_1(data: str) -> int:
    return parseData(data) > foreach(
        pipe | flatten | set | len
    ) | sum


def getAnswer_2(data: str) -> int:
    return parseData(data) > foreach(
        lambda group: group > pipe
        | allAnswers | set
        | (filter, lambda a: allAnswers(group).count(a) == len(group))
        | list | len
    ) | sum


def allAnswers(group): return ''.join(group)


if __name__ == "__main__":
    data = open('input.txt').read()
    print("part one", getAnswer_1(data))
    print("part two", getAnswer_2(data))
