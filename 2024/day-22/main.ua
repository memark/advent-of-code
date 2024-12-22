# https://adventofcode.com/2024/day/22

# uiua v0.15.0-dev.1
# Experimental!

# a XOR b = (a OR b) - (a AND b)
🙅 ← °⋯-⬚0⊃×∨∩⋯
┌─╴🧪
  ⍤⤙≍ 7 🙅 4 3
  ⍤⤙≍ 12 🙅 10 6
└─╴

🤞 ← 🙅
⍤⤙≍ 37 🤞 42 15

✂️ ← ◿₁₆₇₇₇₂₁₆
⍤⤙≍ 16113920 ✂️ 100000000

🥇 ← ✂️🤞⊸×₆₄
🥈 ← ✂️🤞⌊⊸÷₃₂
🥉 ← ✂️🤞⊸×₂₀₄₈

🎖 ← 🥉🥈🥇

⍤⤙≍ 8685429 ⍥₂₀₀₀🎖 1
⍤⤙≍ 4700978 ⍥₂₀₀₀🎖 10
⍤⤙≍ 15273692 ⍥₂₀₀₀🎖 100
⍤⤙≍ 8667524 ⍥₂₀₀₀🎖 2024

🐪 ← /+wait≡pool⍥₂₀₀₀🎖 ⊜⋕⊸≠@\n
┌─╴🧪
  ⍤⤙≍ 37327623 🐪 &fras"example_1a.txt"
  #   ⍤⤙≍ 16299144133 🐪 &fras"input.txt"
└─╴

# &fras"example_1a.txt"
# ⊜⋕⊸≠@\n
# 123
# ?₁⇌[⍥⊸🎖9] # numbers
# ?₁≡◿₁₀    # prices
# ?₁⊸⧈-     # changes
# ?₁⊸⧈∘4    # sequences

# ⊸(≡/∨⦷) [¯1 ¯1 0 2]

# ta fram alla värdena i egna arrayer enligt ovan
# när det är flera starttal, får de väl ligga som subarrayer
# bättre att ha en viss sorts värden i en egen array,
# och dela upp resp array i subarrays baserat på starttal

# ta fram alla möjliga sekvenser (finns för minst ett starttal)
# slå ihop för alla starttal
# glöm inte att ta dedup på dessa

# gå igenom sekvenserna en efter en,
# hitta matchningar,
# räkna ut score,
# summera

# sequences changes prices ? initialNumber
ProcessOneInitial ← memo(
  ⇌[⍥⊸🎖2000]
  ≡◿₁₀ # prices
  ⊸⧈-  # changes
  ⊸⧈∘4 # sequences
)

# ProcessOneInitial 123

# bananas ? sequence changes prices
EvaluateSequence ← memo(
  ⌕
  ≡↻₋₄
  ≡(□⊚)
  ≡(⨬(0|⊡⊢) ±⊸⧻ °□)
)

# &fras"example_2a.txt"
&fras"input.txt"
⊜⋕⊸≠@\n
≡(&pf@. ProcessOneInitial)
◴⍆♭₋₁
dump△
⊙¤⊙⊙¤
wait≡pool(&pf@. EvaluateSequence)
≡/+
/↥
# 1896, correct
