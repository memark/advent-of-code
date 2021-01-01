import re


def parts(line):
    m = re.compile('(\d+)-(\d+) (\w): (\w+)').match(line)
    return (int(m.group(1)), int(m.group(2)), m.group(3), m.group(4))


def policy_1(line):
    (min, max, char, pw) = parts(line)
    count = pw.count(char)
    return min <= count <= max


def policy_2(line):
    (p1, p2, char, pw) = parts(line)
    p1ok = pw[p1 - 1] == char
    p2ok = pw[p2 - 1] == char
    return p1ok != p2ok


def testPolicy(lines, policyFunc):
    return sum(policyFunc(l) for l in lines)


lines = open('input.txt').read().splitlines()

print("part one:", testPolicy(lines, policy_1))
print("part two:", testPolicy(lines, policy_2))
