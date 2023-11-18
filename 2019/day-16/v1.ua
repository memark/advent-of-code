# https://adventofcode.com/2019/day/16

PatternRow ← |1 ↻1 ▽∶ [0 1 0 ¯1] ×[1 1 1 1]
---
⍤∶≍, [1 0 ¯1 0] PatternRow 1
⍤∶≍, [0 0 1 1 1 0 0 0 ¯1 ¯1 ¯1 0] PatternRow 3
---

PatternMatrix ← |1 ∺(↯⊙PatternRow) ⊃(⊂[])(+1⇡)
---
⍤∶≍, [[1 0 ¯1 0] [0 1 1 0] [0 0 1 1] [0 0 0 1]] PatternMatrix 4
---

SolvePartOne! ← |1 (
  ∵parse
  ∶ PatternMatrix ⧻.
  ⍥(♭≡([◿10⌵/+]) ×, ↯⧻.) ^0
  ↙8 /$"__" ⊙;
)
---
⍤∶≍, "01029498" SolvePartOne!(4) "12345678"
⍤∶≍, "24176176" SolvePartOne!(100) "80871224585914546619083218645595"
⍤∶≍, "73745418" SolvePartOne!(100) "19617804207202209144916044189917"
⍤∶≍, "52432133" SolvePartOne!(100) "69317163492948606335995924319873"
⍤∶≍, "25131128" SolvePartOne!(100) &fras "input.txt"
---

&p $"Part 1: _" SolvePartOne!(100) &fras "input.txt"
