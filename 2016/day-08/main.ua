# Uiua 0.13.0
# Experimental!

Lib ~ "../lib.ua"

TurnOnRectangle  ← ⬚0+ =0↯⊙⊚:
RotateRowRight   ← ⍜(⊙⊡:)(↻¯)
RotateColumnDown ← ⍜(⊙⊡:⊙⊙⍉)(↻¯)

# matrix ? instr matrix
ProcessLine ← ⍣(
  case(TurnOnRectangle ∩⋕) °$"rect _x_"
| case(RotateColumnDown ∩⋕) °$"rotate column x=_ by _"
| case(RotateRowRight ∩⋕) °$"rotate row y=_ by _"
)

# output ? input x y
Solve ← (
  ⊜□⊸≠@\n
  ⊙Lib~CreateZeroMatrix
  ∧ProcessLine
)

PartOne ← /+/+ Solve

┌─╴test
  ⍤⤙≍ 6 PartOne &fras "example.txt" 7 3
  ⍤⤙≍ 119 PartOne &fras "input.txt" 50 6
└─╴

PartTwo ← Lib~CreateTextFromMatrix Solve

&p PartTwo &fras "input.txt" 50 6
