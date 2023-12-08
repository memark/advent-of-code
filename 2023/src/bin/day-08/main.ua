Lib ← &i "../../lib.ua"
  SplitBy ← Lib "SplitBy"
  StartsWith ← Lib "StartsWith"
  Trim ← Lib "Trim"

Fst ← ⊡0
Snd ← ⊡1
Trd ← ⊡2

And ← ↧
Or ← ↥
Not ← -:1

# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

ParseNode ← ≡⊔↘1regex"(.*) = \\((.*), (.*)\\)" ⊔

# num instructions
GetInstr ← ⊡◿⧻,
---
⍤:≍, @R GetInstr 5 "LLR"
---

# Mid Nodes
FindNodeIndex ← |2 ⊗1∊ ⊙≡(↙1)
---
⍤:≍, 1 FindNodeIndex "DDD" ["AAA"_"BBB"_"CCC" "DDD"_"EEE"_"FFF" "GGG"_"HHH"_"III"]
⍤:≍, 1 FindNodeIndex "BBB" ["AAA"_"BBB"_"BBB" "BBB"_"AAA"_"ZZZ" "ZZZ"_"ZZZ"_"ZZZ"]
---

# Mid Nodes
FindLeaves ← |2 ↘1 ⊡ FindNodeIndex ⊙.
---
⍤:≍, "EEE"_"FFF" FindLeaves "DDD" ["AAA"_"BBB"_"CCC" "DDD"_"EEE"_"FFF" "GGG"_"HHH"_"III"]
---

# L/R Mid Nodes
FindLRLeaf ← |3 (
  ⊙FindLeaves
  ⍤$"\nExpected: L/R\nActual:   _":∊, "LR" .
  ⊗:"LR"
  ⊡
)
---
⍤:≍, "EEE" FindLRLeaf @L "DDD" ["AAA"_"BBB"_"CCC" "DDD"_"EEE"_"FFF" "GGG"_"HHH"_"III"]
# TODO: Fix
# ⍤:≍, "Only accepts L/R. Got: X" ⍣(FindLRLeaf)($"_") @X "DDD" ["AAA"_"BBB"_"CCC" "DDD"_"EEE"_"FFF" "GGG"_"HHH"_"III"]
---

FollowInstruction ← |4.4 (
  ⊙⊙,
  ⊙.
  ⊙(+1)
  ⊙⊙GetInstr
  ⊙:
  ⊙⊙⊙.
  ⊙:
  :
  ⊙:
  ⊙FindLRLeaf
  :
)

SolvePartOne ← (
  SplitBy@\n
  ⊃(≡ParseNode↘1|⊔⊢)
  "AAA" 0
  ⍢(FollowInstruction)(Not≍"ZZZ")
  ⊙;⊙;;
) # ---
# ⍤:≍, 2 SolvePartOne &fras "example_1"
# ⍤:≍, 6 SolvePartOne &fras "example_2"
# ⍤:≍, 16343 SolvePartOne &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"

# ---
# del två då. jag ska köra ett gäng simuleringar samtidigt.
# de som slutar med A ska gå till nåt som slutar med Z.
# sen ska jag kolla hur många omgångar det tar innan alla samtidigt slutar på Z.

# en optimering längre fram kan vara att skriva upp periodiciteten på resp start.

# jag behöver köra varje ghost för sej och sedan jämför hur långt de kommit

# jag behöver en funktion som tar alla fyra värdena och lägger i en array
# samt en funktion som extraherar alla värdena från en array till stacken

StackToArray ← ⊐[⊙⊙⊙∘]
ArrayToStack ← ⊃(⊔⊡0|⊔⊡1|⊔⊡2|⊔⊡3)

SolvePartTwo ← (
  SplitBy@\n
  ⊃(≡ParseNode↘1|⊔⊢)
  0
  ≡⊢▽≡(=@A⊢⇌⊢). , # get the ghosts
  # &s. ¤⊡0 # use only one
  ⊙¤ ⊙⊙¤ ⊙⊙⊙¤ # fix all lower values
  ≡StackToArray

  ⍢(
    ≡(StackToArray FollowInstruction ArrayToStack)
    (;|&s$"Iteration: _") =0◿100. ⊔⊡1⊡0. # print the iteration
  )(
    ≡(=@Z⊢⇌⊔⊢)
    (;|&s$"Ends: _") >0. /+. # print number of ending Zs
    -:1 /And
  )

  ArrayToStack ⊢
  ⊙⊙;⊙⊙;
  ;;
)
# SolvePartTwo &fras "example_3"
SolvePartTwo &fras "input"
