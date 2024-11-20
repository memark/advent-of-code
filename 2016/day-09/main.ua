# Uiua 0.13.0
# Experimental!

Decompress ← ()

# incr str ? len times data
UtilThree ← (
  ⤚×    # product len ? len times
  ⊙(    # 
    ⊃↙↘ # data₁ data₂ ? len data
  )
  ⊙(⊂♭↯ ⊂1) .
)

# index str ? index str
DecompressOnce ← (
  ⊃(↙
  | ⊙↘.
    ⊙⍣(
      ⍩(UtilThree) ⋕⊙⋕ °$"(_x_)_"
    | ⍩(⊙UtilThree
        +⊙⊸⧻: # newIncr prefix ? prefix incr
        ⊙⊂
      ) ⊙(⋕⊙⋕) °$"_(_x_)_"
    | 0 °$"_" ⊙◌)
    +
  )
  ⊙⊂:
)

# len ? str
SolvePartOne ← ⧻◌ ⍥DecompressOnce∞ 0

┌─╴test
  ⍤⤙≍ 6 SolvePartOne "ADVENT"
  ⍤⤙≍ 7 SolvePartOne "A(1x5)BC"
  ⍤⤙≍ 9 SolvePartOne "(3x3)XYZ"
  ⍤⤙≍ 11 SolvePartOne "A(2x2)BCD(2x2)EFG"
  ⍤⤙≍ 6 SolvePartOne "(6x1)(1x3)A"
  ⍤⤙≍ 18 SolvePartOne "X(8x2)(3x3)ABCY"
  ⍤⤙≍ 123908 SolvePartOne &fras "input.txt"
└─╴
