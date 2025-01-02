# https://adventofcode.com/2024/day/5

# uiua 0.15.0-dev.1
# Experimental!

UpdateMap ← (
  ⊙⊃(⬚(□[])get|⊙∘):
  insert:⍜(°□:)⊂
)

# map $ rules
🗺 ← ∧(UpdateMap°⊟₂) :[]

# updates rules $ data
🗄 ← (
  °/$"_\n\n_"
  ⍚°/$"_\n_"
  ∩°□:°⊟
  ⊓(≡(⍚⊜⋕⊸≠@,) ↘¯1|🗺≡(◇⊜⋕⊸≠@|))
)

# - för varje update
#   - try get i rules
#   - för varje i rules
#     - jämför mot kvarvarande updates

# bool $ update rules
IsCorrectlyOrdered ← (
  ≡⌟◇(/∨◇∊: ⬚[]get ⊙: °⊂ ⇌) ⧅□
  ¬/∨
)

# answer $ updates rules
🐪 ← (
  ▽⤚≡⌟◇IsCorrectlyOrdered
  ≡◇(⊡⌊÷2⊸⧻) # pick out the middle number
  /+
)
┌─╴🧪
  ⍤⤙≍ 143 🐪 🗄 &fras "example.txt"
  ⍤⤙≍ 6498 🐪 🗄 &fras "input.txt"
└─╴
