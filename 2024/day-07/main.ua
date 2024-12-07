# https://adventofcode.com/2024/day/7

# uiua 0.15.0-dev.1
# Experimental!

🗄 ← (
  ⊜□⊸≠@\n
  ≡◇(↘₁♭₁regex "(\\d*): (.*)")
  ≡⊃(□◇⊜⋕⊸≠@\s⊣|⋕⊢)
)

# permutations ? length chars
Permute ← ⧅(1◌◌)
┌─╴🧪
  ⍤⤙≍ ["++" "+*" "*+" "**"] Permute 2 "+*"
└─╴

# ba ? a b
Join ← ⋕⊂:∩°⋕
┌─╴🧪
  ⍤⤙≍ 3412 Join 12 34
└─╴

# res ? nums ops
TryPerm ← (
  °⊂ # extract first num
  :⊙::
  ∧(⨬(×|+|Join) ⊗:"+*|")
)

# bool ? ops nums testvalue
EvalEquation ← /∨= ≡⌞TryPerm :Permute : ⊙(-₁⊸⧻)

Solve! ← /+▽ °⊟⍉wait≡pool⍜:⟜◇(EvalEquation ^0 :)

🐪 ← Solve!"+*" 🗄
┌─╴🧪
  ⍤⤙≍ 3749 🐪 &fras"example.txt"
  ⍤⤙≍ 3119088655389 🐪 &fras"input.txt"
└─╴

🐫 ← Solve!"+*|" 🗄
┌─╴🧪
  ⍤⤙≍ 11387 🐫 &fras"example.txt"
  ⍤⤙≍ 264184041398847 🐫 &fras"input.txt"
└─╴
