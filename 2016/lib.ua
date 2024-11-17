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
