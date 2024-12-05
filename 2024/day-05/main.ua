# https://adventofcode.com/2024/day/5

# Uiua 0.14.0-dev.6
# Experimental!

# &fras "input.txt"
&fras "example.txt"
°/$"_\n\n_"

# ≡(□⊜□⊸≠@\n°□)
# ≡(□°/$"_\n_"°□)
⍚°/$"_\n_"
∩°□:°⊟

ParseFirst  ← ≡(◇⊜⋕⊸≠@|)
ParseSecond ← ≡(⍚⊜⋕⊸≠@,) ↘¯1 # last row is empty...

UpdateMap ← (
  ⊙⊃(⬚(□[])get|⊙∘):
  insert:⍜(°□:)⊂
)

⊓($second ParseSecond|$first ∧(UpdateMap°⊟₂) :[] ParseFirst)

S ←
F ←

# $first ∧(UpdateMap°⊟₂) F $map []
# # ⍜(?⬚(□[])get5?)(?□?⊂?:3°□?) # pattern match failed, bugg?
# S

# - för varje i second
#   - try get i first
#   - för varje i first
#     - jämför mot kvarvarande i second

# first: rules
# second: updates
# both: pages

S
# ↙2
$validrules ≡(
  ⇌°□
  ⇡⊸⧻
  ⊙¤
  ≡(↘
    ◇°⊂
    °□⬚[]get:F
    /∨∈:
  )
  /∨
  -:1
)
▽:S
/+≡(⊡⌊÷2⊸⧻°□)

# 6498, correct
