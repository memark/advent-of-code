from v1 import *


example = open('example.txt').read()


def test_getPassports():
    assert len(getPassports(example)) == 4


def test_getFields():
    assert len(getFields("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd")) == 4


def test_getFieldsForPassports():
    actual = getFieldsForPassports(
        ["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
         """byr:1937
        iyr:2017 cid:147"""])

    assert len(actual) == 2
    assert len(actual[0]) == 4
    assert len(actual[1]) == 3


def test_parseField():
    assert parseField("pid:860033327") == ["pid", "860033327"]


def test_fieldIsRequired_1():
    assert fieldIsRequired("pid:860033327")


def test_fieldIsRequired_2():
    assert not fieldIsRequired("cid:147")


def test_getPassportRequiredFields():
    print(getPassportRequiredFields("hgt:147 eyr:2020 cid:123"))
    assert len(getPassportRequiredFields("hgt:147 eyr:2020 cid:123")) == 2


def test_getPassportsRequiredFields():
    actual = getPassportsRequiredFields(
        ["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
         """byr:1937
         iyr:2017 cid:147"""])

    print(actual)
    assert len(actual) == 2
    assert len(actual[0]) == 4
    assert len(actual[1]) == 2


def test_getValidPassports_1():
    actual = getValidPassports(
        ["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
         """byr:1937
        iyr:2017 cid:147"""])

    print(actual)
    assert len(actual) == 0


def test_getValidPassports_2():
    actual = getValidPassports(
        ["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
         "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm",
         """ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147
        hgt:183cm"""])

    print(actual)
    assert len(actual) == 3


def test_countValidPassports():
    assert countValidPassports(getPassports(example)) == 2


def test_isValidHgt_1():
    assert not isValidHgt("aoe")
    assert not isValidHgt("160")
    assert not isValidHgt("cm")
    assert not isValidHgt("a160cm")
    assert not isValidHgt("160cmx")
    assert not isValidHgt("in")
    assert not isValidHgt("x160in")
    assert not isValidHgt("160inx")

    assert isValidHgt("60in")
    assert isValidHgt("190cm")
    assert not isValidHgt("190in")
    assert not isValidHgt("190")


def test_isValidField():
    assert isValidField("byr:2002")
    assert not isValidField("byr:2003")

    assert isValidField("hgt:60in")
    assert isValidField("hgt:190cm")
    assert not isValidField("hgt:190in")
    assert not isValidField("hgt:190")

    assert isValidField("hcl:#123abc")
    assert not isValidField("hcl:#123abz")
    assert not isValidField("hcl:123abc")

    assert isValidField("ecl:brn")
    assert not isValidField("ecl:wat")

    assert isValidField("pid:000000001")
    assert not isValidField("pid:0123456789")


def test_isValidPassport_2():
    assert isValidPassport_2(
        """pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f""")
    assert isValidPassport_2(
        """eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm""")
    assert isValidPassport_2(
        """hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022""")
    assert isValidPassport_2(
        """iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719""")
    assert not isValidPassport_2(
        """eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926""")
    assert not isValidPassport_2(
        """iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946""")
    assert not isValidPassport_2(
        """hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277""")
    assert not isValidPassport_2(
        """hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007""")
