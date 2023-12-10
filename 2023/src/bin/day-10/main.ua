# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

FindStart ← ⊢⊚⌕@S

# 0 upp - 1 höger - 2 ner - 3 vänster
# char -> []/[dir]/[dir1 dir2]/[...]
GetDirsForChar ← |1 (0_2|1_3|0_1|0_3|2_3|1_2|[]|0_1_2_3) ⊗: "|-LJ7F.S"

# dir -> [dy dx]
GetDeltasForDir ← |1 (¯1_0|0_1|1_0|0_¯1)

# k m -> bool
WithinBounds ← |2 ↧⊃(/↧<:⊙△|/↧≥:⊙0_0)

# k m -> []/[k]/[k1 k2]/[...]
FindNeighbors ← |2 (
  ⊃(+⊃(¤∘|∵GetDeltasForDir GetDirsForChar ⊡)|¤⋅∘)
  ▽⊃(≡WithinBounds|∘)
)

# k m -> []/[k]/[k1 k2]
FindNeighborsPointingBack ← (
  ⊃(FindNeighbors|¤:¤⊙∘) # -> n m k
  ⊃(≡(□FindNeighbors)|∘) # -> nn n k
  :⊙:                    # -> k nn n
  ▽≡(∊⊙°□)               # -> nn
)

# ks m -> k
FindNextNeighbor ← (
  ⊃(FindNeighbors ⊡¯1|⊡¯2)
  ♭▽≡(≠1≍) ¤⊙.:
)

# ks -> bool
AllUnique ← =⊃(⧻⊝|⧻)

SolvePartOne ← (
  SplitBy@\n
  FindStart .
  ⊂:¤⊢⊃(FindNeighborsPointingBack|⊙∘)
  ⍢(
    ⊂:⊃(FindNextNeighbor|⊙∘)
  | AllUnique
  )
  ⊙;
  ⍜⇌(↘1)
  ÷2⧻
)
&p $"Part 1: _" SolvePartOne &fras "input"

---
⍤:≍, 0 WithinBounds ¯1_0 ↯5_5 0
⍤:≍, 0 WithinBounds 0_¯1 ↯5_5 0
⍤:≍, 1 WithinBounds 0_0 ↯5_5 0
⍤:≍, 1 WithinBounds 1_3 ↯5_5 0
⍤:≍, 1 WithinBounds 4_4 ↯5_5 0
⍤:≍, 0 WithinBounds 4_5 ↯5_5 0
⍤:≍, 0 WithinBounds 5_4 ↯5_5 0
---

---
⍤:≍, 4 SolvePartOne &fras "example_2"
⍤:≍, 4 SolvePartOne &fras "example_3"
⍤:≍, 8 SolvePartOne &fras "example_4"
⍤:≍, 8 SolvePartOne &fras "example_5"
⍤:≍, 6860 SolvePartOne &fras "input"
---
