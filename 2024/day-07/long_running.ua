# https://adventofcode.com/2024/day/7

# uiua 0.14.0-dev.7
# Experimental!

# res ? [expr]
EvalMany ← (
  □/◇$"_\n_"
  #   &runc ⊂{"uiua" "eval" "--no-color"}
  □⟜&fwa $"/Volumes/RAMDisk/long-running/temp_.uax"tag °□
  &runc ⊂{"uiua" "run" "--no-color" "--no-format"}
  ⋅⊙◌ # keep stdout
  ⊜□⊸≠@\n
  #   ["1"]◌ # hack
)
┌─╴test
  ⍤⤙≍ {"9" "6"} EvalMany{"* 3 3" "+ 3 3"}
└─╴

# utan filer tog del ett
# 280,02s user 11,40s system 1441% cpu 20,222 total

# med filer tog del ett
# 4712,38s user 23,06s system 1424% cpu 5:32,36 total

# med filer tog del ett (utan format)
# 701,52s user 13,03s system 1426% cpu 50,088 total

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
  #   ⍤⤙≍ 1 TryEquation 3267 [81 40 27]
  #   ⍤⤙≍ 0 TryEquation 3267 [80 40 27]
  #   ⍤⤙≍ 1 TryEquation 292 [11 6 16 20]
└─╴

🐪 ← (
  ⊜(♭₁regex "(\\d*): (.*)")⊸≠@\n
  ≡⊃(□◇⊜⋕⊸≠@\s⊡₂|⋕⊡₁)
  ≡°⊟wait≡pool(
    &pf@> :⟜TryEquation:°□ &pf@<
  )
  &p""
  /+▽
)
┌─╴test
  #   ⍤⤙≍ 3749 🐪 &fras"example.txt"
└─╴

# ab ? a b
C ← ⋕⊂∩°⋕
┌─╴test
  ⍤⤙≍ 1234 C 12 34
└─╴

# bool ? testvalue nums
TryEquationNew ← /∨= ⍜:(
  Combine Permute :"C+*" ⊸⧻
  ≡(□⍜⊜∘≡⋅"⋕⊂∩°⋕" ⊸⦷"C") # handle ||
  ≡⋕?₁EvalMany
)
┌─╴test
  #   ⍤⤙≍ 1 TryEquationNew 3267 [81 40 27]
  #   ⍤⤙≍ 0 TryEquationNew 3267 [80 40 27]
  #   ⍤⤙≍ 1 TryEquationNew 292 [11 6 16 20]
└─╴

# ⧅(1◌◌) :"+*" 3, är detta bra?

🐫 ← (
  ⊜(♭₁regex "(\\d*): (.*)")⊸≠@\n
  ≡⊃(□◇⊜⋕⊸≠@\s⊡₂|⋕⊡₁)
  ≡°⊟wait≡pool(
    &pf@> :⟜TryEquationNew:°□ &pf@<
  )
  &p""
  /+▽
)
# 🐫 &fras"example.txt"
# 🐫 &fras"input.txt"

# ⟜&fwa °⋕tag "+ 3 3"
# &fras

# 🐪 &fras"example.txt"
🐫 &fras"input.txt"
