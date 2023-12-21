# Uiua 0.7.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

# SolvePartOne ← |1 0
# ---
# ⍤:≍, x SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"

M ← ⊜∘≠@\n. &fras ("input"|"example_1") 0
S ← ⊢⊚=@S M

# y_x => [y1_x1 y2_x2 ...]
KNbs ← ☇1 +[0_1 1_0 0_¯1 ¯1_0] ¤

# y_x M
KIsWithinRange ← |2 /↧×⊃(≥0|<:⊙⧻)
---
⍤:≍, 1 KIsWithinRange 2_5 ↯ 10_10 0
⍤:≍, 0 KIsWithinRange ¯2_5 ↯ 10_10 0
⍤:≍, 0 KIsWithinRange 2_12 ↯ 10_10 0
---

# y_x M => c
EAtK ← ⊡⊙⊡ ⊃(⊢⇌|⊢)

M

⍥(
  ☇1≡KNbs                 # neighbors
  ▽≡(KIsWithinRange :M) . # within range
  ≡(EAtK :M) .            # chars
  ▽≠@#                    # remove rocks
  ⊏⍏.                     # sort
  ⊝                       # dedup
)64 S
⧻

# 3598, correct
