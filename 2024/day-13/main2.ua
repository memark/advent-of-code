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

# ok ny variant. prova alla kombinationer av 0..100 och 0..100 och se om det blir prize

8400_5400
[94_34 22_67]
+10⇡2
.⧅⋅⋅1 2
≡(
  ? # coeff tuples offsets
  ∘
  ?
  #   ⊙:
  ?
  #   /+⍜°⊟⊓(×94_34)(×22_67)

  #   /×=8400_5400
)
# ▽

# en ROWS-operation får väl kopiera av den data den behöver?
# dvs jag behöver inte kopiera/bevara underliggande data på stacken.
