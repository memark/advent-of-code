# Uiua 0.4.1 (0.6.1 not working here. what about 0.5.1?)

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"
  StartsWith ← Lib "StartsWith"
  Trim ← Lib "Trim"

Fst ← ⊡0
Snd ← ⊡1
Trd ← ⊡2

And ← ↧
Or ← ↥
Not ← -:1
# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

PredictionForRow ← (
  # make start row look like the rest
  ¤□
  # calc the rows going down
  ⍢(
    ⊂: □≡(-⊃(⊢|⊢⇌))◫2⊔ ⊡¯1 .
  )(
    # not all zeroes?
    ≠0/+⊔⊡¯1
  )
  # extrapolate new last value going up
  # TODO: borde kunna använda omvända windows här också!
  ⍢(
    ⊐⊃(⊡¯1⊔⊡¯1|⊡¯1⊔⊡¯2|⊔⊡¯2|↘¯2)
    ⊂:+
    ⊂:□
  )(>1⧻)
  # take last value
  ⊡¯1⊔⊢
)

SolvePartOne ← /+ ≡PredictionForRow ≡(⊐≡parse ⊐SplitBy@ ) SplitBy@\n
---
⍤:≍, 114 SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
---

&p $"Part 2: _" SolvePartOne &fras "input"
# 1581680965 too high
