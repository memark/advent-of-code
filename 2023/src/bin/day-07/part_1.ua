Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"
  SplitByDelimiter ← Lib "SplitByDelimiter"
  StartsWith ← Lib "StartsWith"
  Trim ← Lib "Trim"

Fst ← ⊡0
Snd ← ⊡1
Trd ← ⊡2

And ← ↧
Or ← ↥

# SolvePartOne ← x SplitLines
# ---
# ⍤:≍, x SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
# ---

# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"
# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

# jag vill returnera en sträng som börjar med en siffra för huvudgrejen
# 7 femtal, 6 fyrtal, 5 kåk, 4 triss, 3 tvåpar, 2 par, högt kort
# plus ett tal för varje kort i serien

# OBS! Kan det ställa till det att dessa kollar delvis överlappar?

# skriv ihop dessa till en enda!
HasFiveOfAKind ← ≥5 /↥ ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasFiveOfAKind "55555"
⍤:≍, 0 HasFiveOfAKind "55455"
---

HasFourOfAKind ← ≥4 /↥ ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasFourOfAKind "55555"
⍤:≍, 1 HasFourOfAKind "55455"
⍤:≍, 0 HasFourOfAKind "55454"
---

# kan förfinas och ta bort 55555
HasFullHouse ← ≥5 /+ ▽≥2. ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasFullHouse "55555"
⍤:≍, 0 HasFullHouse "45555"
⍤:≍, 1 HasFullHouse "44555"
⍤:≍, 0 HasFullHouse "44556"
---

HasThreeOfAKind ← ≥3 /↥ ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasThreeOfAKind "55455"
⍤:≍, 1 HasThreeOfAKind "55454"
⍤:≍, 0 HasThreeOfAKind "55434"
---

HasTwoPairs ← ≥2⧻ ▽≥2. ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasTwoPairs "33444"
⍤:≍, 1 HasTwoPairs "33445"
⍤:≍, 0 HasTwoPairs "33456"
⍤:≍, 0 HasTwoPairs "34567"
---

HasOnePair ← ≥2 /↥ ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasOnePair "35455"
⍤:≍, 1 HasOnePair "33455"
⍤:≍, 0 HasOnePair "12345"
---

HasHighCard ← 1

GetCardValue ← +2⊗:"23456789TJQKA"
---
⍤:≍, 5 GetCardValue @5
⍤:≍, 13 GetCardValue @K
---

GetScoreForHand ← ⊐[
  ⊃(
    HasFiveOfAKind
  | HasFourOfAKind
  | HasFullHouse
  | HasThreeOfAKind
  | HasTwoPairs
  | HasOnePair
  | HasHighCard
  | GetCardValue ⊡0
  | GetCardValue ⊡1
  | GetCardValue ⊡2
  | GetCardValue ⊡3
  | GetCardValue ⊡4
  )]

SplitLines &fras ("example_1"|"input") 1
⊐≡(SplitByDelimiter@ )
⊃(⊐≡∘ Fst|⊐≡parse Snd)⍉
Hands ←
Bids ←

Bids
≡GetScoreForHand .Hands
⊐≡⊂        # join with hands
⊐≡⊂        # join with bids
⊐⊏⍏.       # sort by score
⊐≡⊂: +1⇡⧻. # join with ranks
≡(∵⊔↘2)    # keep bids and ranks
/+≡/×      # calc total winnings

# example 6440
# input 249726565
