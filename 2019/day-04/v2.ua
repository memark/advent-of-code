# https://adventofcode.com/2019/day/4

NoDesc ← |1 ≍ ⊏⍏. .
---
⍤. =1 NoDesc "111111"
⍤. =0 NoDesc "223450"
---

AdjSame! ← |1 ≥1⧻ ▽^1. ∵⧻ ⊜□ . -@0
---
⍤. =1 AdjSame!'≥2 "122345"
⍤. =1 AdjSame!'≥2 "111111"
⍤. =0 AdjSame!'≥2 "123789"
---
---
⍤. =1 AdjSame!'=2 "112233"
⍤. =0 AdjSame!'=2 "123444"
⍤. =1 AdjSame!'=2 "111122"
---

RangeFromInterval ← ∵$"_" +⇡+1-,∶ ⍘⊟ ∵parse ⊜□ ≠, @-
---
⍤. ≍["111" "112" "113"] RangeFromInterval "111-113"
---

PuzzleInput ← "178416-676461"
&pf "Part 1: "
&p ⧻ ▽≡(NoDesc). ▽≡(AdjSame!'≥2). RangeFromInterval PuzzleInput
&pf "Part 2: "
&p ⧻ ▽≡(NoDesc). ▽≡(AdjSame!'=2). RangeFromInterval PuzzleInput
