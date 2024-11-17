# Uiua 0.13.0

~ "../lib.ua" ~ LeastCommonLetter MostCommonLetter

SolveWithSort! ← (
  ⊜∘⊸≠@\n
  ⍜⍉(
    ≡(⊕□ ⊸⊛ ⊏⊸⍏)
    ⊸∵(⧻°□)
    ≡(⊏^0)
  )
  ≡◇⊢ ⊢
)

PartOne ← SolveWithSort!⍖

┌─╴test
  ⍤⤙≍ "easter" PartOne &fras "example.txt"
  ⍤⤙≍ "tzstqsua" PartOne &fras "input.txt"
└─╴

PartTwo ← SolveWithSort!(⍏)

┌─╴test
  ⍤⤙≍ "advent" PartTwo &fras "example.txt"
  ⍤⤙≍ "myregdnr" PartTwo &fras "input.txt"
└─╴
