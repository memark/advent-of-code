Lib ← &i "../../lib.ua"
  SplitByDelimiter ← Lib "SplitByDelimiter"

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

# &p $"Part 1: _" SolvePartOne &fras "input"

# ---

HasFiveOfAKind ← ≥5 /↥ ⊐≡⧻ ⊐⊜∘ . ⊏⍏. -@\0
---
⍤:≍, 1 HasFiveOfAKind "55555"
⍤:≍, 0 HasFiveOfAKind "55455"
⍤:≍, 0 HasFiveOfAKind ▽≠@J. "JJJJJ"
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

HasHighCard ← 1;

HasJoker ← ∊@J
---
⍤:≍, 0 HasJoker "34567"
⍤:≍, 1 HasJoker "34J67"
---

NumJokers ← /+ ∊:@J
---
⍤:≍, 0 NumJokers "33333"
⍤:≍, 2 NumJokers "333JJ"
⍤:≍, 4 NumJokers "J3JJJ"
---

HasFiveOfAKindWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasFiveOfAKind)
  | And⊃(=1NumJokers|HasFourOfAKind ▽≠@J.)
  | And⊃(=2NumJokers|HasThreeOfAKind ▽≠@J.)
  | And⊃(=3NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasFiveOfAKindWithJokers "33333"
⍤:≍, 1 HasFiveOfAKindWithJokers "3333J"
⍤:≍, 1 HasFiveOfAKindWithJokers "333JJ"
⍤:≍, 1 HasFiveOfAKindWithJokers "33JJJ"
⍤:≍, 1 HasFiveOfAKindWithJokers "3JJJJ"
⍤:≍, 1 HasFiveOfAKindWithJokers "JJJJJ"
⍤:≍, 0 HasFiveOfAKindWithJokers "34JJJ"
---

HasFourOfAKindWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasFourOfAKind ▽≠@J.)
  | And⊃(=1NumJokers|HasThreeOfAKind ▽≠@J.)
  | And⊃(=2NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=3NumJokers|1)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasFourOfAKindWithJokers "33333"
⍤:≍, 1 HasFourOfAKindWithJokers "3333J"
⍤:≍, 1 HasFourOfAKindWithJokers "333JJ"
⍤:≍, 1 HasFourOfAKindWithJokers "33JJJ"
⍤:≍, 1 HasFourOfAKindWithJokers "3JJJJ"
⍤:≍, 1 HasFourOfAKindWithJokers "JJJJJ"
⍤:≍, 1 HasFourOfAKindWithJokers "34JJJ"
⍤:≍, 0 HasFourOfAKindWithJokers "345JJ"
---

HasThreeOfAKindWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasThreeOfAKind ▽≠@J.)
  | And⊃(=1NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=2NumJokers|1;)
  | And⊃(=3NumJokers|1;)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasThreeOfAKindWithJokers "33333"
⍤:≍, 1 HasThreeOfAKindWithJokers "3333J"
⍤:≍, 1 HasThreeOfAKindWithJokers "333JJ"
⍤:≍, 1 HasThreeOfAKindWithJokers "33JJJ"
⍤:≍, 1 HasThreeOfAKindWithJokers "3JJJJ"
⍤:≍, 1 HasThreeOfAKindWithJokers "JJJJJ"
⍤:≍, 1 HasThreeOfAKindWithJokers "34JJJ"
⍤:≍, 1 HasThreeOfAKindWithJokers "345JJ"
⍤:≍, 0 HasThreeOfAKindWithJokers "3456J"
---

HasOnePairWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=1NumJokers|1;)
  | And⊃(=2NumJokers|1;)
  | And⊃(=3NumJokers|1;)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasOnePairWithJokers "33333"
⍤:≍, 1 HasOnePairWithJokers "3333J"
⍤:≍, 1 HasOnePairWithJokers "333JJ"
⍤:≍, 1 HasOnePairWithJokers "33JJJ"
⍤:≍, 1 HasOnePairWithJokers "3JJJJ"
⍤:≍, 1 HasOnePairWithJokers "JJJJJ"
⍤:≍, 1 HasOnePairWithJokers "34JJJ"
⍤:≍, 1 HasOnePairWithJokers "345JJ"
⍤:≍, 1 HasOnePairWithJokers "3456J"
⍤:≍, 1 HasOnePairWithJokers "34534"
⍤:≍, 1 HasOnePairWithJokers "34536"
⍤:≍, 0 HasOnePairWithJokers "34567"
---

HasFullHouseWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasFullHouse ▽≠@J.)
  | And⊃(=1NumJokers|HasTwoPairs ▽≠@J.)
  | And⊃(=1NumJokers|HasThreeOfAKind ▽≠@J.)
  | And⊃(=2NumJokers|HasThreeOfAKind ▽≠@J.)
  | And⊃(=3NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasFullHouseWithJokers "55555"
⍤:≍, 0 HasFullHouseWithJokers "45555"
⍤:≍, 1 HasFullHouseWithJokers "44555"
⍤:≍, 0 HasFullHouseWithJokers "44556"
⍤:≍, 1 HasFullHouseWithJokers "J5555"
⍤:≍, 1 HasFullHouseWithJokers "J5555"
⍤:≍, 1 HasFullHouseWithJokers "J4555"
⍤:≍, 0 HasFullHouseWithJokers "J4556"
---

HasTwoPairsWithJokers ← /Or[
  ⊃(
    And⊃(=0NumJokers|HasTwoPairs ▽≠@J.)
  | And⊃(=1NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=2NumJokers|HasOnePair ▽≠@J.)
  | And⊃(=3NumJokers|1;)
  | And⊃(=4NumJokers|1;)
  | And⊃(=5NumJokers|1;)
  )]
---
⍤:≍, 1 HasTwoPairsWithJokers "33444"
⍤:≍, 1 HasTwoPairsWithJokers "33445"
⍤:≍, 0 HasTwoPairsWithJokers "33456"
⍤:≍, 0 HasTwoPairsWithJokers "34567"
⍤:≍, 1 HasTwoPairsWithJokers "J3444"
⍤:≍, 1 HasTwoPairsWithJokers "J3445"
⍤:≍, 0 HasTwoPairsWithJokers "J3456"
⍤:≍, 0 HasTwoPairsWithJokers "J4567"
---

HasHighCardWithJokers ← 1;

GetCardValue ← +1⊗:"J23456789TxQKA"
---
⍤:≍, 1 GetCardValue @J
⍤:≍, 5 GetCardValue @5
⍤:≍, 13 GetCardValue @K
---

GetStrengthForHand ← ¯⊗1 ⊐[
  ⊃(
    HasFiveOfAKindWithJokers
  | HasFourOfAKindWithJokers
  | HasFullHouseWithJokers
  | HasThreeOfAKindWithJokers
  | HasTwoPairsWithJokers
  | HasOnePairWithJokers
  )]

GetScoreForHand ← ⊐[
  ⊃(
    GetStrengthForHand
  | GetCardValue ⊡0
  | GetCardValue ⊡1
  | GetCardValue ⊡2
  | GetCardValue ⊡3
  | GetCardValue ⊡4
  )]

PrepareFile ← ⍉ ⊐≡(SplitByDelimiter@ ) SplitByDelimiter@\n
GetHands ← Fst
GetBids ← ⊐≡parse Snd

SolvePartTwo ← (
  ⊃(≡GetScoreForHand GetHands|GetHands|GetBids) PrepareFile
  ⊐≡(⊂⊂)     # join scores, hands, bids
  ⊐⊏⍏.       # sort by score
  ⊐≡⊂: +1⇡⧻. # join with ranks
  ≡(∵⊔⊏2_3)  # keep bids and ranks
  /+≡/×      # calc total winnings
)
---
⍤:≍, 5905 SolvePartTwo &fras "example_1"
⍤:≍, 251135960 SolvePartTwo &fras "input"
---

&p $"Part 2: _" SolvePartTwo &fras "input"
