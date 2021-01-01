import mpmath
import operator
import numpy as np
import matplotlib.pyplot as plt
from numpy.core.arrayprint import format_float_positional
from numpy.core.records import recarray
from functools import *

# t = np.arange(0, 10)
# f = np.array([7*(t+0) for t1 in t])
# g = np.array([13*(t+1) for t1 in t])

# plt.plot(t, f, '-')
# plt.plot(t, g, '-')

# idx = np.argwhere(np.diff(np.sign(f - g))).flatten()
# print(idx, t[idx], f[idx], g[idx])
# plt.plot(t[idx], f[idx], 'ro')
# plt.show()


from itertools import *


def printt(*args):
    print(" => ".join((str(a) for a in args)))


a = "17,x,13,19"    # 3417
a = "67,7,59,61"    # 754018
a = "67,x,7,59,61"  # 779210
a = "67,7,x,59,61"  # 1261476
a = "1789,37,47,1889"  # 1202161486
print(f"{a=}")

b = a.split(",")
print(f"{b=}")

c = [(int(bb), i) for i in range(len(b)) if (bb := b[i]) != 'x']
print(f"{c=}")

d = reduce(operator.mul, [cc[0] for cc in c])
print(f"{d=}")

e = [(d/cc[0], cc[1]*d/cc[0]) for cc in c]
print(f"{e=}")

f = reduce(operator.add, [ee[0] for ee in e])
print(f"{f=}")

g = reduce(operator.add, [ee[1] for ee in e])
print(f"{g=}")


for x in count():
    t = (mpmath.mpf(d)*mpmath.mpf(x)-mpmath.mpf(g)) / mpmath.mpf(f)
    # if t.is_integer():
    if mpmath.isint(t):
        # print(x, t, test)
        v1 = (mpmath.mpf(f)*t+mpmath.mpf(g)) % mpmath.mpf(d)
        if v1 != 0:
            v2 = min(v1, d-v1)
            print(f"{t=} was close by {v2=}")
            continue
        printt(a, int(t), t, v1)
        exit()


exit()


def f1(t): return (t+0) % 67
def f2(t): return (t+1) % 7
def f3(t): return (t+2) % 59
def f4(t): return (t+3) % 61


print(next(t for t in range(10000000) if f1(t)+f2(t)+f3(t)+f4(t) == 0))


# tt = np.arange(3410, 3420)
# ff = np.array([f(t) for t in tt])
# gg = np.array([g(t) for t in tt])
# # hh = np.array([h(t) for t in tt])
# rr = ff+gg  # +hh

# for r in rr:
#     print(r)
# # for i in range(4000):

# print(tt)
# print(ff)
# print(gg)
# # print(hh)
# print(rr)

# plt.plot(tt, ff, '-')
# plt.plot(tt, gg, '-')
# plt.plot(tt, hh, '-')
# plt.plot(tt, rr, '-')
# plt.show()
