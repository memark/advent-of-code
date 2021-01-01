def getAnswer(lines, right, down):
    yl = len(lines)
    xl = len(lines[0])
    yf = down
    xf = right

    def stepIsTree(n):
        x = (n * xf) % xl
        y = (n * yf)
        c = lines[y][x]
        return c == "#"

    result = sum(stepIsTree(n) for n in range(1, yl // yf))

    return result


if __name__ == "__main__":
    lines = open('input.txt').read().splitlines()
    print("part one", getAnswer(lines, 3, 1))
    print("part two",
          getAnswer(lines, 1, 1)
          * getAnswer(lines, 3, 1)
          * getAnswer(lines, 5, 1)
          * getAnswer(lines, 7, 1)
          * getAnswer(lines, 1, 2))
