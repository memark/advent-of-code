# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false


import pytest
from v1 import *


@pytest.mark.parametrize("input, expected", [
    ('1', '1'),
    ('(1)', '1'),
    ('((1))', '1'),
    ('1+2', '3'),
    ('1*2', '2'),
    ('(1+2)', '3'),
    ('((1)+2)', '3'),
    ('(((1)+2))', '3'),
    ('1+2+3', '6'),
    ('1*2*3', '6'),
    ('1+2*3', '9'),
    ('2+(8*3)', '26'),
    ('2+(8*3+1)', '27'),
    ('2+(8*3*2)', '50'),
])
def test_evalexpr_1a(input, expected):
    assert evalexpr_1a(input) == expected
