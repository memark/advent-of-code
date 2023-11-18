# https://adventofcode.com/2019/day/12

Lib ← &i "../lib.ua"
  Compare ← Lib "Compare"
  SplitLines ← Lib "SplitLines"

ApplyGravityy!!! ← ⍜(⊡^0)+∶ ↻3♭ ⬚0↯1_6↙3 ≡(Compare ⊃(⊡^0)(⊡^0)) ⍉.

ApplyGravity ← (|1
  ApplyGravityy!!!0 0 1
  ApplyGravityy!!!0 0 2
  ApplyGravityy!!!0 0 3

  ApplyGravityy!!!1 1 0
  ApplyGravityy!!!1 1 2
  ApplyGravityy!!!1 1 3

  ApplyGravityy!!!2 2 0
  ApplyGravityy!!!2 2 1
  ApplyGravityy!!!2 2 3

  ApplyGravityy!!!3 3 0
  ApplyGravityy!!!3 3 1
  ApplyGravityy!!!3 3 2
)

ApplyVelocity ← |1 ≡(⊂+,⊃↙↘3)

TotalEnergy ← |1 /+≡(×⊃(/+⌵↙3)(/+⌵↘3))

SolvePartOne ← (|2
  SplitLines∶
  ∵(∵(parse ⊔)⊐regex "-?\\d+")
  ≡(⊂∶0_0_0)
  ⍥(ApplyVelocity ApplyGravity)∶
  TotalEnergy
)
---
⍤∶≍, 1940 SolvePartOne 100 &fras "example_2.txt"
⍤∶≍, 14780 SolvePartOne 1000 &fras "input.txt"
---

&p $"Part 1: _" SolvePartOne 1000 &fras "input.txt"
