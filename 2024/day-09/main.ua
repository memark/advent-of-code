# https://adventofcode.com/2024/day/9

# uiua v0.14.0-dev.7
# Experimental!

Intersperse ← ↘₁♭≡⊂

# array ? old new array
Replace ← ⍜▽⋅∘=, ⊙:
┌─╴🧪
  ⍤⤙≍ 1_3_8_3_2 Replace 5 3 [1 5 8 5 2]
└─╴

# TODO: neg1 blir alltid ¯ 1 (med mellanslag)
# TODO: Bryt ut en riktig ReplaceSubstring här. Dags för årets lib.ua nu.

# ? array
Format ← ⍜⊜∘≡⋅"." ⊸⦷ "-1" /◇⊂≡(□°⋕)
Print  ← &p Format

IteratePartOne ← (
  ⊸⊃((⊣⊚≠¯1)|(⊢⊚=¯1))
  ⨬(◌◌|⍜⊏⇌⊂)◡<
)

Checksum ← /+×⇡⊸⧻ ▽⊸≠¯1

🐪 ← (
  ≡⋕ ▽⊸≠@\n
  ▽: Intersperse¯1 ⇡⊸(⌈÷₂⧻)
  ⍥IteratePartOne∞
  Checksum
)
┌─╴🧪
  ⍤⤙≍ 1928 🐪 &fras"example.txt"
  ⍤⤙≍ 6386640365805 🐪 &fras"input.txt"
└─╴

# map ? id map
TryCompact ← (
  ⊃(⊙: -1=¯1⇡ ⊸⧻ ⊚⊸⌕
    ⊚>₀⊸⦷
    ⬚¯1↙⧻:⊙:
    $freeSpaceIds
  | $idsToMove ⊚⌕ ⊙∘
  )
  ⊙:
  ◡⊃(
    (/×<)  # ¯1 means wrong order
  | (=¯1⊢) # ¯1 means not enough space
  )
  ∨ # one ¯1 is enough to abort
  ⨬(⍜⊏⇌ ⊂|◌◌)
)

🐫 ← (
  ≡⋕ ▽⊸≠@\n
  ▽: Intersperse¯1 ⇡⊸(⌈÷₂⧻)
  $fileIds ⇌⍆◴⊸▽⊸≠¯1
  ∧TryCompact
  ⇡⊸⧻
  ∩▽,⊙(⊸≠¯1)
  /+×
)
┌─╴test
  ⍤⤙≍ 2858 🐫 &fras"example.txt"
  ⍤⤙≍ 6423258376982 🐫 &fras"input.txt"
└─╴
