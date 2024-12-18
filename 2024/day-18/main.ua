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

💾"example.txt"
≡(⇌∵⋕↘₁) regex $ (\d*),(\d*)

↙₁₂ # temp?
$corrupted

$goal6_6
$start0_0

Neis ← ()

# ibneis ? curr
🏘 ← (
  $candidates+ $deltas⊸[∩¯,,⇌.⇡2]¤
  $valid ▽⊸≡(🏰:+₁ 6_6)
  ⊙(¤,)
  ▽⤚≡(¬/↥≡/↧⦷)
  ⊙⊙◌
  ⊙◌
)

⊢path🏘≍
&s⊸🗺!@O
-₁⧻
# ¤[2_2 3_3 5_5]
# [4_4 3_3 3_4 4_3]
# ▽⊸≡(¬/↥≡/↧⦷)
