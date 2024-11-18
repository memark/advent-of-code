AssertString ← ⍤"expected string" ≍1 type

MostCommonLetter ← (
  ⊸AssertString
  ⊢⊏⍖ ⊕⊃⧻⊢ ⊸⊛
)

┌─╴test
  ⍤⤙≍ @b MostCommonLetter "abbc"
└─╴

LeastCommonLetter ← (
  ⊸AssertString
  ⊢⊏⍏ ⊕⊃⧻⊢ ⊸⊛
)

┌─╴test
  ⍤⤙≍ @b LeastCommonLetter "aaabbccc"
└─╴

# matrix ? x y
CreateZeroMatrix ← ↯⊙⊚:

# matrix ? x y
CreateOneMatrix ← =0↯⊙⊚:

CreateTextFromMatrix ← (
  +@\s×-@\s@*
  /⊂≡⊂:¤@\n
)
