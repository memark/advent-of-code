# https://adventofcode.com/2024/day/8

# uiua 0.14.0-dev.7
# Experimental!

# ? matrix
🔤 ← $chars ▽⊸≠@. ▽⊸≠@# ◴/⊂

# ? char matrix
🗺 ← $coords ⊚=

# ? antennas
👥 ← $pairs ⧅<2

# ? pair
〰️ ← $antinodes /⊂ ≡⍜°⊟⊃(+⊸-:|+⊸-)

# ? coord coordmax
🏰 ← $isinside ×⊃(/×<:|/×≥₀)
┌─╴🧪
  ⍤⤙≍ 0 🏰 ¯1_¯1 10_10
  ⍤⤙≍ 1 🏰 5_5 10_10
  ⍤⤙≍ 1 🏰 0_0 10_10
  ⍤⤙≍ 0 🏰 10_10 10_10
└─╴

🐪 ← (
  ⊜∘⊸≠@\n
  ⊃(/◇⊂≡(□〰️👥🗺)⊙¤⊸🔤|△)
  ▽⤚≡🏰⊙¤
  ⧻◴
)
┌─╴🧪
  ⍤⤙≍ 14 🐪 &fras"example.txt"
  ⍤⤙≍ 240 🐪 &fras"input.txt"
└─╴

🔧 ← ⍜▽⋅∞⊸=¯∞

# ? antennapair coordmax
➰ ← (
  ⊚↯:1:
  ⊸(:¤):
  ≡(/=🔧/÷-¤)
  ▽
)
┌─╴🧪
  ⍤⤙≍ [0_0 1_3 2_6 3_9] ➰ [0_0 1_3] 10_10
  ⍤⤙≍ [0_11 1_8 2_5 3_2] ➰ [1_8 2_5] 12_12
└─╴

🐫 ← (
  ⊜∘⊸≠@\n
  ⊃(/◇⊂ ≡(□👥🗺)⊙¤⊸🔤|△)
  /◇⊂≡(□➰)⊙¤
  ⧻◴
)
┌─╴🧪
  ⍤⤙≍ 34 🐫 &fras"example.txt"
  ⍤⤙≍ 955 🐫 &fras"input.txt"
└─╴
