lines = open('input.txt').read().splitlines()
for xs in lines:
    for ys in lines:
        x = int(xs)
        y = int(ys)
        if (int(xs) + int(ys) == 2020):
            print(int(xs) * int(ys))
for xs in lines:
    for ys in lines:
        for zs in lines:
            if (int(xs) + int(ys) + int(zs) == 2020):
                print(int(xs) * int(ys) * int(zs))
