# https://adventofcode.com/2019/day/4

NeverDecreases ← |1 ≍ ⊏⍏. .
---
⍤. =1 NeverDecreases "111111"
⍤. =0 NeverDecreases "223450"
---

AtLeastTwoAdjacentSameDigits ← ≥1⧻ ▽≥2. ∵⧻ ⊜□ . -@0
---
⍤. =1 AtLeastTwoAdjacentSameDigits "122345"
⍤. =1 AtLeastTwoAdjacentSameDigits "111111"
⍤. =0 AtLeastTwoAdjacentSameDigits "123789"
---

ExactlyTwoAdjacentSameDigits ← ≥1⧻ ▽=2. ∵⧻ ⊜□ . -@0
---
⍤. =1 ExactlyTwoAdjacentSameDigits "112233"
⍤. =0 ExactlyTwoAdjacentSameDigits "123444"
⍤. =1 ExactlyTwoAdjacentSameDigits "111122"
---

RangeFromInterval ← ∵$"_" +⇡+1-,∶ ⍘⊟ ∵parse ⊜□ ≠, @-
---
⍤. ≍["111" "112" "113"] RangeFromInterval "111-113"
---

PuzzleInput ← "178416-676461"
&pf "Part 1: "
&p ⧻ ▽≡(NeverDecreases). ▽≡(AtLeastTwoAdjacentSameDigits). RangeFromInterval PuzzleInput
&pf "Part 2: "
&p ⧻ ▽≡(NeverDecreases). ▽≡(ExactlyTwoAdjacentSameDigits). RangeFromInterval PuzzleInput
