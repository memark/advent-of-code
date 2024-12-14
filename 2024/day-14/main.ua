# https://adventofcode.com/2024/day/14

# uiua v0.14.0-rc.1
# Experimental!

# ⊞⋅⋅0.⇡ 7 # ett sätt att få 0-matris
# jag vill byta håll på x och y
# jag vill regexa ut delarna av raden var för sej
# skicka PR med decimal sleep
# gör en gif av del två

🔢 ← ≡⊃↘↙2 ⋕≡↘₁regex$ p=(\d*),(\d*) v=(-?\d*),(-?\d*)

# robots ? denormRobots shape
📥 ← ≡◿¤:

# robots ? delta robots shape
🔂 ← 📥+

# grid = robots shape
MakeGrid ← ⬚0↯:°⊚

# factor ? grid
🧮 ← (
  ⍜↻↘₁⌊÷₂⊸⧻     # drop mid row
  ⍜⍉(⍜↻↘₁⌊÷₂⊸⧻) # drop mid col
  ♭₋₁⧈∘¤÷2⊸△    # split into quadrants
  /×≡(/+/+)     # calc factor
)

# factor ? data shape
🐪 ← (
  🧮 MakeGrid ⊸🔂×₁₀₀🔢
)
┌─╴🧪
  ⍤⤙≍ 12 🐪 &fras"example.txt" 11_7
  ⍤⤙≍ 228690000 🐪 &fras"input.txt" 101_103
└─╴

# grid ? grid
✨ ← ⨬@.@*± ⍉

# ? robots shape
# grid ? grid
🖨 ← &p /$"_\n_" ✨ MakeGrid

# bool ? robots shap
🎄 ← ±/↥⌕"*******************************" /$"_\n_" ✨ MakeGrid

# secs ? data shape
🐫 ← |2 ⊙⋅⋅◌ ⍢⊓+₁(:⊙⊙◌ ◡🔂)⋅⋅(¬🎄) 0 🔢
┌─╴🧪
  ⍤⤙≍ 7093 🐫 &fras"input.txt" 101_103
└─╴
