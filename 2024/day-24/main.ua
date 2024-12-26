# https://adventofcode.com/2024/day/24

# uiua v0.15.0-dev.1
# Experimental!

# a XOR b = (a OR b) - (a AND b)
🙅 ← °⋯-⬚0⊃×∨∩⋯
┌─╴🧪
  ⍤⤙≍ 7 🙅 4 3
  ⍤⤙≍ 12 🙅 10 6
  ⍤⤙≍ 0 🙅 1 1
└─╴

🗄 ← ⊃(
  $state map ≡⊓°□◇⋕ °⊟⍉ ≡↘₁regex"(...): (.+)"
| $gates map ≡⊃⊣(⊏0_2_1) ∵°□ ≡↘₁regex"(...) (AND|OR |XOR) (...) -> (...)" ⍜⊜∘≡⋅" OR "⊸⦷" OR"
)

GetKeys ← ⊙◌°map

# bool ? gateKey state
StateHasKey ← ≍⊙GetKeys

# bool ? gateKey state gates
StateHasGateKeyInputs ← /×≡⌟has ↙₂𝄐⌟get

# bit ? gateKey state gates
RunGate ← (
  ⊃(⊡₋₁|≡⌟get↙₂)𝄐⌟get
  ⨬(/×|/🙅|/∨) ⊗□:{"AND" "XOR" "OR "}
)

# state gates ? state gates
Iterate ← (
  GetKeys,
  ⍜⊙¤(▽¬◡≡StateHasKey)
  ⍜⊙∩¤(▽◡≡StateHasGateKeyInputs)
  ◡⍜⊙∩¤≡RunGate
  ∧insert:
)

# zs ? state
💤 ← get⍆ ▽≡⊸(=@z⊢) ⊸GetKeys

🐪 ← °⋯💤 ⊙◌ ⍥Iterate∞ 🗄

┌─╴🧪
  ⍤⤙≍ 4 🐪 &fras"example_1a.txt"
  ⍤⤙≍ 2024 🐪 &fras"example_1b.txt"
  ⍤⤙≍ 59619940979346 🐪 &fras"input.txt"
└─╴
