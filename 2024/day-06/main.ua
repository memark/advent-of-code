# https://adventofcode.com/2024/day/6

# uiua v0.14.0-dev.7
# Experimental!

G‼! ← |3 (
  $delta^1
  $new⊸+
  ◡𝄐⊡
  $coll=@#
  ⨬(         # no collision
    ⊙⍜⊡(@X◌) # - mark visited
    ⍜⊡(^0◌)  # - set new pos
  |          # collision
    ⊙⍜⊡(^2◌) # - turn
    ◌        # - discard new pos
  )
)

F ← |1 (
  $matrix
  $curr♭⊸(⊚∈"><^v")
  $char◡⊡
  ⍣(°$"^"
    ⍩(G‼!@^[¯1 0]@>)
  | °$">"
    ⍩(G‼!@>[0 1]@v)
  | °$"v"
    ⍩(G‼!@v[1 0]@<)
  | °$"<"
    ⍩(G‼!@<[0 ¯1]@^)
  )
)

🐪 ← (
  ⍥(⍣F)∞
  /+♭∈"X><^v"
)

┌─╴test
  ⍤⤙≍ 41 🐪 ≡°□ °/$"_\n_" &fras$ example.txt
  ⍤⤙≍ 5067 🐪 ≡°□ ↘¯1 °/$"_\n_" &fras$ input.txt
└─╴
