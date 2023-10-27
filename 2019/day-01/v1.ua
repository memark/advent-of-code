# https://adventofcode.com/2019/day/1

Fuel ← ↥0-2⌊÷3
---
⍤. =2 Fuel 12
⍤. =2 Fuel 14
⍤. =654 Fuel 1969
⍤. =33583 Fuel 100756
⍤. =0 Fuel 2
---

SplitLines ← ⊜□ ≠, @\n
---
⍤. ≍{"ab" "cd"} SplitLines "ab\ncd"
---

RecFuel ← |1 /+[⍥(⎋=0..Fuel)∞]
---
⍤. =966 RecFuel 1969
⍤. =50346 RecFuel 100756
---

# Part 1
/+ ∵Fuel ∵parse SplitLines &fras "input.txt"

# Part 2
/+ ∵RecFuel ∵parse SplitLines &fras "input.txt"
