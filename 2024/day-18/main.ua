# https://adventofcode.com/2024/day/18

# uiua 0.14.0-rc.3
# Experimental!

💾 ← &fras

# map ? coords
🗺! ← (
  °⊚
  ⨬@.^0
  $map
)

# ? coord coordmax
🏰 ← $isinside ×⊃(/×<:|/×≥₀)

# 🏘 ← (
#   +⊸[∩¯,,⇌.⇡2]¤
# #   ▽⊸≡(🏰:△Grid)
# )

# # validNeighbors ? current
# 🔜 ← (
#   ⊸🏘
#   ⊙(+1⊡:Grid)     # ta fram värdet+1
#   ▽:⊙=:⊸≡(⊡:Grid) # kolla dess värden
# )

Example ← 0

💾⨬("input.txt"|"example.txt")Example
≡(⇌∵⋕↘₁) regex $ (\d*),(\d*)

# ↙₁₂
# ↙₁₀₂₄
$corrupted

# $goal70_70
$goal⨬(70_70|6_6)Example
$start0_0

# ibneis ? curr goal corrupted
🏘 ← (
  ⊙◌
  $candidates+ $deltas⊸[∩¯,,⇌.⇡2]¤
  $valid ▽⊸≡(🏰:+₁ ⨬(70_70|6_6)Example)
  ▽⤚≡(¬/↥≡/↧⦷) ⊙¤
)

# ¤[2_2 3_3 5_5]
# [4_4 3_3 3_4 4_3]
# ▽⊸≡(¬/↥≡/↧⦷)

F ← (
  path🏘≍
  # &s⊸🗺!@O
  # -₁
  ⧻⊢
)

G ← F ⊃(⋅⊙∘|↙⊙⋅⋅∘)

# G 21

⋅⋅($"_,_"°⊟⇌⊡3032)
# 65,6 correct

# man skulle kunna se det som iterationer där datan isf inte hänger ihop
# men lika gärna bara göra rows på en range, tror jag börjar med det
# 🐫 ← |2 ⊙⋅⋅◌ ⍢⊓+₁(:⊙⊙◌ ◡🔂)⋅⋅(¬🎄) 0 🔢

# ⨬(+1024 ⇡ -1024 3450|+1⇡25)Example
# ◌wait≡pool(&p$"_: _"⟜G) ∩₃¤
# G 3033
