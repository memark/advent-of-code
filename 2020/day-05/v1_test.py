from v1 import *


def test_getRow():
    assert getRow("F") == 0
    assert getRow("B") == 1
    assert getRow("FBFBBFFRLR") == 44


def test_getCol():
    assert getCol("xxxxxxxL") == 0
    assert getCol("xxxxxxxR") == 1
    assert getCol("FBFBBFFRLR") == 5


def test_getID():
    assert getID("FBFBBFFRLR") == 357


def test_all():
    assert getRow("BFFFBBFRRR") == 70
    assert getCol("BFFFBBFRRR") == 7
    assert getID("BFFFBBFRRR") == 567

    assert getRow("FFFBBBFRRR") == 14
    assert getCol("FFFBBBFRRR") == 7
    assert getID("FFFBBBFRRR") == 119

    assert getRow("BBFFBBFRLL") == 102
    assert getCol("BBFFBBFRLL") == 4
    assert getID("BBFFBBFRLL") == 820
