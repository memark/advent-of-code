# https://adventofcode.com/2015/day/2

# Uiua 0.13.0
# Experimental!

📦 ← + ⊃(/+×₂|⊢⍆) ⍜°⊟₃⊃(×⊙∘|×⊙⋅∘|×⋅⊙∘)
┌─╴test
  ⍤⤙≍ 58 📦 2_3_4
  ⍤⤙≍ 43 📦 1_1_10
└─╴

🐪 ← /+ ⊜(📦⊜⋕⊸≠@x)⊸≠@\n
┌─╴test
  ⍤⤙≍ 1598415 🐪 &fras "input"
└─╴

🎗️ ← + ⊃(/×)(×₂/+ ↙₂⍆)
┌─╴test
  ⍤⤙≍ 34 🎗️ 2_3_4
  ⍤⤙≍ 14 🎗️ 1_1_10
└─╴

🐫 ← /+ ⊜(🎗️⊜⋕⊸≠@x)⊸≠@\n
┌─╴test
  ⍤⤙≍ 3812909 🐫 &fras "input"
└─╴

&p $"Part 1: _" 🐪 &fras "input"
&p $"Part 2: _" 🐫 &fras "input"
