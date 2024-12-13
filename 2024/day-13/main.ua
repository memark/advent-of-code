# https://adventofcode.com/2024/day/13

# uiua v0.14.0-dev.7
# Experimental!

~ "../day-10/lib.ua" ~ 🏰

&fras"example.txt"
⊜□⊸≠@\n
↯∞_3
# det gick ändå smidigt!
◌

# nu är min idé att faktiskt använda pathfiding även här.
# jag söker mej från 0_0 till y_x för priset
# neighbors blir curr +A resp +B
# cost blir den token cost som finns

# A kostar 3
# B kostar 1

# A B P
# 94_34 22_67 8400_5400
# NC ← [3 1] [94_34 22_67]
C ← [3 1]
N ← [94_34 22_67]

# 0_0 8400_5400
# ⊢path(
#   #   ⊙.
#   #   &sl1
#   ?
#   C+N
#   ?
#   #   ▽?≡(/×≥)
#   ▽:⊙⟜▽ ⊙◡≡(/×≥)
#   ?
#   ⊙⊙◌
# )(≍?)

# ┌╴? main.ua:37:3
# │╴╴╴╶╶╶
# ├╴[8400 5400]
# │ ╭─           
# ├╴╷ 5494 5434  
# │             ╯
# ├╴[3]
# └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴
# Error: Number of nodes 2 does not match number of costs 1

# det här fattar jag inte. hur ska man annars returnera noder och cost när man filtrerat?

# man kan aldrig gå tillbaka, så är vi över goal får N filtreras bort.
# +N
# +[0_10000 0_0]
# C
# ▽:⊙⟜▽ ⊙◡≡(/×≥)

# jag behöver ta cost och neis och filtrera dem ihop

# # bool ? coordPair matrix
# CoordPairBelongsToSameRegion ← (
#   °⊟
#   ?
#   ⊙◌path(
#     ⊙⊙. # always keep matrix at bottom of stack

#     #   ⊃(△|∘)
#     #   #   +N¤.
#     #   #   ⊙⊙(◡⊡)
#     #   #   ⊙:
#     #   #   ◌◌◌
#     ⊃(⊡⊙⋅∘|+N¤|¤△⋅⋅∘) # crop neis shape ? curr goal matrix
#     ⊙(▽⤚≡🏰)           # crop withinBoundsNeis ? crop neis shape
#     ⊃(▽=⊙⤚⊡)          # validNeis ? crop withinBoundsNeis matrix
#   )(≍
#   )
#   ⧻
# )

8400_5400
[8469_8409
 8337_8382]
[3 1]

⊙◡≡(/×≥)
# ▽:⊙⟜▽
