lines = open('input.txt').read().splitlines()

# n  # vilket stop
# x  # aktuell kolumn
# y  # aktuell rad

yl = len(lines)
xl = len(lines[0])
print('yl', yl)
print('xl', xl)

r = 0

xf = 1
yf = 2

print("yl // yf", yl // yf)

for n in range(1, yl // yf + 1):

    x = (n * xf) % xl
    y = (n * yf)

    c = lines[y][x]

    if c == "#":
        r = r + 1

    print("*")

print("r", r)


print("answer", 66 * 153 * 79 * 92 * 33)
