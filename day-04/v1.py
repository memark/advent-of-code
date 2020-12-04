import re


def getPassports(data):
    return data.split("\n\n")


def parseField(field):
    return field.split(":")


def fieldIsRequired(field):
    return parseField(field)[0] in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]


def getFields(passport):
    return [f for f in re.split("[ \\n]{1}", passport) if f != ""]


def getPassportRequiredFields(passport):
    return [f for f in getFields(passport) if fieldIsRequired(f)]


def passportHasRequiredFields(passport):
    return len(getPassportRequiredFields(passport)) >= 7


def getFieldsForPassports(passports):
    return [getFields(p) for p in passports]


def getPassportsRequiredFields(passports):
    return [[f for f in getFields(p) if fieldIsRequired(f)] for p in passports]


def getValidPassports(passports):
    return [pf for pf in getPassportsRequiredFields(passports) if len(pf) >= 7]


def countValidPassports(data):
    return len(getValidPassports(data))


def getAnswer(data):
    return countValidPassports(getPassports(data))


# byr (Birth Year) - four digits; at least 1920 and at most 2002.
# iyr (Issue Year) - four digits; at least 2010 and at most 2020.
# eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
# hgt (Height) - a number followed by either cm or in:
# If cm, the number must be at least 150 and at most 193.
# If in, the number must be at least 59 and at most 76.
# hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
# ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
# pid (Passport ID) - a nine-digit number, including leading zeroes.
# cid (Country ID) - ignored, missing or not.


def isValidField(field):
    parsed = parseField(field)
    name = parsed[0]
    value = parsed[1]

    if name == "byr":
        return 1920 <= int(value) <= 2002
    if name == "iyr":
        return 2010 <= int(value) <= 2020
    if name == "eyr":
        return 2020 <= int(value) <= 2030
    if name == "hgt":
        return isValidHgt(value)
    if name == "hcl":
        return bool(re.match("^#[0-9a-f]{6}$", value))
    if name == "ecl":
        return value in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    if name == "pid":
        return bool(re.match("^[0-9]{9}$", value))
    if name == "cid":
        return True


def isValidHgt(value):
    m = re.match("^([0-9]+)(cm|in)$", value)
    if m == None:
        return False
    if m.group(2) == "cm":
        return 150 <= int(m.group(1)) <= 193
    if m.group(2) == "in":
        return 59 <= int(m.group(1)) <= 76


def getValidFields(fields):
    return [f for f in fields if isValidField(f)]


def printFields(fields):
    # print(fields)
    return True


def isValidPassport_2(passport):
    p = passport

    validFields = [f for f in getFields(p) if not f.startswith("cid")]

    return len(validFields) >= 7 and all(isValidField(f) for f in validFields) and printFields(validFields)


def getValidPassports_2(passports):
    return [p for p in passports if isValidPassport_2(p)]


def getAnswer_2(data):
    return len(getValidPassports_2(getPassports(data)))


if __name__ == "__main__":
    data = open('input.txt').read()
    print("part one", getAnswer(data))  # 07.24
    print("part two", getAnswer_2(data))
    print("passports", len(getPassports(data)))
    print("valid passports", len(getValidPassports_2(getPassports(data))))

    vps = getValidPassports_2(getPassports(data))
    print(">= 7", len([vp for vp in vps if len(getFields(vp)) >= 7]))
    print("< 7", len([vp for vp in vps if len(getFields(vp)) < 7]))

# 162 too high, 08.26
# 161 no
# 160 no
# 147, 08.52
