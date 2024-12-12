# https://adventofcode.com/2024/day/12

# uiua 0.14.0-dev.7
# Experimental!

~ "../day-10/lib.ua" ~ 👨‍💻‼
~ "../day-10/lib.ua" ~ 🏰
N ← ⊂⟜¯⋯1_2

# 🏘 ← (
#   +⊸[∩¯,,⇌.⇡2]¤
#   ▽⊸≡(🏰:△Grid)
# )

# HeadsAndFeet ← ♭₋₁⊞⊟:⊃(⊚=₉|⊚=₀)

# 📐 ← ⍤⤙≍⊙△

# # validNeighbors ? current
# 🔜 ← (
#   ⊸📐[2]
#   ⊸🏘
#   ⊙(+1⊡:Grid)     # ta fram värdet+1
#   ▽:⊙=:⊸≡(⊡:Grid) # kolla dess värden
# )

# Solve! ← /+^0 ≡(⧻ 👨‍💻‼🔜 ≍ °⊟₂) HeadsAndFeet

&fras"example_1a.txt"
⊜∘⊸≠@\n

# ta först fram regionerna
# starta på valfri A, se vilka övriga A jag kan nå
# kanske genom dijkstra mellan alla par av A?
# eller genom rekursiv flood fill? det vore ju nyttigt eftersom jag aldrig gjort det innan.

# räkna sen ut aria
# räkna sen ut omkrets
# räkna sen ut pris

# bool ? coordPair matrix
CoordPairBelongsToSameRegion ← (
  °⊟
  ?
  ⊙◌path(
    ⊙⊙. # always keep matrix at bottom of stack

    #   ⊃(△|∘)
    #   #   +N¤.
    #   #   ⊙⊙(◡⊡)
    #   #   ⊙:
    #   #   ◌◌◌
    ⊃(⊡⊙⋅∘|+N¤|¤△⋅⋅∘) # crop neis shape ? curr goal matrix
    ⊙(▽⤚≡🏰)           # crop withinBoundsNeis ? crop neis shape
    ⊃(▽=⊙⤚⊡)          # validNeis ? crop withinBoundsNeis matrix
  )(≍
  )
  ⧻
)

⊚⊸⌕@C
⧅<2

# hack
?
# CoordPairBelongsToSameRegion ⊡2

▽⤚≡CoordPairBelongsToSameRegion⊙¤

# kanske lättare att ta med hela listan av C in i funktionen?
# och bara kolla om grannarna tillhör den listan?
# det kan jag iofs refaktorera senare
