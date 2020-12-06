# pyright: reportWildcardImportFromLibrary=false
# pyright: reportMissingImports=false, reportUndefinedVariable=false

from typing import *


sample = """abc

a
b
c

ab
ac

a
a
a
a

b"""

data = open('input.txt').read()


groups = data.strip().split("\n\n")
answers = [list(g.replace("\n", "")) for g in groups]
answersUniq = [set(a) for a in answers]
answersCount = [len(a) for a in answersUniq]
s = sum(answersCount)
print(s)

# c = [([1 for a in g if len()]) for g in answersUniq]

m = 0

for i, g in enumerate(answersUniq):
    n = 0
    # print(g)
    for au in g:
        #gl = len(g)
        gl = len(groups[i].split("\n"))
        ac = answers[i].count(au)
        # print(i, au, gl, answers[i], ac)
        if ac == gl:
            n = n + 1
    # print(n)
    m = m + n

print(m)


if __name__ == "__main__":
    pass
