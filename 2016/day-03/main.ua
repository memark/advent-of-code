# Uiua 0.13.0

SolveWithTransform! ← (
  ⊜∘⊸≠@\n
  ≡⊜⋕⊸≠@\s
  ^0
  /+ ≡(<+ °[⊙⊙∘] ⊏⍏.)
)

PartOne ← SolveWithTransform!∘

┌─╴test
  ⍤ ⤙=983 PartOne &fras "input.txt"
└─╴

PartTwo ← SolveWithTransform!⍜(↯∞_3_3)°⍉

┌─╴test
  ⍤ ⤙=1836 PartTwo &fras "input.txt"
└─╴
