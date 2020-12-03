# Part 1

import re
lines = open('input.txt').read().splitlines()

p = re.compile('(\w*)-(\w*) (\w): (\w*)')

n = 0

for l in lines:
    m = p.match(l)

    min = int(m.group(1))
    max = int(m.group(2))
    char = m.group(3)
    pw = m.group(4)

    count = pw.count(char)
    ok = min <= count <= max
    if ok:
        n = n + 1

print(n)

# Part 2

n = 0

for l in lines:
    m = p.match(l)

    p1 = int(m.group(1))
    p2 = int(m.group(2))
    char = m.group(3)
    pw = m.group(4)

    ok1 = pw[p1 - 1] == char
    ok2 = pw[p2 - 1] == char

    ok = ok1 != ok2
    if ok:
        n = n + 1

print(n)
