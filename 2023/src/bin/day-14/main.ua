# Uiua 0.6.1

Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"

F ← |1 (
  ⊚⌕".O".
  ⊃(∘|⍜(⊏|↯:@O⧻))
  ⍜(⊏|↯:@.⧻)+1
)

CollapseRow ← ; ⍢(F|⊙.=0≍,:) F.

TiltNorth ← ⍜(⍉)≡CollapseRow
TiltWest ← ≡CollapseRow
TiltSouth ← ⍜(⍉⇌)≡CollapseRow
TiltEast ← ≡⍜(⇌|CollapseRow)

CalculateLoad ← (
  =@O
  ×⇌+1⇡⧻.
  /+♭
)

SolvePartOne ← (
  SplitBy@\n
  TiltNorth
  CalculateLoad
)
---
⍤:≍, 136 SolvePartOne &fras "example_1"
⍤:≍, 106517 SolvePartOne &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"

SolvePartTwo ← (
  SplitBy@\n
  ⍥(
    TiltNorth
    TiltWest
    TiltSouth
    TiltEast
    # &pf@ &pf CalculateLoad.
  ) 1000
  CalculateLoad
)

# SolvePartTwo &fras "input"

# jag har hittat en period som ser ut så här:
# 79708 79714 79711 79708 79718 79724 79727 79731 79718 79723 79730 79736 79733 79743 79742 79734 79723 79716
# perioden är 18
# den startar efter 125 gånger
# "mod18 -125 1000000000" => 11
# 12e => 79723, rätt!
