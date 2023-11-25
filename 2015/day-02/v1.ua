SplitLines ← ⊜□≠, @\n

ParseRow ← ⊜parse ≠, @x

LW ← /×⊏0_1
WH ← /×⊏1_2
HL ← /×⊏0_2

# Testa planet notation här
CalcPaper ← + /+×2: /↧. [⊃⊃LW WH HL]
---
⍤:≍, 58 CalcPaper ParseRow "2x3x4"
⍤:≍, 43 CalcPaper ParseRow "1x1x10"
---

SolvePartOne ← /+ ⊐≡(CalcPaper ParseRow) SplitLines
---
⍤:≍, 1598415 SolvePartOne &fras "input"
---

CalcRibbon ← + ⊃(/×)(×2/+ ↙2⊏⍏.)
---
⍤:≍, 34 CalcRibbon ParseRow "2x3x4"
⍤:≍, 14 CalcRibbon ParseRow "1x1x10"
---

SolvePartTwo ← /+ ⊐≡(CalcRibbon ParseRow) SplitLines
---
⍤:≍, 3812909 SolvePartTwo &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
