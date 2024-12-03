# Uiua 0.14.0-dev.5
# Experimental!

# 🐪 ← /+≡/× ≡(≡⋕↘₁)regex$ mul\((\d{1,3}),(\d{1,3})\)

🗄️ ← ♭regex$ mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)

# prod state ? instr state
🔂 ← ⍣(
  ⍩(0 1◌) °$"do()"
| ⍩(0 0◌) °$"don't()"
| ⍩(× ∩⋕) °$"mul(_,_)"
)

🐪 ← ◌∧(⊙+:🔂) :1 :0 🗄️
🐫 ← ◌∧(⊙+:⊸×🔂) :1 :0 🗄️

┌─╴test
  ⍤⤙≍ 161 🐪 &fras"example_1.txt"
  ⍤⤙≍ 175015740 🐪 &fras"input.txt"
  ⍤⤙≍ 48 🐫 &fras"example_2.txt"
  ⍤⤙≍ 112272912 🐫 &fras"input.txt"
└─╴

$PartOne 🐪 &fras"input.txt"
$PartTwo 🐫 &fras"input.txt"
