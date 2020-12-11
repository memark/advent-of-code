# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from v1 import *
import pytest

# def test_1():
#     assert magic([0]) == 1
#     assert magic([0, 1]) == 1
#     assert magic([0, 2]) == 1
#     assert magic([0, 3]) == 1
#     assert magic([0, 1, 4]) == 1
#     assert magic([0, 1, 2, 4]) == 2


sample_1a = sorted([
    16,
    10,
    15,
    5,
    1,
    11,
    7,
    19,
    6,
    12,
    4,

    0,
    22
])


sample_1b = sorted([
    28,
    33,
    18,
    42,
    31,
    14,
    46,
    20,
    48,
    47,
    24,
    23,
    49,
    45,
    19,
    38,
    39,
    11,
    1,
    32,
    25,
    35,
    8,
    17,
    7,
    9,
    4,
    2,
    34,
    10,
    3,

    0,
    52
])


@pytest.mark.parametrize("input, expected", [
    ([0], 1),
    ([0, 1], 1),
    ([0, 2], 1),
    ([0, 3], 1),
    ([0, 1, 4], 1),
    ([0, 1, 2, 4], 3),
    (sample_1a, 8),
    (sample_1b, 19208)
])
def test_2(input, expected):
    assert magic(input) == expected
