Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"

CalibrationValue ← parse /$"__" ♭ ∵parse ∵⊔ ⊂⊃(⊔♭↙1|⊔♭↙1⇌) (⊔∘|⊔▽[2]) =1⧻. regex "\\d"
---
⍤:≍, 12 CalibrationValue "1abc2"
⍤:≍, 38 CalibrationValue "pqr3stu8vwx"
⍤:≍, 15 CalibrationValue "a1b2c3d4e5f"
⍤:≍, 77 CalibrationValue "treb7uchet"
---

SolvePartOne ← /+ ≡(CalibrationValue ⊔) SplitLines
---
⍤:≍, 54968 SolvePartOne &fras "input"
---

Numberify ← (
  (∘|⍜(↙4)((∘|"z0ro") ≍"zero" .)) ≥4⧻.
  (∘|⍜(↙3)((∘|"o1e") ≍"one" .)) ≥3⧻.
  (∘|⍜(↙3)((∘|"t2o") ≍"two" .)) ≥3⧻.
  (∘|⍜(↙5)((∘|"t3ree") ≍"three" .)) ≥5⧻.
  (∘|⍜(↙4)((∘|"f4ur") ≍"four" .)) ≥4⧻.
  (∘|⍜(↙4)((∘|"f5ve") ≍"five" .)) ≥4⧻.
  (∘|⍜(↙3)((∘|"s6x") ≍"six" .)) ≥3⧻.
  (∘|⍜(↙5)((∘|"s7ven") ≍"seven" .)) ≥5⧻.
  (∘|⍜(↙5)((∘|"e8ght") ≍"eight" .)) ≥5⧻.
  (∘|⍜(↙4)((∘|"n9ne") ≍"nine" .)) ≥4⧻.
)

NumberifyUnderDrop ← ⍜↘Numberify ↧⧻,
MegaNumberifyUnderDrop ← (
  NumberifyUnderDrop 0
  NumberifyUnderDrop 1
  NumberifyUnderDrop 2
  NumberifyUnderDrop 3
  NumberifyUnderDrop 4
  NumberifyUnderDrop 5
  NumberifyUnderDrop 6
  NumberifyUnderDrop 7
  NumberifyUnderDrop 8
  NumberifyUnderDrop 9
  NumberifyUnderDrop 10
  NumberifyUnderDrop 11
  NumberifyUnderDrop 12
  NumberifyUnderDrop 13
  NumberifyUnderDrop 14
  NumberifyUnderDrop 15
  NumberifyUnderDrop 16
  NumberifyUnderDrop 17
  NumberifyUnderDrop 18
  NumberifyUnderDrop 19
  NumberifyUnderDrop 20
  NumberifyUnderDrop 21
  NumberifyUnderDrop 22
  NumberifyUnderDrop 23
  NumberifyUnderDrop 24
  NumberifyUnderDrop 25
  NumberifyUnderDrop 26
  NumberifyUnderDrop 27
  NumberifyUnderDrop 28
  NumberifyUnderDrop 29
  NumberifyUnderDrop 30
  NumberifyUnderDrop 31
  NumberifyUnderDrop 32
  NumberifyUnderDrop 33
  NumberifyUnderDrop 34
  NumberifyUnderDrop 35
  NumberifyUnderDrop 36
  NumberifyUnderDrop 37
  NumberifyUnderDrop 38
  NumberifyUnderDrop 39
  NumberifyUnderDrop 40
  NumberifyUnderDrop 41
  NumberifyUnderDrop 42
  NumberifyUnderDrop 43
  NumberifyUnderDrop 44
  NumberifyUnderDrop 45
  NumberifyUnderDrop 46
  NumberifyUnderDrop 47
  NumberifyUnderDrop 48
  NumberifyUnderDrop 49
  NumberifyUnderDrop 50
  NumberifyUnderDrop 51
  NumberifyUnderDrop 52
  NumberifyUnderDrop 53
)
---
⍤:≍, 29 CalibrationValue MegaNumberifyUnderDrop "two1nine"
⍤:≍, 83 CalibrationValue MegaNumberifyUnderDrop "eightwothree"
⍤:≍, 13 CalibrationValue MegaNumberifyUnderDrop "abcone2threexyz"
⍤:≍, 24 CalibrationValue MegaNumberifyUnderDrop "xtwone3four"
⍤:≍, 42 CalibrationValue MegaNumberifyUnderDrop "4nineeightseven2"
⍤:≍, 14 CalibrationValue MegaNumberifyUnderDrop "zoneight234"
⍤:≍, 76 CalibrationValue MegaNumberifyUnderDrop "7pqrstsixteen"
---

SolvePartTwo ← /+ ⊐≡(CalibrationValue) ⊐≡(MegaNumberifyUnderDrop) SplitLines
---
⍤:≍, 54094 SolvePartTwo &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
