# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

# c n => n
F ← |2 ◿256 ×17 + -@\0
# s n => s n
G ← |2.2 :F ⊙: ⊃(⊢|↘1)
# s => n
H ← |1 ;⍢(G|>0⧻) :0
# i => n
SolvePartOne ← |1 (
  ⊜□≠, @,
  ⊐≡H
  /+
)
---
⍤:≍, 1320 SolvePartOne &fras "example_1"
⍤:≍, 516804 SolvePartOne &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
