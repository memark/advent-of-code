# Uiua 0.9.5

SplitBy ← |2 ⊜□≠,

PredictionForRowEnd ← (
  # make start row look like the rest
  ¤□
  # calc the rows going down
  ⍢(
    ⊂: □≡(-⊃(⊢|⊢⇌))◫2°□⊡¯1 .
  | # not ALL zeroes?
    ≠0◇/+≠0⊡¯1
  )
  # extrapolate new last value going up
  # TODO: borde kunna använda omvända windows här också!
  ⍢(
    ⊃(⊡¯1°□⊡¯1|⊡¯1°□⊡¯2|⊡¯2|↘¯2)°□
    ⊂:+
    ⊂:□
  | >1⧻
  )
  # take last value
  °□◇⊡¯1◇⊢
)

SolvePartOne ← /+ ≡PredictionForRowEnd ≡(⋕◇SplitBy@ ) SplitBy@\n
---
⍤:≍, 114 SolvePartOne &fras "example_1"
⍤:≍, 1581679977 SolvePartOne &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"

PredictionForRowStart ← (
  # make start row look like the rest
  ¤□
  # calc the rows going down
  ⍢(
    &s . ⊂: □≡(-⊃(⊢|⊢⇌))◫2°□⊡0 .
  | # not ALL zeroes?
    ≠0◇/+≠0⊡0
  )
  &s .
  # extrapolate new first value going up
  # TODO: borde kunna använda omvända windows här också!
  ⍢(
    ⊃(⊡0°□⊡0|⊡0°□⊡1|⊡1|↘1)°□
    ⊂:+
    ⊂:□
  | >1⧻
  )
  # take first value
  °□◇⊡0◇⊢
)

SolvePartTwo ← /+ ≡PredictionForRowStart ≡(⋕◇SplitBy@ ) SplitBy@\n
# ---
⍤:≍, 2 SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

&p $"Part 2: _" SolvePartTwo &fras "input"

# ---
