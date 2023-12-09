Compare ← |2 ±-
---
⍤:≍, 1 Compare 3 5
⍤:≍, 0 Compare 3 3
⍤:≍, ¯1 Compare 5 3
---

SplitBy ← ⊜□≠,
SplitByDelimiter ← ⊜□≠,
---
---

SplitLines ← ⊜□≠, @\n
---
⍤:≍, {"hej" "hoj"} SplitLines "hej\n\nhoj"
⍤:≍, {"hej" "hoj"} SplitLines "hej\nhoj\n\n"
⍤:≍, {"hej" "hoj"} SplitLines "\n\nhej\nhoj"
⍤:≍, {"heja" "hoj"} SplitLines "\n\nheja\nhoj"
---

StartsWith ← =1⊢⌕
---
⍤:≍, 1 StartsWith "eight" "eightwothree"
⍤:≍, 0 StartsWith "two" "eightwothree"
---

Trim ← ▽×⍜(⊟⇌)≡\↥.≠@ .
---
---

# https://old.aplwiki.com/SieveOfEratosthenes
PrimesLessThan ← ▽ =2/+=0⍉rows◿¤. .⇡
---
⍤:≍, 2_3_5_7 PrimesLessThan 10
⍤:≍, 2_3_5_7 PrimesLessThan 11
⍤:≍, 2_3_5_7_11 PrimesLessThan 12
---

PrimeFactors ← ▽: ⊙(=0◿) . PrimesLessThan .
---
⍤:≍, ¤5 PrimeFactors 25
⍤:≍, 2_3_17 PrimeFactors 102
---

SetIntersection ← ⊝▽∊,
---
⍤:≍, 2_3 SetIntersection 1_2_3 2_3_4_2
---

# Greatest Common Divisor
GCD ← /×SetIntersection ∩PrimeFactors
---
⍤:≍, 15 GCD 30 75
---

# LeastCommonMultiple
LCM ← ÷⊃(GCD|×)
---
⍤:≍, 150 LCM 30 75
---
