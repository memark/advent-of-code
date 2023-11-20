# https://adventofcode.com/2019/day/16

# Uiua 0.3.0

PatternRow ← |1 ↻1 ▽: [0 1 0 ¯1] ×[1 1 1 1]
---
⍤:≍, [1 0 ¯1 0] PatternRow 1
⍤:≍, [0 0 1 1 1 0 0 0 ¯1 ¯1 ¯1 0] PatternRow 3
---

PatternMatrix ← |1 ≡(↯¤ ⊙PatternRow) ⊃¤(+1⇡)
---
⍤:≍, [[1 0 ¯1 0] [0 1 1 0] [0 0 1 1] [0 0 0 1]] PatternMatrix 4
---

SolvePartOne! ← |1 (
  ∵parse
  : PatternMatrix ⧻.
  ⍥(♭≡([◿10⌵/+]) ×, ↯⧻.) ^0
  ↙8 /$"__" ⊙;
)
---
⍤:≍, "01029498" SolvePartOne!(4) "12345678"
⍤:≍, "24176176" SolvePartOne!(100) "80871224585914546619083218645595"
⍤:≍, "73745418" SolvePartOne!(100) "19617804207202209144916044189917"
⍤:≍, "52432133" SolvePartOne!(100) "69317163492948606335995924319873"
⍤:≍, "25131128" SolvePartOne!(100) &fras "input.txt"
---

# &p $"Part 1: _" SolvePartOne!(100) &fras "input.txt"

MakeTimesLonger ← ↯¤×⧻,
---
⍤:≍, [1 2 1 2 1 2] MakeTimesLonger 3 [1 2]
---

SolvePartTwo! ← |1 (
  ∵parse
  ∶ PatternMatrix ⧻.
  # ⍥(♭≡([◿10⌵/+]) ×, ↯⧻.) ^0
  ⍥(♭≡([◿10⌵/+]) ∺×,) ^0
  ↙8 /$"__" ⊙;
)

SolvePartTwo!(100) MakeTimesLonger 100 "80871224585914546619083218645595"
# SolvePartTwo!(100) "80871224585914546619083218645595"

# Nä, det gick ju alldeles för långsamt så klart. (100 MTL 100 tog 2,5 s.) (100 MTL 1000 tog 4,5 min.)
# Då är det dags att börja optimera.
# Säkerligen behöver jag inte skapa upp en hel matris för att kunna multiplicera. Det borde väl gå med en matris och en array?

# [[1 1 1] [2 2 2] [3 3 3]]
# [1 2 3]
# # ∺×
# ↯⧻.
