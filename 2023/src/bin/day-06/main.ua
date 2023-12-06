Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"

GetRacesFromFilePartOne ← ⍉⊐≡(≡parse regex"\\d+") SplitLines

# length
GetAltsForRaceLength ← |1 ×-,:⇡+1.
# length record
GetNumOfWinsForRace ← |2 /+ < GetAltsForRaceLength

SolvePartOne ← /× ≡(GetNumOfWinsForRace ⊃(⊢|⊢⇌)) GetRacesFromFilePartOne
---
⍤:≍, 288 SolvePartOne &fras "example_1"
⍤:≍, 503424 SolvePartOne &fras "input"
---

GetRacesFromFilePartTwo ← ¤⊐≡(parse /⊂ regex"\\d+") SplitLines

SolvePartTwo ← /× ≡(GetNumOfWinsForRace ⊃(⊢|⊢⇌)) GetRacesFromFilePartTwo
---
⍤:≍, 71503 SolvePartTwo &fras "example_1"
⍤:≍, 32607562 SolvePartTwo &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
