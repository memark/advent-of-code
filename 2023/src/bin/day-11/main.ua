# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

GetEmptyRows ← ⊚=0≡/↥ =@#

# [y_x y_x] -> bool
PairsAreDifferent ← ≠1/↧/=

# m -> [y1_x1 y2_x2 ...]
GetUniqueGalaxyPairs ← (
  ⊚=@#
  ☇2⊠⊟.
  ⊝≡(⊏⍏.)
  ▽≡PairsAreDifferent.
)

# xs [x1 x2] -> n
NumBetween ← /+ ☇0 ⊠(↧⊃(<⊡1|>⊡0)) ¤⊏⍏. :

# exp_factor xs ys [y1_x1 y2_x2] -> z
DistanceForPairWithExpansion ← (
  ⊙⊙⊙.
  × -1⊙(+ ⊙NumBetween: ⊙NumBetween ⊙⊙(⊃(⊡0|⊡1)⍉))
  + ⊙(/+⌵/-)
)

# exp_factor file
SolveProblem ← (
  ⊙(
    SplitBy@\n
    ⊃(¤GetEmptyRows⍉|¤GetEmptyRows|GetUniqueGalaxyPairs)
  )
  ≡(DistanceForPairWithExpansion)
  /+
)

SolvePartOne ← SolveProblem 2
---
⍤:≍, 374 SolvePartOne &fras "example_1"
⍤:≍, 9623138 SolvePartOne &fras "input"
---

SolvePartTwo ← SolveProblem 1000000
---
⍤:≍, 726820169514 SolvePartTwo &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
