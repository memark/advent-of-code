# pyright: reportMissingImports=false, reportUndefinedVariable=false

from v2 import *


def test_getID():
    assert getID("FBFBBFFRLR") == 357
    assert getID("BFFFBBFRRR") == 567
    assert getID("FFFBBBFRRR") == 119
    assert getID("BBFFBBFRLL") == 820


def test_input():
    data = open('input.txt').read().splitlines()

    assert getAnswer_1(data) == 832
    assert getAnswer_2(data) == 517
