lines = open('input.txt').read().splitlines()
numbers = [int(x) for x in lines]
print(next(x * y for x in numbers for y in numbers if x + y == 2020))
print(next(x * y * z for x in numbers for y in numbers for z in numbers if x + y + z == 2020))
