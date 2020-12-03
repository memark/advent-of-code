def getAnswer(lines, right, down):
    yl = len(lines)
    xl = len(lines[0])

    r = 0

    xf = right
    yf = down

    n = 0
    x = 0
    y = 0

    for n in range(1, yl // yf):

        x = (n * xf) % xl
        y = (n * yf)

        c = lines[y][x]

        if c == "#":
            r = r + 1

    return r


if __name__ == "__main__":
    lines = open('input.txt').read().splitlines()
    print("part one", getAnswer(lines, 3, 1))
    print("part two",
          getAnswer(lines, 1, 1)
          * getAnswer(lines, 3, 1)
          * getAnswer(lines, 5, 1)
          * getAnswer(lines, 7, 1)
          * getAnswer(lines, 1, 2))
