# https://adventofcode.com/2024/day/4

# Uiua 0.14.0-dev.7
# Experimental!

Z ← =¯1⇡
Q ← +@_=¯1⇡
R ← ≡(↻) ⇡⊸⧻

Pad ← (
  ⊂Q ⊡1△.
  ⊂ ⟜[Q ⊡1△]
  ⍉
  ⊂Q ⊡1△.
  ⊂⟜[Q ⊡1△]
  °⍉
)

🐪 ← (
  ⊜∘⊸≠@\n
  Pad
  ⇌[⊃(≡⇌⍉R⇌∘|⍉R⇌∘|≡⇌⍉R∘|⍉R∘|⍉⇌∘|⍉∘|≡⇌∘|∘)]
  /+/+/+≡≡(⌕"XMAS")
)
┌─╴🧪
  ⍤⤙≍ 18 🐪 &fras"example_2.txt"
  ⍤⤙≍ 2401 🐪 &fras"input.txt"
└─╴

✖️ ← ×⊃(/∨≡≍"MMSS"_"MSMS"_"SMSM"_"SSMM"¤⊡⧅⋅⋅1 2 0_2|=@A ⊡1_1)
🐫  ← /+♭⧈✖️ 3_3 ⊜∘⊸≠@\n
┌─╴🧪
  ⍤⤙≍ 9 🐫 &fras"example_2.txt"
  ⍤⤙≍ 1822 🐫 &fras"input.txt"
└─╴
