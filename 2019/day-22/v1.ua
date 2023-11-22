# https://adventofcode.com/2019/day/22
# Uiua 0.3.1

FactoryOrder ← |1 ⇡
---
⍤:≍, [0 1 2 3 4 5 6 7 8 9] FactoryOrder 10
---

DealIntoNewStack ← |1 ⇌
---
⍤:≍, [9 8 7 6 5 4 3 2 1 0] DealIntoNewStack [0 1 2 3 4 5 6 7 8 9]
---

Cut ← |2 ↻
---
⍤:≍, [3 4 5 6 7 8 9 0 1 2] Cut 3 [0 1 2 3 4 5 6 7 8 9]
⍤:≍, [6 7 8 9 0 1 2 3 4 5] Cut ¯4 [0 1 2 3 4 5 6 7 8 9]
---

# DealWithIncrement ← |2 ◿ ⧻. -: ⧻. ×
DealWithIncrement ← (
  ⊙(⇡⧻.)
  ◿ ⧻. -: ⧻. ×
  ⊏
)
---
⍤:≍, [0 7 4 1 8 5 2 9 6 3] DealWithIncrement 3 [0 1 2 3 4 5 6 7 8 9]
⍤:≍, [1 8 5 2 9 6 3 10 7 4] DealWithIncrement 3 [1 2 3 4 5 6 7 8 9 10]
---

---
FactoryOrder 10
DealWithIncrement 7
DealIntoNewStack
DealIntoNewStack
⍤:≍, [0 3 6 9 2 5 8 1 4 7]

FactoryOrder 10
Cut 6
DealWithIncrement 7
DealIntoNewStack
⍤:≍, [3 0 7 4 1 8 5 2 9 6]

FactoryOrder 10
DealWithIncrement 7
DealWithIncrement 9
DealWithIncrement 9
Cut ¯2
# ⍤:≍, [6 3 0 7 4 1 8 5 2 9]
---

# .FactoryOrder 10
# .Cut 6
# .DealWithIncrement 7
# .DealIntoNewStack

+1⇡10
×7.
:-100
◿10
⊏
