# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

IsMirroredAfterRow ← (
  ⊃(⇌↙|↘)
  ∩(⧻.)
  ⊙:
  ,:↧
  ⊙↙ ↙
  ≍
)

GetRowScore ← (
  ≡IsMirroredAfterRow ⊙¤ ↘1⇡⧻.
  ⬚0⊢+1⊚
)
GetColScore ← GetRowScore ⍉

SolvePartOne ← (
  ⊜□-:1⌕ "\n\n" .
  ≡(□⊐SplitBy@\n)
  + ⊃(/+⊐≡GetColScore|×100/+⊐≡GetRowScore)
)
---
⍤:≍, 405 SolvePartOne &fras "example_1"
⍤:≍, 34993 SolvePartOne &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
