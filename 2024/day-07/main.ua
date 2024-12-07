# https://adventofcode.com/2024/day/7

# uiua 0.14.0-dev.7
# Experimental!

# res ? [expr]
EvalMany ← (
  &runc ⊂{"uiua" "eval" "--no-color"} □/◇$"_\n_"
  ⋅⊙◌ # keep stdout
  ⊜□⊸≠@\n
)
┌─╴test
  ⍤⤙≍ {"9" "6"} EvalMany{"* 3 3" "+ 3 3"}
└─╴

# permutations ? length chars
Permute ← ⧅(1◌◌)
┌─╴test
  ⍤⤙≍ ["++" "+*" "*+" "**"] Permute 2 "+*"
└─╴

# combs ? ops nums
Combine ← (
  ⊙≡(□$"_")
  ⊙¤
  ≡(/◇$"_ _"⊂↘1)
)

# bool ? testvalue nums
TryEquation ← /∨= ⍜:(≡⋕EvalMany Combine Permute :"+*" ⊸⧻)
┌─╴test
  ⍤⤙≍ 1 TryEquation 3267 [81 40 27]
  ⍤⤙≍ 0 TryEquation 3267 [80 40 27]
  ⍤⤙≍ 1 TryEquation 292 [11 6 16 20]
└─╴

🐪 ← (
  ⊜(♭₁regex "(\\d*): (.*)")⊸≠@\n
  ≡⊃(□◇⊜⋕⊸≠@\s⊡₂|⋕⊡₁)
  ≡°⊟wait≡pool(
    &pf@> :⟜TryEquation:°□ &pf@<
  )
  &p""
  /+▽)

┌─╴test
  ⍤⤙≍ 3749 🐪 &fras"example.txt"
└─╴

# ⧅(1◌◌) :"+*" 3, är detta bra?
