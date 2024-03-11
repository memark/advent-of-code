# Uiua 0.9.5

SplitBy ← |2 ⊜□≠,

PredictionForRow ← (
  # make start row look like the rest
  ¤□
  # calc the rows going down
  ⍢(
    ⊂: □≡(-⊃(⊢|⊢⇌))◫2°□⊡¯1 .
  | # not all zeroes?
    ≠0◇/+⊡¯1
  )
  &s.
  # extrapolate new last value going up
  # TODO: borde kunna använda omvända windows här också!
  ⍢(
    ⊃(⊡¯1°□⊡¯1|⊡¯1°□⊡¯2|⊡¯2|↘¯2)°□
    ⊂:+
    ⊂:□
  | >1⧻
  )
  &s.
  # take last value
  °□◇⊡¯1◇⊢
)

SolvePartOne ← /+ ≡PredictionForRow ≡(⋕◇SplitBy@ ) SplitBy@\n
---
⍤:≍, 114 SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
---

# &p $"Part 1: _" SolvePartOne &fras "input"
# 1581680965 too high

# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---
