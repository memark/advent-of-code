from v2 import getAnswer

lines = open('example.txt').read().splitlines()


def test_input_part_one():
    actual = getAnswer(lines, 3, 1) == 7


def test_input_part_two_a():
    actual = getAnswer(lines, 1, 1) == 2


def test_input_part_two_b():
    actual = getAnswer(lines, 3, 1) == 7


def test_input_part_two_c():
    actual = getAnswer(lines, 5, 1) == 3


def test_input_part_two_d():
    actual = getAnswer(lines, 7, 1) == 4


def test_input_part_two_e():
    actual = getAnswer(lines, 1, 2) == 2
