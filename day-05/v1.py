from typing import *


def getRow(code: str):
    sr = code[0:7]
    t = ["0" if x == "F" else "1" for x in sr]
    t2 = "".join(t)
    ints = int(t2, 2)
    return ints


def getCol(code: str):
    sr = code[7:10]
    t = ["0" if x == "L" else "1" for x in sr]
    t2 = "".join(t)
    ints = int(t2, 2)
    return ints


def getID(code: str):
    return getRow(code) * 8 + getCol(code)


def getAnswer_1(data: List[str]):
    IDs = [getID(d) for d in data]
    return max(IDs)


def getAnswer_2(data: List[str]):
    IDs = [getID(d) for d in data]
    myID = next(ID for ID in range(0, 1000) if (
        ID not in IDs and ID - 1 in IDs and ID + 1 in IDs))
    return myID


if __name__ == "__main__":
    data = open('input.txt').read().splitlines()
    print("part one", getAnswer_1(data))
    print("part two", getAnswer_2(data))
