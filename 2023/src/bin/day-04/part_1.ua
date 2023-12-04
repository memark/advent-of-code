Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"
  SplitByDelimiter ← Lib "SplitByDelimiter"
  StartsWith ← Lib "StartsWith"

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

# example 5
# input 10

# SplitLines &fras "example_1"

GetCardNumFromLine ← parse⊔⊡1⊐SplitByDelimiter @ ⊡0⊐SplitByDelimiter @: ⊡0⊐SplitByDelimiter @|
# GetCardNumFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetWinNumsFromLine ← ⊐≡parse ⊐SplitByDelimiter @ ⊡1⊐SplitByDelimiter @: ⊡0⊐SplitByDelimiter @|
# GetWinNumsFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetOwnNumsFromLine ← ⊐≡parse ⊐SplitByDelimiter @ ⊡1⊐SplitByDelimiter @|
# GetOwnNumsFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetScoreFromLine ← ⌊ⁿ:2-1 /+ ∊ ⊃(GetOwnNumsFromLine|GetWinNumsFromLine)
# GetScoreFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
# GetScoreFromLine "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
# GetScoreFromLine "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
# GetScoreFromLine "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
# GetScoreFromLine "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
# GetScoreFromLine "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"

/+ ⊐≡GetScoreFromLine SplitLines &fras "example_1"
/+ ⊐≡GetScoreFromLine SplitLines &fras "input"

# 21568, correct
