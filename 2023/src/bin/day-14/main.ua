# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

F ← |1 (
  ⊚⌕".O".
  ⊙(⍜(⊏|↯:@O⧻)) .
  ⍜(⊏|↯:@.⧻)+1
)

CollapseRow ← ; ⍢(F|⊙.=0≍,:) F.

TiltNorth ← ⍜⍉≡CollapseRow

SolvePartOne ← (
  SplitBy@\n
  TiltNorth
  =@O
  ×⇌+1⇡⧻.
  /+♭
)
---
⍤:≍, 136 SolvePartOne &fras "example_1"
⍤:≍, 106517 SolvePartOne &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
