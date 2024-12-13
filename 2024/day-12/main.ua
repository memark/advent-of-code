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

# ta först fram regionerna
# starta på valfri A, se vilka övriga A jag kan nå
# kanske genom dijkstra mellan alla par av A?
# eller genom rekursiv flood fill? det vore ju nyttigt eftersom jag aldrig gjort det innan.

# räkna sen ut aria
# räkna sen ut omkrets
# räkna sen ut pris

# bool ? coordPair matrix
PathsBetweenCoordPair ← (
  °⊟
  ⊙◌path(
    ⊙⊙.               # always retain matrix at bottom of stack
    ⊃(⊡⊙⋅∘|+N¤|¤△⋅⋅∘) # crop neis shape ? curr goal matrix
    ⊙(▽⤚≡🏰)           # crop withinBoundsNeis ? crop neis shape
    ⊃(▽=⊙⤚⊡)          # validNeis ? crop withinBoundsNeis matrix
  )(≍
  )
)

# ⊚⊸⌕@C . ⊜∘⊸≠@\n &fras"example_1a.txt"
⊚⊸⌕@X . ⊜∘⊸≠@\n &fras"example_1b.txt"

⧅≤ 2

⤚≡(⊃⧻□PathsBetweenCoordPair) ⊙¤

# ok, nu har jag fått fram vilka crops som hänger ihop.
# hur använder jag det för att få fram vilka regioner som finns?
# jag måste på nåt sätt titta på alla paths samtidigt för att täcka upp alla rutor
# det är väl iofs inte nåt problem, bara att slå ihop alla patharrayerna
# men VILKA är det jag ska slå ihop?

# parvägar som delar någon enda koordinat kan slås ihop

# gruppera koordinatparen baserat på lägsta koordinaten.
# slå ihop alla vägar som hittades.
# dedup på dessa sammanslagna borde ge alla regioner.
