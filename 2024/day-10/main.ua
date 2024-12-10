# https://adventofcode.com/2024/day/10

# uiua v0.14.0-dev.7
# Experimental!

~ "lib.ua" ~ 🏰 👨‍💻‼

Grid ← ⊜≡⍣(⋕|¯1)⊸≠@\n &fras"input.txt"

🏘 ← (
  +⊸[∩¯,,⇌.⇡2]¤
  ▽⊸≡(🏰:△Grid)
)

HeadsAndFeet ← ♭₋₁⊞⊟:⊃(⊚=₉|⊚=₀)

📐 ← ⍤⤙≍⊙△

# validNeighbors ? current
🔜 ← (
  ⊸📐[2]
  ⊸🏘
  ⊙(+1⊡:Grid)     # ta fram värdet+1
  ▽:⊙=:⊸≡(⊡:Grid) # kolla dess värden
)

Solve! ← /+^0 ≡(⧻ 👨‍💻‼🔜≍ °⊟₂) HeadsAndFeet

🐪 ← Solve!± Grid
┌─╴🧪
  ⍤⤙≍ 550 🐪
└─╴

🐫 ← Solve!∘ Grid
┌─╴🧪
  ⍤⤙≍ 1255 🐫
└─╴
