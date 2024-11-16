# Uiua 0.13.0

ParseLines ← (
  ⊜□⊸≠@\n
  ≡(≡↘1 ◇regex "(.*)-(.*)\\[(.*)\\]")
  ↯∞_3
)

ComputeChecksum ← (
  ▽≠@-.
  ∩≡□ ⊕⊃⊢⧻ ⊛.
  :
  ≡°□
  ⇌⊕□
  ≡(□⊏⍏.°□)
  /◇⊂
  ≡°□
  ↙5
)

HasCorrectCheckSum ← (
  ⊃(ComputeChecksum °□⊡0|°□⊡2)
  /×=
)

PartOne ← (
  ParseLines
  ▽≡HasCorrectCheckSum.
  /+≡(⋕°□⊡1)
)

┌─╴test
  ⍤⤙≍ 1514 PartOne &fras "example.txt"
  ⍤⤙≍ 137896 PartOne &fras "input.txt"
└─╴

# plainText ? cipherText numKey
DecryptWithKey ← (
  ∵(=@-.
    ⨬( # 
      ⍜(-@a)(◿¯ +1-@a@z +)
    | @\s)
  )
)

┌─╴test
  ⍤⤙≍ "very encrypted name" DecryptWithKey "qzmt-zixmtkozy-ivhz" 343
└─╴

PartTwo ← (
  ParseLines
  ▽≡HasCorrectCheckSum.
  ≡(°[⊙⊙∘]
    ⊙⊙◌
    °□
    ⊙(⋕°□)
    ⊸DecryptWithKey
    ⌕"northpole".
    >0/+
    ⊙□
  )
  ⟜▽
  ⊙:
  ▽
  ⊙◌
  °¤
)

┌─╴test
  ⍤⤙≍ 501 PartTwo &fras "input.txt"
└─╴
