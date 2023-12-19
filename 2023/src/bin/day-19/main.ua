# Uiua 0.7.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

# SolvePartOne ← |1 0
# ---
# ⍤:≍, x SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"

# &fras "example_1"
# :°⊟ ⊜□-:1⌕"\n\n".
# ∩(⊜□ ≠,@\n °□)

ParseRatingLine ← ⊜(□⊜□≠@=.)≠@,. ↘¯1↘1
# ParseRatingLine ← ↯2_4 ↘1☇0regex"\\{(x)=(\\d+),(m)=(\\d+),(a)=(\\d+),(s)=(\\d+)\\}"
ParseRatingLine "{x=787,m=2655,a=1222,s=2876}"

ParseWorkflowLine ← ⊜□≠@,. ↘¯1 °□⊡1 ⊜□≠@{.
# ParseWorkflowLine "px{a<2006:qkq,m>2090:A,rfg}"

# ParseCondition ← ⊜□↧⊃(≠@<|≠@:).
ParseCondition ← (
  °□ ⊜□≠@:.
  ⊃(⊡0|⊡1)
  ↘1☇0regex"([xmas])([<>])(\\d*)" °□
  # ⍜(⊡0)(⊢)
  # ⍜(⊡1)(⊢)
  ⍜(⊡2)(⋕)
  ⊂
)
ParseCondition "a<2006:qkq"
